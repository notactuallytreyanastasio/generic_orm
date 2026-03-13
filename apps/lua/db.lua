-- db.lua - SQLite database layer using generic ORM for query building
--
-- Uses the vendored generic ORM library (compiled from Temper) for SELECT,
-- INSERT, UPDATE, and DELETE query generation via the security-focused API:
--   - from() for SELECT queries
--   - changeset() for INSERT/UPDATE
--   - deleteSql() for DELETE
--   - SqlBuilder for WHERE clause construction

local sqlite3 = require("lsqlite3complete")

-- vendor path bootstrap
local script_dir = debug.getinfo(1, "S").source:match("^@(.*/)") or "./"
package.path = script_dir .. "vendor/?.lua;"
    .. script_dir .. "vendor/?/init.lua;"
    .. script_dir .. "vendor/orm/?.lua;"
    .. script_dir .. "vendor/orm/?/init.lua;"
    .. script_dir .. "vendor/temper-core/?.lua;"
    .. script_dir .. "vendor/temper-core/?/init.lua;"
    .. script_dir .. "vendor/std/?.lua;"
    .. script_dir .. "vendor/std/?/init.lua;"
    .. script_dir .. "?.lua;"
    .. package.path

local temper = require("temper-core")
local orm    = require("orm/src")

-- ORM type shortcuts
local safeIdentifier = orm.safeIdentifier
local TableDef       = orm.TableDef
local FieldDef       = orm.FieldDef
local StringField    = orm.StringField
local IntField       = orm.IntField
local SqlBuilder     = orm.SqlBuilder
local changeset      = orm.changeset
local deleteSql      = orm.deleteSql
local from           = orm.from

-- Schema definitions using TableDef + FieldDef
-- FieldDef(name: SafeIdentifier, fieldType: FieldType, nullable: boolean)

local name_field       = FieldDef(safeIdentifier("name"),       StringField(), false)
local created_at_field = FieldDef(safeIdentifier("created_at"), StringField(), true)

local lists_table = TableDef(
    safeIdentifier("lists"),
    temper.listof(name_field, created_at_field)
)

local title_field     = FieldDef(safeIdentifier("title"),     StringField(), false)
local completed_field = FieldDef(safeIdentifier("completed"), IntField(),    true)
local list_id_field   = FieldDef(safeIdentifier("list_id"),   IntField(),    false)
local todo_created_at = FieldDef(safeIdentifier("created_at"), StringField(), true)

local todos_table = TableDef(
    safeIdentifier("todos"),
    temper.listof(title_field, completed_field, list_id_field, todo_created_at)
)

-- helpers

-- Build a temper Map from a plain Lua table of key/value pairs.
-- All values are stringified because the changeset treats field values as strings.
local function make_params(tbl)
    local pairs_list = {}
    for k, v in pairs(tbl) do
        pairs_list[#pairs_list + 1] = temper.pair_constructor(k, tostring(v))
    end
    return temper.map_constructor(pairs_list)
end

-- Build a SELECT query via from() and return the SQL string.
-- `opts` is an optional table: { where = {SqlFragment,...}, order = {{fieldDef,asc},...}, limit = n }
local function build_select(tableDef, opts)
    opts = opts or {}
    local q = from(tableDef.tableName)

    if opts.where then
        for _, fragment in ipairs(opts.where) do
            q = q:where(fragment)
        end
    end

    if opts.order then
        for _, o in ipairs(opts.order) do
            q = q:orderBy(o[1], o[2])
        end
    end

    if opts.limit then
        q = q:limit(opts.limit)
    end

    return q:toSql():toString()
end

-- Build a WHERE clause: column = int_value, returning a SqlFragment
local function where_eq_int(col_name, value)
    local b = SqlBuilder()
    b:appendSafe(col_name)
    b:appendSafe(" = ")
    b:appendInt32(value)
    return b.accumulated
end

-- Extract SafeIdentifier names from a list of FieldDef objects
local function field_names(field_defs)
    local names = {}
    for _, fd in ipairs(field_defs) do
        names[#names + 1] = fd.name
    end
    return temper.listof(table.unpack(names))
end

-- Build an INSERT via changeset and return the SQL string.
-- `fields` is a temper list of FieldDef objects to cast/validate against.
local function build_insert(tableDef, values_table, cast_fields)
    local params = make_params(values_table)
    local safe_ids = field_names(cast_fields)
    local cs = changeset(tableDef, params)
    cs = cs:cast(safe_ids)
    cs = cs:validateRequired(safe_ids)
    return cs:toInsertSql():toString()
end

-- module state
local db   = {}
local conn = nil

-- connection management

function db.open(path)
    path = path or (script_dir .. "todo.db")
    conn = sqlite3.open(path)
    conn:exec("PRAGMA journal_mode=WAL;")
    conn:exec("PRAGMA foreign_keys=ON;")
    return conn
end

function db.close()
    if conn then
        conn:close()
        conn = nil
    end
end

function db.get_conn()
    return conn
end

-- DDL (raw SQL - table creation is beyond ORM scope)

function db.migrate()
    conn:exec([[
        CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        );
    ]])
    conn:exec([[
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            list_id INTEGER NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        );
    ]])
end

-- seed data

