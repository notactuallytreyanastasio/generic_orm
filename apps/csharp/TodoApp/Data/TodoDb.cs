using Microsoft.Data.Sqlite;
using Orm.Src;
using TodoApp.Models;
using TemperLang.Core;

namespace TodoApp.Data;

/// <summary>
/// Data access layer using the generic ORM for query generation and Microsoft.Data.Sqlite for execution.
/// The ORM generates safe SQL via its SqlFragment type (contextual autoescaping).
/// Raw SQL is used for DDL where the ORM does not provide helpers.
/// </summary>
public class TodoDb
{
    private readonly string _connectionString;

    // Safe identifiers for table and field names (validated at construction time)
    private static readonly ISafeIdentifier ListsTableName = SrcGlobal.SafeIdentifier("lists");
    private static readonly ISafeIdentifier TodosTableName = SrcGlobal.SafeIdentifier("todos");

    private static readonly ISafeIdentifier NameField = SrcGlobal.SafeIdentifier("name");
    private static readonly ISafeIdentifier CreatedAtField = SrcGlobal.SafeIdentifier("created_at");
    private static readonly ISafeIdentifier TitleField = SrcGlobal.SafeIdentifier("title");
    private static readonly ISafeIdentifier CompletedField = SrcGlobal.SafeIdentifier("completed");
    private static readonly ISafeIdentifier ListIdField = SrcGlobal.SafeIdentifier("list_id");
    private static readonly ISafeIdentifier IdField = SrcGlobal.SafeIdentifier("id");

    // ORM TableDefs with typed fields
    public static readonly TableDef ListTableDef = new TableDef(
        ListsTableName,
        Listed.CreateReadOnlyList<FieldDef>(
            new FieldDef(NameField, new StringField(), false),
            new FieldDef(CreatedAtField, new StringField(), false)
        )
    );

    public static readonly TableDef TodoTableDef = new TableDef(
        TodosTableName,
        Listed.CreateReadOnlyList<FieldDef>(
            new FieldDef(TitleField, new StringField(), false),
            new FieldDef(CompletedField, new IntField(), false),
            new FieldDef(ListIdField, new IntField(), false),
            new FieldDef(CreatedAtField, new StringField(), false)
        )
    );

    // Field lists for Cast/ValidateRequired on lists
    private static readonly IReadOnlyList<ISafeIdentifier> ListInsertFields =
        Listed.CreateReadOnlyList<ISafeIdentifier>(NameField, CreatedAtField);

    // Field lists for Cast/ValidateRequired on todos
    private static readonly IReadOnlyList<ISafeIdentifier> TodoInsertFields =
        Listed.CreateReadOnlyList<ISafeIdentifier>(TitleField, CompletedField, ListIdField, CreatedAtField);

    public TodoDb(string connectionString)
    {
        _connectionString = connectionString;
    }

    private SqliteConnection Open()
    {
        var conn = new SqliteConnection(_connectionString);
        conn.Open();
        return conn;
    }

