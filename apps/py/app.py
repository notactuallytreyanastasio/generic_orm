import os
import sys
import sqlite3
from datetime import datetime, timezone

# ---------------------------------------------------------------------------
# Vendor path setup
# ---------------------------------------------------------------------------
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "temper-core"))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "std"))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "orm"))

from flask import (
    Flask,
    abort,
    redirect,
    render_template,
    request,
    send_file,
    url_for,
)

from orm.src import (
    safe_identifier, from_, changeset, delete_sql,
    TableDef, FieldDef, StringField, IntField,
    SqlBuilder,
)
from temper_core import Pair, map_constructor

# ---------------------------------------------------------------------------
# App configuration
# ---------------------------------------------------------------------------
DB_PATH = os.path.join(BASE_DIR, "todos.db")

app = Flask(__name__, static_folder="static")
app.config["SECRET_KEY"] = "retro-todo-secret-key"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _make_map(pairs_dict):
    return map_constructor(tuple(Pair(k, v) for k, v in pairs_dict.items()))


def _build_where(column, value):
    b = SqlBuilder()
    b.append_safe(column.sql_value)
    b.append_safe(" = ")
    if isinstance(value, int):
        b.append_int32(value)
    else:
        b.append_string(value)
    return b.accumulated


class _Obj:
    def __init__(self, **kw):
        self.__dict__.update(kw)


# ---------------------------------------------------------------------------
# ORM Schema Definitions
# ---------------------------------------------------------------------------
_name_f      = safe_identifier("name")
_title_f     = safe_identifier("title")
_completed_f = safe_identifier("completed")
_list_id_f   = safe_identifier("list_id")
_created_at_f = safe_identifier("created_at")
_id_f        = safe_identifier("id")

list_table = TableDef(
    safe_identifier("lists"),
    (
        FieldDef(_name_f, StringField(), False),
        FieldDef(_created_at_f, StringField(), True),
    ),
)

todo_table = TableDef(
    safe_identifier("todos"),
    (
        FieldDef(_title_f, StringField(), False),
        FieldDef(_completed_f, IntField(), False),
        FieldDef(_list_id_f, IntField(), False),
        FieldDef(_created_at_f, StringField(), True),
    ),
)

# ---------------------------------------------------------------------------
# Database helpers
# ---------------------------------------------------------------------------

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA foreign_keys=ON")
    return conn


def _now_iso():
    return datetime.now(timezone.utc).isoformat()


# ---------------------------------------------------------------------------
# Serve shared retro.css
# ---------------------------------------------------------------------------

@app.route("/retro.css")
def retro_css():
    css_path = os.path.join(BASE_DIR, "static", "retro.css")
    return send_file(css_path, mimetype="text/css")


# ---------------------------------------------------------------------------
# Routes -- Lists
# ---------------------------------------------------------------------------

@app.route("/")
def index():
    db = get_db()
    try:
        sql = (
            from_(safe_identifier("lists"))
            .order_by(safe_identifier("created_at"), True)
            .to_sql()
            .to_string()
        )
        lists = db.execute(sql).fetchall()

        list_data = []
        for row in lists:
            total = db.execute(
                "SELECT COUNT(*) FROM todos WHERE list_id = ?", (row["id"],)
            ).fetchone()[0]
            done = db.execute(
                "SELECT COUNT(*) FROM todos WHERE list_id = ? AND completed = 1",
                (row["id"],),
            ).fetchone()[0]
            lst_obj = _Obj(id=row["id"], name=row["name"], created_at=row["created_at"])
            list_data.append({"list": lst_obj, "total": total, "done": done})

        return render_template("index.html", list_data=list_data)
    finally:
        db.close()


@app.route("/lists", methods=["POST"])
def create_list():
    name = request.form.get("name", "").strip()
    if name:
        db = get_db()
        try:
            params = _make_map({"name": name, "created_at": _now_iso()})
            cs = changeset(list_table, params).cast((_name_f, _created_at_f)).validate_required((_name_f,))
            sql = cs.to_insert_sql().to_string()
            db.execute(sql)
            db.commit()
        except Exception:
            pass
        finally:
            db.close()
    return redirect(url_for("index"))


@app.route("/lists/<int:list_id>")
def show_list(list_id):
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        lst = _Obj(id=row["id"], name=row["name"], created_at=row["created_at"])

        sql = (
            from_(safe_identifier("todos"))
            .where(_build_where(_list_id_f, list_id))
            .order_by(_completed_f, True)
            .order_by(_created_at_f, True)
            .to_sql()
            .to_string()
        )
        todo_rows = db.execute(sql).fetchall()

        todos = [
            _Obj(
                id=r["id"],
                title=r["title"],
                completed=bool(r["completed"]),
                list_id=r["list_id"],
                created_at=r["created_at"],
            )
            for r in todo_rows
        ]
        total = len(todos)
        done = sum(1 for t in todos if t.completed)
        return render_template("list.html", list=lst, todos=todos, total=total, done=done)
    finally:
        db.close()


