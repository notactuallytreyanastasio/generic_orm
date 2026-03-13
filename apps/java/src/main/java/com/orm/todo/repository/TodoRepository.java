package com.orm.todo.repository;

import com.orm.todo.model.TodoItem;
import com.orm.todo.model.TodoList;
import orm.src.Changeset;
import orm.src.FieldDef;
import orm.src.IntField;
import orm.src.Query;
import orm.src.SafeIdentifier;
import orm.src.SqlBuilder;
import orm.src.SqlFragment;
import orm.src.SrcGlobal;
import orm.src.StringField;
import orm.src.TableDef;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;
import temper.core.Core;

import jakarta.annotation.PostConstruct;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.AbstractMap.SimpleImmutableEntry;
import java.util.List;
import java.util.Map;

/**
 * JDBC repository backed by the generic ORM for query building.
 *
 * The ORM handles: schema definition via TableDef/FieldDef, SELECT generation via Query,
 * INSERT/UPDATE generation via Changeset, and DELETE generation via deleteSql.
 * Raw JDBC handles: DDL and executing the generated SQL.
 */
@Repository
public class TodoRepository {

    private final JdbcTemplate jdbc;

    // ORM table definitions
    private final TableDef listTableDef;
    private final TableDef itemTableDef;

    // SafeIdentifiers for table names
    private final SafeIdentifier listsTable;
    private final SafeIdentifier todosTable;

    // SafeIdentifiers for field names (reused across queries)
    private final SafeIdentifier idField;
    private final SafeIdentifier nameField;
    private final SafeIdentifier createdAtField;
    private final SafeIdentifier titleField;
    private final SafeIdentifier completedField;
    private final SafeIdentifier listIdField;

    private static final DateTimeFormatter DT_FMT = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    public TodoRepository(JdbcTemplate jdbc) {
        this.jdbc = jdbc;

        // Create SafeIdentifiers
        this.listsTable = SrcGlobal.safeIdentifier("lists");
        this.todosTable = SrcGlobal.safeIdentifier("todos");
        this.idField = SrcGlobal.safeIdentifier("id");
        this.nameField = SrcGlobal.safeIdentifier("name");
        this.createdAtField = SrcGlobal.safeIdentifier("created_at");
        this.titleField = SrcGlobal.safeIdentifier("title");
        this.completedField = SrcGlobal.safeIdentifier("completed");
        this.listIdField = SrcGlobal.safeIdentifier("list_id");

        // Define table schemas using ORM TableDef/FieldDef
        this.listTableDef = new TableDef(listsTable, List.of(
            new FieldDef(nameField, new StringField(), false),
            new FieldDef(createdAtField, new StringField(), false)
        ));

        this.itemTableDef = new TableDef(todosTable, List.of(
            new FieldDef(titleField, new StringField(), false),
            new FieldDef(completedField, new IntField(), false),
            new FieldDef(createdAtField, new StringField(), false),
            new FieldDef(listIdField, new IntField(), false)
        ));
    }

    @PostConstruct
    public void initTables() {
        jdbc.execute(
            "CREATE TABLE IF NOT EXISTS lists (" +
            "  id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "  name TEXT NOT NULL," +
            "  created_at TEXT NOT NULL" +
            ")"
        );
        jdbc.execute(
            "CREATE TABLE IF NOT EXISTS todos (" +
            "  id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "  title TEXT NOT NULL," +
            "  completed INTEGER NOT NULL DEFAULT 0," +
            "  created_at TEXT NOT NULL," +
            "  list_id INTEGER NOT NULL REFERENCES lists(id) ON DELETE CASCADE" +
            ")"
        );
        // Enable foreign key enforcement for SQLite
        jdbc.execute("PRAGMA foreign_keys = ON");
    }

    // ---------------------------------------------------------------
    // Row mappers
    // ---------------------------------------------------------------