function db.seed()
    -- Check existing count via ORM SELECT
    local count_sql = build_select(lists_table)
    local count = 0
    for _ in conn:nrows(count_sql) do count = count + 1 end
    if count > 0 then return false end

    conn:exec("BEGIN;")

    -- Insert lists using ORM changeset
    local sql = build_insert(lists_table, { name = "Work Tasks" }, temper.listof(name_field))
    conn:exec(sql)
    local work_id = conn:last_insert_rowid()

    sql = build_insert(lists_table, { name = "Shopping List" }, temper.listof(name_field))
    conn:exec(sql)
    local shop_id = conn:last_insert_rowid()

    -- Insert todos using ORM changeset
    local todo_cast_fields = temper.listof(title_field, completed_field, list_id_field)

    local work_todos = {
        { "Review pull requests", 1 },
        { "Write unit tests",    0 },
        { "Update documentation", 0 },
        { "Fix login bug",       1 },
        { "Deploy to staging",   0 },
    }
    for _, t in ipairs(work_todos) do
        sql = build_insert(todos_table, {
            title     = t[1],
            completed = t[2],
            list_id   = work_id,
        }, todo_cast_fields)
        conn:exec(sql)
    end

    local shop_todos = {
        { "Milk",         1 },
        { "Bread",        0 },
        { "Eggs",         0 },
        { "Coffee beans", 1 },
        { "Bananas",      0 },
    }
    for _, t in ipairs(shop_todos) do
        sql = build_insert(todos_table, {
            title     = t[1],
            completed = t[2],
            list_id   = shop_id,
        }, todo_cast_fields)
        conn:exec(sql)
    end

    conn:exec("COMMIT;")
    return true
end

-- ================================================================
-- Lists CRUD
-- ================================================================

function db.lists_create(name)
    local sql = build_insert(lists_table, { name = name }, temper.listof(name_field))
    conn:exec(sql)
    return conn:last_insert_rowid()
end

function db.lists_get_all()
    -- This query uses a LEFT JOIN + GROUP BY which is beyond ORM's
    -- current scope, so we keep it as raw SQL.
    local results = {}
    local stmt = conn:prepare([[
        SELECT l.id, l.name, l.created_at,
               COUNT(t.id) as total,
               SUM(CASE WHEN t.completed = 1 THEN 1 ELSE 0 END) as done
        FROM lists l
        LEFT JOIN todos t ON t.list_id = l.id
        GROUP BY l.id
        ORDER BY l.created_at DESC
    ]])
    while stmt:step() == sqlite3.ROW do
        table.insert(results, {
            id         = stmt:get_value(0),
            name       = stmt:get_value(1),
            created_at = stmt:get_value(2),
            total      = stmt:get_value(3),
            done       = stmt:get_value(4) or 0,
        })
    end
    stmt:finalize()
    return results
end

function db.lists_get_by_id(id)
    -- ORM SELECT with WHERE id = ?
    local sql = build_select(lists_table, {
        where = { where_eq_int("id", id) },
        limit = 1,
    })
    local result = nil
    for row in conn:nrows(sql) do
        result = {
            id         = row.id,
            name       = row.name,
            created_at = row.created_at,
        }
        break
    end
    return result
end

function db.lists_delete(id)
    -- ORM deleteSql
    local sql = deleteSql(lists_table, id):toString()
    conn:exec(sql)
    return conn:changes() > 0
end

-- ================================================================
-- Todos CRUD
-- ================================================================

function db.todos_create(title, list_id)
    local sql = build_insert(todos_table, {
        title   = title,
        list_id = list_id,
    }, temper.listof(title_field, list_id_field))
    conn:exec(sql)
    return conn:last_insert_rowid()
end

function db.todos_get_by_list(list_id)
    -- ORM SELECT with WHERE + ORDER BY
    local sql = build_select(todos_table, {
        where = { where_eq_int("list_id", list_id) },
        order = { { completed_field.name, true }, { todo_created_at.name, false } },
    })
    local results = {}
    for row in conn:nrows(sql) do
        table.insert(results, {
            id         = row.id,
            title      = row.title,
            completed  = row.completed,
            list_id    = row.list_id,
            created_at = row.created_at,
        })
    end
    return results
end

function db.todos_get_by_id(id)
    local sql = build_select(todos_table, {
        where = { where_eq_int("id", id) },
        limit = 1,
    })
    local result = nil
    for row in conn:nrows(sql) do
        result = {
            id         = row.id,
            title      = row.title,
            completed  = row.completed,
            list_id    = row.list_id,
            created_at = row.created_at,
        }
        break
    end
    return result
end

function db.todos_toggle(id)
    -- Toggle uses raw SQL since it requires 1-completed expression
    local stmt = conn:prepare("UPDATE todos SET completed = 1 - completed WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_delete(id)
    -- ORM deleteSql
    local sql = deleteSql(todos_table, id):toString()
    conn:exec(sql)
    return conn:changes() > 0
end

function db.todos_get_list_id(id)
    local sql = build_select(todos_table, {
        where = { where_eq_int("id", id) },
        limit = 1,
    })
    local list_id = nil
    for row in conn:nrows(sql) do
        list_id = row.list_id
        break
    end
    return list_id
end

return db
