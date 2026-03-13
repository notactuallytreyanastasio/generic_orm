use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use orm::src::{
    changeset, delete_sql, from, safe_identifier, ChangesetTrait, FieldDef, FieldType,
    IntField, SafeIdentifier, SqlBuilder, SqlFragment, StringField, TableDef,
};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use temper_core::ToList;
use tower_http::services::ServeDir;

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    list_table: TableDef,
    todo_table: TableDef,
    // Pre-built SafeIdentifiers for field names
    sid_name: SafeIdentifier,
    sid_title: SafeIdentifier,
    sid_completed: SafeIdentifier,
    sid_list_id: SafeIdentifier,
    sid_id: SafeIdentifier,
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TodoList {
    id: i64,
    name: String,
    #[allow(dead_code)]
    created_at: Option<String>,
}

#[derive(Debug)]
struct TodoItem {
    id: i64,
    title: String,
    completed: bool,
    #[allow(dead_code)]
    list_id: i64,
    #[allow(dead_code)]
    created_at: Option<String>,
}

/// Extended list info with todo count for index page
struct ListWithCount {
    id: i64,
    name: String,
    todo_count: i64,
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    lists: Vec<ListWithCount>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    list: TodoList,
    todos: Vec<TodoItem>,
    completed_count: usize,
}

// ---------------------------------------------------------------------------
// Form types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CreateListForm {
    name: String,
}

#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
}

#[derive(Deserialize)]
struct EditTodoForm {
    title: String,
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

struct AppError(Box<dyn std::error::Error + Send + Sync>);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// ---------------------------------------------------------------------------
// ORM schema definitions (using generic ORM's TableDef/FieldDef)
// ---------------------------------------------------------------------------

fn sid(name: &str) -> SafeIdentifier {
    safe_identifier(name).expect(&format!("invalid identifier: {}", name))
}

fn build_list_table() -> TableDef {
    TableDef::new(
        sid("lists"),
        [
            FieldDef::new(sid("name"), FieldType::new(StringField::new()), false),
            FieldDef::new(sid("created_at"), FieldType::new(StringField::new()), true),
        ],
    )
}

fn build_todo_table() -> TableDef {
    TableDef::new(
        sid("todos"),
        [
            FieldDef::new(sid("title"), FieldType::new(StringField::new()), false),
            FieldDef::new(sid("completed"), FieldType::new(IntField::new()), false),
            FieldDef::new(sid("list_id"), FieldType::new(IntField::new()), false),
            FieldDef::new(sid("created_at"), FieldType::new(StringField::new()), true),
        ],
    )
}

// ---------------------------------------------------------------------------
// Database setup
// ---------------------------------------------------------------------------

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Enable WAL mode and foreign keys
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Create tables
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        );",
    )?;

    // Seed data if empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM lists", [], |row| row.get(0))?;

    if count == 0 {
        conn.execute("INSERT INTO lists (name) VALUES (?1)", ["Personal"])?;
        conn.execute("INSERT INTO lists (name) VALUES (?1)", ["Work"])?;

        for title in &[
            "Buy groceries",
            "Clean the house",
            "Call mom",
            "Read a book",
        ] {
            conn.execute(
                "INSERT INTO todos (title, list_id) VALUES (?1, 1)",
                [title],
            )?;
        }
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Buy groceries'",
            [],
        )?;

        for title in &[
            "Finish quarterly report",
            "Review pull requests",
            "Update documentation",
            "Team standup notes",
            "Deploy to staging",
        ] {
            conn.execute(
                "INSERT INTO todos (title, list_id) VALUES (?1, 2)",
                [title],
            )?;
        }
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Review pull requests'",
            [],
        )?;
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Team standup notes'",
            [],
        )?;

        println!("Seeded database with sample data.");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers: build SQL using the generic ORM
// ---------------------------------------------------------------------------

/// Build a WHERE condition fragment: `column = value` (for integers)
fn where_eq_int(column: &SafeIdentifier, value: i64) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(column.sql_value());
    b.append_safe(" = ");
    b.append_int64(value);
    b.accumulated()
}

/// Build a SELECT query for todos filtered by list_id using ORM's `from()`.
fn todos_for_list_sql(state: &AppState, list_id: i64) -> String {
    let condition = where_eq_int(&state.sid_list_id, list_id);
    let frag = from(state.todo_table.table_name())
        .r#where(condition)
        .order_by(state.sid_completed.clone(), true)
        .to_sql();
    frag.to_string().to_string()
}