@app.route("/lists/<int:list_id>/edit", methods=["POST"])
def edit_list(list_id):
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        name = request.form.get("name", "").strip()
        if name:
            db.execute("UPDATE lists SET name = ? WHERE id = ?", (name, list_id))
            db.commit()
    finally:
        db.close()
    return redirect(url_for("index"))


@app.route("/lists/<int:list_id>/delete", methods=["POST"])
def delete_list(list_id):
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        db.execute("DELETE FROM todos WHERE list_id = ?", (list_id,))
        sql = delete_sql(list_table, list_id).to_string()
        db.execute(sql)
        db.commit()
    finally:
        db.close()
    return redirect(url_for("index"))


# ---------------------------------------------------------------------------
# Routes -- Todos
# ---------------------------------------------------------------------------

@app.route("/lists/<int:list_id>/todos", methods=["POST"])
def create_todo(list_id):
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        title = request.form.get("title", "").strip()
        if title:
            try:
                params = _make_map({
                    "title": title,
                    "completed": "0",
                    "list_id": str(list_id),
                    "created_at": _now_iso(),
                })
                cast_fields = (_title_f, _completed_f, _list_id_f, _created_at_f)
                cs = changeset(todo_table, params).cast(cast_fields).validate_required((_title_f, _completed_f, _list_id_f))
                sql = cs.to_insert_sql().to_string()
                db.execute(sql)
                db.commit()
            except Exception:
                pass
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/toggle", methods=["POST"])
def toggle_todo(todo_id):
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        new_val = 0 if todo["completed"] else 1
        db.execute("UPDATE todos SET completed = ? WHERE id = ?", (new_val, todo_id))
        db.commit()
        list_id = todo["list_id"]
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/edit", methods=["POST"])
def edit_todo(todo_id):
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        title = request.form.get("title", "").strip()
        if title:
            try:
                params = _make_map({"title": title})
                cs = changeset(todo_table, params).cast((_title_f,)).validate_required((_title_f,))
                sql = cs.to_update_sql(todo_id).to_string()
                db.execute(sql)
                db.commit()
            except Exception:
                pass
        list_id = todo["list_id"]
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/delete", methods=["POST"])
def delete_todo(todo_id):
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        list_id = todo["list_id"]
        sql = delete_sql(todo_table, todo_id).to_string()
        db.execute(sql)
        db.commit()
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


# ---------------------------------------------------------------------------
# Database initialisation & seed data
# ---------------------------------------------------------------------------

DDL_LISTS = """\
CREATE TABLE IF NOT EXISTS lists (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    created_at TEXT
)"""

DDL_TODOS = """\
CREATE TABLE IF NOT EXISTS todos (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT    NOT NULL,
    completed  INTEGER NOT NULL DEFAULT 0,
    list_id    INTEGER NOT NULL REFERENCES lists(id),
    created_at TEXT
)"""


def init_db():
    db = get_db()
    try:
        db.execute(DDL_LISTS)
        db.execute(DDL_TODOS)
        db.commit()
    finally:
        db.close()


def seed_database():
    db = get_db()
    try:
        row = db.execute("SELECT id FROM lists LIMIT 1").fetchone()
        if row is not None:
            return

        now = _now_iso()

        params = _make_map({"name": "Personal", "created_at": now})
        cs = changeset(list_table, params).cast((_name_f, _created_at_f)).validate_required((_name_f,))
        db.execute(cs.to_insert_sql().to_string())

        params = _make_map({"name": "Work", "created_at": now})
        cs = changeset(list_table, params).cast((_name_f, _created_at_f)).validate_required((_name_f,))
        db.execute(cs.to_insert_sql().to_string())

        db.commit()

        personal_id = db.execute("SELECT id FROM lists WHERE name = 'Personal'").fetchone()["id"]
        work_id = db.execute("SELECT id FROM lists WHERE name = 'Work'").fetchone()["id"]

        sample_todos = [
            ("Buy groceries", 0, personal_id),
            ("Call the dentist", 1, personal_id),
            ("Read a book", 0, personal_id),
            ("Go for a walk", 0, personal_id),
            ("Finish quarterly report", 0, work_id),
            ("Reply to client emails", 1, work_id),
            ("Update project roadmap", 0, work_id),
        ]
        for title, completed, lid in sample_todos:
            params = _make_map({
                "title": title,
                "completed": str(completed),
                "list_id": str(lid),
                "created_at": now,
            })
            cast_fields = (_title_f, _completed_f, _list_id_f, _created_at_f)
            cs = changeset(todo_table, params).cast(cast_fields).validate_required((_title_f, _completed_f, _list_id_f))
            db.execute(cs.to_insert_sql().to_string())

        db.commit()
    finally:
        db.close()


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    init_db()
    seed_database()
    port = int(os.environ.get("PORT", 5001))
    app.run(debug=True, port=port)