    private final RowMapper<TodoList> listMapper = (rs, rowNum) -> {
        TodoList list = new TodoList();
        list.setId(rs.getLong("id"));
        list.setName(rs.getString("name"));
        list.setCreatedAt(LocalDateTime.parse(rs.getString("created_at"), DT_FMT));
        return list;
    };

    private final RowMapper<TodoItem> itemMapper = (rs, rowNum) -> {
        TodoItem item = new TodoItem();
        item.setId(rs.getLong("id"));
        item.setTitle(rs.getString("title"));
        item.setCompleted(rs.getInt("completed") != 0);
        item.setCreatedAt(LocalDateTime.parse(rs.getString("created_at"), DT_FMT));
        item.setListId(rs.getLong("list_id"));
        return item;
    };

    // ---------------------------------------------------------------
    // List operations
    // ---------------------------------------------------------------

    /**
     * Fetch all lists, with completed/total counts populated.
     * Uses ORM Query.from() to build the SELECT query.
     */
    public List<TodoList> findAllLists() {
        String sql = SrcGlobal.from(listsTable)
                .orderBy(idField, true)
                .toSql()
                .toString();

        List<TodoList> lists = jdbc.query(sql, listMapper);

        // Populate counts for each list
        for (TodoList list : lists) {
            populateListCounts(list);
        }
        return lists;
    }

    /**
     * Find a single list by id.
     * Uses ORM Query with SqlBuilder-based WHERE clause.
     */
    public TodoList findListById(Long id) {
        SqlFragment whereCondition = buildEqCondition(idField, id.intValue());

        String sql = SrcGlobal.from(listsTable)
                .where(whereCondition)
                .limit(1)
                .toSql()
                .toString();

        List<TodoList> results = jdbc.query(sql, listMapper);
        if (results.isEmpty()) {
            return null;
        }
        TodoList list = results.get(0);
        populateListCounts(list);
        return list;
    }

    /**
     * Insert a new list.
     * Uses ORM Changeset to build the INSERT statement.
     */
    public void insertList(String name) {
        Map<String, String> params = Core.mapConstructor(List.of(
            new SimpleImmutableEntry<>("name", name),
            new SimpleImmutableEntry<>("created_at", LocalDateTime.now().format(DT_FMT))
        ));

        List<SafeIdentifier> fields = List.of(nameField, createdAtField);
        SqlFragment insertSql = SrcGlobal.changeset(listTableDef, params)
                .cast(fields)
                .validateRequired(fields)
                .toInsertSql();
        jdbc.execute(insertSql.toString());
    }

    /**
     * Delete a list and its todos.
     * Uses ORM deleteSql for type-safe DELETE generation.
     */
    public void deleteList(Long id) {
        // Delete child todos first
        jdbc.update("DELETE FROM todos WHERE list_id = ?", id);
        // Delete the list using ORM deleteSql
        SqlFragment deleteSql = SrcGlobal.deleteSql(listTableDef, id.intValue());
        jdbc.execute(deleteSql.toString());
    }

    /**
     * Count all lists (used by DataLoader to check if seeding is needed).
     */
    public long countLists() {
        Long count = jdbc.queryForObject("SELECT COUNT(*) FROM lists", Long.class);
        return count != null ? count : 0;
    }

    // ---------------------------------------------------------------
    // Todo item operations
    // ---------------------------------------------------------------

    /**
     * Find all todos for a given list, ordered by created_at ascending.
     * Uses ORM Query with WHERE and ORDER BY.
     */
    public List<TodoItem> findItemsByListId(Long listId) {
        SqlFragment whereCondition = buildEqCondition(listIdField, listId.intValue());

        String sql = SrcGlobal.from(todosTable)
                .where(whereCondition)
                .orderBy(createdAtField, true)
                .toSql()
                .toString();

        return jdbc.query(sql, itemMapper);
    }

    /**
     * Find a single todo item by id.
     * Uses ORM Query with WHERE clause.
     */
    public TodoItem findItemById(Long id) {
        SqlFragment whereCondition = buildEqCondition(idField, id.intValue());

        String sql = SrcGlobal.from(todosTable)
                .where(whereCondition)
                .limit(1)
                .toSql()
                .toString();

        List<TodoItem> results = jdbc.query(sql, itemMapper);
        return results.isEmpty() ? null : results.get(0);
    }