/// Build an INSERT statement for the lists table using ORM's changeset.
fn insert_list_sql(state: &AppState, name: &str) -> String {
    let values = temper_core::Map::new(&[
        (Arc::new("name".to_string()), Arc::new(name.to_string())),
    ]);
    let cs = changeset(state.list_table.clone(), values)
        .cast([state.sid_name.clone()].to_list())
        .validate_required([state.sid_name.clone()].to_list());
    assert!(cs.is_valid(), "Changeset validation failed for list insert");
    cs.to_insert_sql()
        .expect("to_insert_sql failed")
        .to_string()
        .to_string()
}

/// Build an INSERT statement for the todos table using ORM's changeset.
fn insert_todo_sql(state: &AppState, title: &str, list_id: i64) -> String {
    let values = temper_core::Map::new(&[
        (Arc::new("title".to_string()), Arc::new(title.to_string())),
        (
            Arc::new("list_id".to_string()),
            Arc::new(list_id.to_string()),
        ),
    ]);
    let cs = changeset(state.todo_table.clone(), values)
        .cast([state.sid_title.clone(), state.sid_list_id.clone()].to_list())
        .validate_required([state.sid_title.clone(), state.sid_list_id.clone()].to_list());
    assert!(cs.is_valid(), "Changeset validation failed for todo insert");
    cs.to_insert_sql()
        .expect("to_insert_sql failed")
        .to_string()
        .to_string()
}

/// Build a SELECT for a single list by id using ORM's `from()`.
fn list_by_id_sql(state: &AppState, id: i64) -> String {
    let condition = where_eq_int(&state.sid_id, id);
    let frag = from(state.list_table.table_name())
        .r#where(condition)
        .limit(1)
        .expect("limit failed")
        .to_sql();
    frag.to_string().to_string()
}

/// Build a SELECT for a single todo by id using ORM's `from()`.
fn todo_by_id_sql(state: &AppState, id: i64) -> String {
    let condition = where_eq_int(&state.sid_id, id);
    let frag = from(state.todo_table.table_name())
        .r#where(condition)
        .limit(1)
        .expect("limit failed")
        .to_sql();
    frag.to_string().to_string()
}

/// Build a DELETE statement using ORM's `delete_sql`.
fn delete_todo_sql(state: &AppState, id: i32) -> String {
    let frag = delete_sql(state.todo_table.clone(), id);
    frag.to_string().to_string()
}

/// Build a DELETE statement for a list using ORM's `delete_sql`.
fn delete_list_sql(state: &AppState, id: i32) -> String {
    let frag = delete_sql(state.list_table.clone(), id);
    frag.to_string().to_string()
}

/// Build an UPDATE statement for toggling todo completion using ORM's changeset.
fn update_todo_completed_sql(state: &AppState, id: i32, completed: bool) -> String {
    let new_val = if completed { "1" } else { "0" };
    let values = temper_core::Map::new(&[(
        Arc::new("completed".to_string()),
        Arc::new(new_val.to_string()),
    )]);
    let cs = changeset(state.todo_table.clone(), values)
        .cast([state.sid_completed.clone()].to_list());
    cs.to_update_sql(id)
        .expect("to_update_sql failed")
        .to_string()
        .to_string()
}

/// Build an UPDATE statement for editing a todo title using ORM's changeset.
fn update_todo_title_sql(state: &AppState, id: i32, title: &str) -> String {
    let values = temper_core::Map::new(&[(
        Arc::new("title".to_string()),
        Arc::new(title.to_string()),
    )]);
    let cs = changeset(state.todo_table.clone(), values)
        .cast([state.sid_title.clone()].to_list())
        .validate_required([state.sid_title.clone()].to_list());
    assert!(cs.is_valid(), "Changeset validation failed for todo update");
    cs.to_update_sql(id)
        .expect("to_update_sql failed")
        .to_string()
        .to_string()
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET / - Show all lists
async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let lists = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            // Use raw SQL for the JOIN + aggregate (ORM doesn't support joins)
            let mut stmt = conn
                .prepare(
                    "SELECT l.id, l.name, COUNT(t.id) as todo_count
                     FROM lists l
                     LEFT JOIN todos t ON t.list_id = l.id
                     GROUP BY l.id
                     ORDER BY l.created_at DESC",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(ListWithCount {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        todo_count: row.get(2)?,
                    })
                })
                .unwrap();
            rows.collect::<Result<Vec<_>, _>>().unwrap()
        })
        .await?
    };

    let template = IndexTemplate { lists };
    Ok(Html(template.render()?))
}

