import express from 'express';
import path from 'path';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';
import {
  from, safeIdentifier, changeset, deleteSql,
  TableDef, FieldDef, StringField, IntField,
  SqlBuilder,
} from 'orm/src';
import { mapConstructor, pairConstructor } from '@temperlang/core';

const require = createRequire(import.meta.url);
const Database = require('better-sqlite3');

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = process.env.PORT || 5006;

// ---------------------------------------------------------------------------
// Helpers — build a Temper Map from a JS object
// ---------------------------------------------------------------------------
function makeMap(obj) {
  const pairs = Object.entries(obj).map(([k, v]) => pairConstructor(k, v));
  return mapConstructor(Object.freeze(pairs));
}

// ---------------------------------------------------------------------------
// ORM Schema Definitions
// ---------------------------------------------------------------------------
const listTableName = safeIdentifier('lists');
const todoTableName = safeIdentifier('todos');

const nameField    = safeIdentifier('name');
const titleField   = safeIdentifier('title');
const completedF   = safeIdentifier('completed');
const listIdField  = safeIdentifier('list_id');
const createdAtF   = safeIdentifier('created_at');

const listTable = new TableDef(listTableName, Object.freeze([
  new FieldDef(nameField,   new StringField(), false),
  new FieldDef(createdAtF,  new StringField(), true),
]));

const todoTable = new TableDef(todoTableName, Object.freeze([
  new FieldDef(titleField,  new StringField(), false),
  new FieldDef(completedF,  new IntField(),    false),
  new FieldDef(listIdField, new IntField(),    false),
  new FieldDef(createdAtF,  new StringField(), true),
]));

// ---------------------------------------------------------------------------
// SQL helpers using the ORM
// ---------------------------------------------------------------------------
function buildWhere(column, value) {
  const b = new SqlBuilder();
  b.appendSafe(column.sqlValue);
  b.appendSafe(' = ');
  if (typeof value === 'number') {
    b.appendInt32(value);
  } else {
    b.appendString(value);
  }
  return b.accumulated;
}

// ---------------------------------------------------------------------------
// Middleware
// ---------------------------------------------------------------------------
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));
app.use(express.urlencoded({ extended: true }));
app.use(express.static(path.join(__dirname, 'public')));

// ---------------------------------------------------------------------------
// Database Setup
// ---------------------------------------------------------------------------
const db = new Database(path.join(__dirname, 'todo.db'));
db.pragma('journal_mode = WAL');
db.pragma('foreign_keys = ON');

db.exec(`
  CREATE TABLE IF NOT EXISTS lists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
  )
`);

db.exec(`
  CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    completed INTEGER DEFAULT 0,
    list_id INTEGER REFERENCES lists(id),
    created_at TEXT DEFAULT (datetime('now'))
  )
`);

// ---------------------------------------------------------------------------
// Seed Data
// ---------------------------------------------------------------------------
const listCount = db.prepare('SELECT COUNT(*) AS cnt FROM lists').get().cnt;
if (listCount === 0) {
  const insertList = db.prepare('INSERT INTO lists (name) VALUES (?)');
  const insertTodo = db.prepare('INSERT INTO todos (title, completed, list_id) VALUES (?, ?, ?)');

  const seedDb = db.transaction(() => {
    const personal = insertList.run('Personal');
    const work = insertList.run('Work');

    insertTodo.run('Buy groceries', 0, personal.lastInsertRowid);
    insertTodo.run('Clean the apartment', 0, personal.lastInsertRowid);
    insertTodo.run('Read a book', 1, personal.lastInsertRowid);
    insertTodo.run('Go for a run', 0, personal.lastInsertRowid);

    insertTodo.run('Finish quarterly report', 0, work.lastInsertRowid);
    insertTodo.run('Email the team', 1, work.lastInsertRowid);
    insertTodo.run('Prepare slide deck', 0, work.lastInsertRowid);
  });

  seedDb();
}

// ---------------------------------------------------------------------------
// Prepared Statements (raw SQL for aggregate/JOIN queries)
// ---------------------------------------------------------------------------
const stmts = {
  allLists: db.prepare(`
    SELECT l.*,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id) AS todo_count,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id AND completed = 1) AS done_count
    FROM lists l ORDER BY l.created_at DESC
  `),
  toggleTodo: db.prepare('UPDATE todos SET completed = CASE WHEN completed = 1 THEN 0 ELSE 1 END WHERE id = ?'),
  countTodosByList: db.prepare('SELECT COUNT(*) AS total, SUM(completed) AS done FROM todos WHERE list_id = ?'),
};

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