    /**
     * Insert a new todo item.
     * Uses ORM Changeset to build the INSERT statement.
     */
    public void insertItem(String title, Long listId) {
        Map<String, String> params = Core.mapConstructor(List.of(
            new SimpleImmutableEntry<>("title", title),
            new SimpleImmutableEntry<>("completed", "0"),
            new SimpleImmutableEntry<>("created_at", LocalDateTime.now().format(DT_FMT)),
            new SimpleImmutableEntry<>("list_id", String.valueOf(listId))
        ));

        List<SafeIdentifier> fields = List.of(titleField, completedField, createdAtField, listIdField);
        SqlFragment insertSql = SrcGlobal.changeset(itemTableDef, params)
                .cast(fields)
                .validateRequired(fields)
                .toInsertSql();
        jdbc.execute(insertSql.toString());
    }

    /**
     * Insert a todo item with a specific completed state (used by DataLoader).
     * Uses ORM Changeset to build the INSERT statement.
     */
    public void insertItem(String title, Long listId, boolean completed) {
        Map<String, String> params = Core.mapConstructor(List.of(
            new SimpleImmutableEntry<>("title", title),
            new SimpleImmutableEntry<>("completed", completed ? "1" : "0"),
            new SimpleImmutableEntry<>("created_at", LocalDateTime.now().format(DT_FMT)),
            new SimpleImmutableEntry<>("list_id", String.valueOf(listId))
        ));

        List<SafeIdentifier> fields = List.of(titleField, completedField, createdAtField, listIdField);
        SqlFragment insertSql = SrcGlobal.changeset(itemTableDef, params)
                .cast(fields)
                .validateRequired(fields)
                .toInsertSql();
        jdbc.execute(insertSql.toString());
    }

    /**
     * Toggle the completed state of a todo item.
     * Uses raw JDBC for the conditional UPDATE.
     */
    public void toggleItem(Long id) {
        jdbc.update("UPDATE todos SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ?", id);
    }

    /**
     * Update the title of a todo item.
     * Uses ORM Changeset to build the UPDATE statement.
     */
    public void updateItemTitle(Long id, String title) {
        Map<String, String> params = Core.mapConstructor(List.of(
            new SimpleImmutableEntry<>("title", title)
        ));

        List<SafeIdentifier> fields = List.of(titleField);
        SqlFragment updateSql = SrcGlobal.changeset(itemTableDef, params)
                .cast(fields)
                .validateRequired(fields)
                .toUpdateSql(id.intValue());
        jdbc.execute(updateSql.toString());
    }

    /**
     * Delete a todo item.
     * Uses ORM deleteSql for type-safe DELETE generation.
     */
    public void deleteItem(Long id) {
        SqlFragment deleteSql = SrcGlobal.deleteSql(itemTableDef, id.intValue());
        jdbc.execute(deleteSql.toString());
    }

    // ---------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------

    /**
     * Build a WHERE condition: field = intValue
     * Uses SqlBuilder for safe SQL fragment construction.
     */
    private SqlFragment buildEqCondition(SafeIdentifier field, int value) {
        SqlBuilder b = new SqlBuilder();
        b.appendSafe(field.getSqlValue());
        b.appendSafe(" = ");
        b.appendInt32(value);
        return b.getAccumulated();
    }

    private void populateListCounts(TodoList list) {
        Long total = jdbc.queryForObject(
            "SELECT COUNT(*) FROM todos WHERE list_id = ?", Long.class, list.getId());
        Long completed = jdbc.queryForObject(
            "SELECT COUNT(*) FROM todos WHERE list_id = ? AND completed = 1", Long.class, list.getId());
        list.setTotalCount(total != null ? total : 0);
        list.setCompletedCount(completed != null ? completed : 0);
    }
}