/// POST /lists - Create a new list (uses ORM changeset + to_insert_sql)
async fn create_list(
    State(state): State<AppState>,
    Form(form): Form<CreateListForm>,
) -> Result<Redirect, AppError> {
    let name = form.name.trim().to_string();
    if !name.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let sql = insert_list_sql(&state, &name);
            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to("/"))
}

/// POST /lists/:id/delete - Delete a list (uses ORM delete_sql)
async fn delete_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let conn = state.db.lock().unwrap();
        // Delete child todos first (ORM delete_sql for each table)
        conn.execute(
            "DELETE FROM todos WHERE list_id = ?1",
            [id],
        )
        .unwrap();
        let sql = delete_list_sql(&state, id as i32);
        conn.execute(&sql, []).unwrap();
    })
    .await?;
    Ok(Redirect::to("/"))
}

/// GET /lists/:id - Show a list with its todos (uses ORM Query for SELECT)
async fn show_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let (list, todos) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list using ORM-generated SELECT
            let list_sql = list_by_id_sql(&state, id);
            let list = conn
                .query_row(&list_sql, [], |row| {
                    Ok(TodoList {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                })
                .unwrap();

            // Fetch todos using ORM-generated SELECT
            let todos_sql = todos_for_list_sql(&state, id);
            let mut stmt = conn.prepare(&todos_sql).unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: {
                            let v: i64 = row.get(2)?;
                            v != 0
                        },
                        list_id: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                })
                .unwrap();
            let todos: Vec<TodoItem> = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (list, todos)
        })
        .await?
    };

    let completed_count = todos.iter().filter(|t| t.completed).count();

    let template = ListTemplate {
        list,
        todos,
        completed_count,
    };
    Ok(Html(template.render()?))
}

/// POST /lists/:id/todos - Create a todo in a list (uses ORM changeset + to_insert_sql)
async fn create_todo(
    State(state): State<AppState>,
    Path(list_id): Path<i64>,
    Form(form): Form<CreateTodoForm>,
) -> Result<Redirect, AppError> {
    let title = form.title.trim().to_string();
    if !title.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let sql = insert_todo_sql(&state, &title, list_id);
            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/toggle - Toggle todo completed status (uses ORM changeset + to_update_sql)
async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch current todo using ORM-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let (completed, list_id): (bool, i64) = conn
                .query_row(&sql, [], |row| {
                    let c: i64 = row.get(2)?;
                    Ok((c != 0, row.get(3)?))
                })
                .unwrap();

            // UPDATE using ORM changeset + to_update_sql
            let update_sql = update_todo_completed_sql(&state, id as i32, !completed);
            conn.execute(&update_sql, []).unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/delete - Delete a todo (uses ORM delete_sql)
async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list_id using ORM-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let list_id: i64 = conn.query_row(&sql, [], |row| row.get(3)).unwrap();

            // DELETE using ORM delete_sql
            let del_sql = delete_todo_sql(&state, id as i32);
            conn.execute(&del_sql, []).unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/edit - Edit a todo title (uses ORM changeset + to_update_sql)
async fn edit_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<EditTodoForm>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        let title = form.title.trim().to_string();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list_id using ORM-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let list_id: i64 = conn.query_row(&sql, [], |row| row.get(3)).unwrap();

            // UPDATE using ORM changeset + to_update_sql
            if !title.is_empty() {
                let update_sql = update_todo_title_sql(&state, id as i32, &title);
                conn.execute(&update_sql, []).unwrap();
            }

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ORM runtime (required for Temper-compiled code)
    orm::init(None).unwrap();

    // Open rusqlite connection
    let conn = Connection::open("todo.db")?;
    init_db(&conn)?;

    // Build ORM table definitions
    let list_table = build_list_table();
    let todo_table = build_todo_table();

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        list_table,
        todo_table,
        sid_name: sid("name"),
        sid_title: sid("title"),
        sid_completed: sid("completed"),
        sid_list_id: sid("list_id"),
        sid_id: sid("id"),
    };

    // Build routes
    let app = Router::new()
        .route("/", get(index))
        .route("/lists", post(create_list))
        .route("/lists/{id}", get(show_list))
        .route("/lists/{id}/delete", post(delete_list))
        .route("/lists/{id}/todos", post(create_todo))
        .route("/todos/{id}/toggle", post(toggle_todo))
        .route("/todos/{id}/delete", post(delete_todo_handler))
        .route("/todos/{id}/edit", post(edit_todo))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "5003".to_string());
    println!("Todo App running at http://localhost:{port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