// GET / — Show all lists
app.get('/', (req, res) => {
  const lists = stmts.allLists.all();
  res.render('index', { lists });
});

// POST /lists — Create a new list (uses ORM changeset)
app.post('/lists', (req, res) => {
  const name = (req.body.name || '').trim();
  if (name) {
    const params = makeMap({ name });
    const cs = changeset(listTable, params)
      .cast(Object.freeze([nameField]))
      .validateRequired(Object.freeze([nameField]));
    try {
      const sql = cs.toInsertSql().toString();
      db.prepare(sql).run();
    } catch (e) {
      // changeset validation failed — ignore
    }
  }
  res.redirect('/');
});

// POST /lists/:id/delete — Delete a list (uses ORM deleteSql)
app.post('/lists/:id/delete', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const deleteListAndTodos = db.transaction((listId) => {
    // Delete todos by list_id — use ORM query to find, raw SQL to delete
    db.prepare('DELETE FROM todos WHERE list_id = ?').run(listId);
    const sql = deleteSql(listTable, listId).toString();
    db.prepare(sql).run();
  });
  deleteListAndTodos(id);
  res.redirect('/');
});

// GET /lists/:id — Show a single list with its todos (uses ORM Query)
app.get('/lists/:id', (req, res) => {
  const id = parseInt(req.params.id, 10);

  // Get list by ID using ORM
  const listSql = from(listTableName)
    .where(buildWhere(safeIdentifier('id'), id))
    .toSql().toString();
  const list = db.prepare(listSql).get();
  if (!list) return res.redirect('/');

  // Get todos for this list using ORM
  const todosSql = from(todoTableName)
    .where(buildWhere(listIdField, id))
    .orderBy(completedF, true)
    .orderBy(createdAtF, false)
    .toSql().toString();
  const todos = db.prepare(todosSql).all();

  const counts = stmts.countTodosByList.get(id);
  res.render('list', { list, todos, counts });
});

// POST /lists/:id/todos — Add a todo to a list (uses ORM changeset)
app.post('/lists/:id/todos', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const title = (req.body.title || '').trim();
  if (title) {
    const params = makeMap({ title, completed: '0', list_id: String(id) });
    const castFields = Object.freeze([titleField, completedF, listIdField]);
    const cs = changeset(todoTable, params)
      .cast(castFields)
      .validateRequired(castFields);
    try {
      const sql = cs.toInsertSql().toString();
      db.prepare(sql).run();
    } catch (e) {
      // changeset validation failed
    }
  }
  res.redirect(`/lists/${id}`);
});

// POST /todos/:id/toggle — Toggle a todo's completed status
app.post('/todos/:id/toggle', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const todoSql = from(todoTableName)
    .where(buildWhere(safeIdentifier('id'), id))
    .toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo) {
    stmts.toggleTodo.run(id);
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// POST /todos/:id/delete — Delete a todo (uses ORM deleteSql)
app.post('/todos/:id/delete', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const todoSql = from(todoTableName)
    .where(buildWhere(safeIdentifier('id'), id))
    .toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo) {
    const sql = deleteSql(todoTable, id).toString();
    db.prepare(sql).run();
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// POST /todos/:id/edit — Edit a todo's title (uses ORM changeset toUpdateSql)
app.post('/todos/:id/edit', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const title = (req.body.title || '').trim();
  const todoSql = from(todoTableName)
    .where(buildWhere(safeIdentifier('id'), id))
    .toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo && title) {
    const params = makeMap({ title });
    const cs = changeset(todoTable, params)
      .cast(Object.freeze([titleField]))
      .validateRequired(Object.freeze([titleField]));
    try {
      const sql = cs.toUpdateSql(id).toString();
      db.prepare(sql).run();
    } catch (e) {
      // changeset validation failed
    }
    res.redirect(`/lists/${todo.list_id}`);
  } else if (todo) {
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// ---------------------------------------------------------------------------
// Start Server
// ---------------------------------------------------------------------------
app.listen(PORT, () => {
  console.log(`Todo app running at http://localhost:${PORT}`);
});