    /// <summary>
    /// Create tables if they don't exist. Raw DDL -- the ORM does not generate DDL.
    /// </summary>
    public void EnsureCreated()
    {
        using var conn = Open();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            CREATE TABLE IF NOT EXISTS lists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                list_id INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE
            );
            PRAGMA foreign_keys = ON;
        ";
        cmd.ExecuteNonQuery();
    }

    // ────────────────────────────────────────
    // Helper: build a WHERE condition as SqlFragment
    // ────────────────────────────────────────

    private static SqlFragment WhereEq(ISafeIdentifier column, int value)
    {
        var b = new SqlBuilder();
        b.AppendSafe(column.SqlValue);
        b.AppendSafe(" = ");
        b.AppendInt32(value);
        return b.Accumulated;
    }

    // ────────────────────────────────────────
    // Lists
    // ────────────────────────────────────────

    /// <summary>
    /// Get all lists ordered by created_at, with their todo items attached.
    /// Uses ORM Query via SrcGlobal.From() for the SELECT.
    /// </summary>
    public List<TodoList> GetAllLists()
    {
        using var conn = Open();

        // Build SELECT * FROM lists ORDER BY created_at ASC via the ORM
        var listQuery = SrcGlobal.From(ListsTableName)
            .OrderBy(CreatedAtField, true);
        string listSql = listQuery.ToSql().ToString();

        var lists = new List<TodoList>();
        using (var cmd = conn.CreateCommand())
        {
            cmd.CommandText = listSql;
            using var reader = cmd.ExecuteReader();
            while (reader.Read())
            {
                lists.Add(ReadList(reader));
            }
        }

        // Attach todos to each list
        foreach (var list in lists)
        {
            list.Todos = GetTodosForList(conn, list.Id);
        }

        return lists;
    }

    /// <summary>
    /// Get a single list by id, with todos. Uses ORM Where clause.
    /// </summary>
    public TodoList? GetList(int id)
    {
        using var conn = Open();

        var query = SrcGlobal.From(ListsTableName)
            .Where(WhereEq(IdField, id));
        string sql = query.ToSql().ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        if (!reader.Read())
            return null;

        var list = ReadList(reader);
        list.Todos = GetTodosForList(conn, list.Id);
        return list;
    }

    /// <summary>
    /// Insert a new list. Uses ORM Changeset for safe INSERT generation.
    /// </summary>
    public void InsertList(string name)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("name", name),
                new KeyValuePair<string, string>("created_at", DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
            )
        );
        string sql = SrcGlobal.Changeset(ListTableDef, values)
            .Cast(ListInsertFields)
            .ValidateRequired(ListInsertFields)
            .ToInsertSql()
            .ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Update a list name. Uses ORM Changeset for safe UPDATE generation.
    /// </summary>
    public void UpdateList(int id, string name)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("name", name)
            )
        );
        var nameOnly = Listed.CreateReadOnlyList<ISafeIdentifier>(NameField);
        string sql = SrcGlobal.Changeset(ListTableDef, values)
            .Cast(nameOnly)
            .ValidateRequired(nameOnly)
            .ToUpdateSql(id)
            .ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Delete a list and its todos. Uses ORM DeleteSql for safe DELETE generation.
    /// </summary>
    public void DeleteList(int id)
    {
        using var conn = Open();

        // Enable foreign keys so CASCADE works
        using (var pragma = conn.CreateCommand())
        {
            pragma.CommandText = "PRAGMA foreign_keys = ON";
            pragma.ExecuteNonQuery();
        }

        string sql = SrcGlobal.DeleteSql(ListTableDef, id).ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    // ────────────────────────────────────────
    // Todos
    // ────────────────────────────────────────

    /// <summary>
    /// Get todos for a list. Uses ORM Query with Where and OrderBy.
    /// </summary>
    private List<TodoItem> GetTodosForList(SqliteConnection conn, int listId)
    {
        var query = SrcGlobal.From(TodosTableName)
            .Where(WhereEq(ListIdField, listId))
            .OrderBy(CompletedField, true)
            .OrderBy(CreatedAtField, true);
        string sql = query.ToSql().ToString();

        var items = new List<TodoItem>();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            items.Add(ReadTodo(reader));
        }
        return items;
    }

    /// <summary>
    /// Insert a new todo. Uses ORM Changeset for safe INSERT generation.
    /// </summary>
    public void InsertTodo(string title, int listId)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("title", title),
                new KeyValuePair<string, string>("completed", "0"),
                new KeyValuePair<string, string>("list_id", listId.ToString()),
                new KeyValuePair<string, string>("created_at", DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
            )
        );
        string sql = SrcGlobal.Changeset(TodoTableDef, values)
            .Cast(TodoInsertFields)
            .ValidateRequired(TodoInsertFields)
            .ToInsertSql()
            .ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Toggle a todo's completed status. Uses SqlBuilder for safe SQL construction.
    /// </summary>
    public void ToggleTodo(int todoId)
    {
        using var conn = Open();
        var sb = new SqlBuilder();
        sb.AppendSafe("UPDATE todos SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ");
        sb.AppendInt32(todoId);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Update a todo title. Uses ORM Changeset for safe UPDATE generation.
    /// </summary>
    public void UpdateTodo(int todoId, string title)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("title", title)
            )
        );
        var titleOnly = Listed.CreateReadOnlyList<ISafeIdentifier>(TitleField);
        string sql = SrcGlobal.Changeset(TodoTableDef, values)
            .Cast(titleOnly)
            .ValidateRequired(titleOnly)
            .ToUpdateSql(todoId)
            .ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Delete a todo. Uses ORM DeleteSql for safe DELETE generation.
    /// </summary>
    public void DeleteTodo(int todoId)
    {
        using var conn = Open();
        string sql = SrcGlobal.DeleteSql(TodoTableDef, todoId).ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Check if any lists exist (for seeding).
    /// </summary>
    public bool HasAnyLists()
    {
        using var conn = Open();
        var query = SrcGlobal.From(ListsTableName).Limit(1);
        string sql = query.ToSql().ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        return reader.Read();
    }

    // ────────────────────────────────────────
    // Row mappers
    // ────────────────────────────────────────

    private static TodoList ReadList(SqliteDataReader reader)
    {
        return new TodoList
        {
            Id = reader.GetInt32(reader.GetOrdinal("id")),
            Name = reader.GetString(reader.GetOrdinal("name")),
            CreatedAt = reader.GetString(reader.GetOrdinal("created_at"))
        };
    }

    private static TodoItem ReadTodo(SqliteDataReader reader)
    {
        return new TodoItem
        {
            Id = reader.GetInt32(reader.GetOrdinal("id")),
            Title = reader.GetString(reader.GetOrdinal("title")),
            Completed = reader.GetInt32(reader.GetOrdinal("completed")) != 0,
            ListId = reader.GetInt32(reader.GetOrdinal("list_id")),
            CreatedAt = reader.GetString(reader.GetOrdinal("created_at"))
        };
    }
}
