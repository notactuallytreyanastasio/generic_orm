# Query Tests

    let sid(name: String): SafeIdentifier {
      safeIdentifier(name) orelse panic()
    }

    test("bare from produces SELECT *") {
      let q = from(sid("users"));
      assert(q.toSql().toString() == "SELECT * FROM users") { "bare query" };
    }

    test("select restricts columns") {
      let q = from(sid("users")).select([sid("id"), sid("name")]);
      assert(q.toSql().toString() == "SELECT id, name FROM users") { "select columns" };
    }

    test("where adds condition with int value") {
      let q = from(sid("users")).where(sql"age > ${18}");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE age > 18") { "where int" };
    }

    test("where adds condition with bool value") {
      let q = from(sid("users")).where(sql"active = ${true}");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE active = TRUE") { "where bool" };
    }

    test("chained where uses AND") {
      let q = from(sid("users"))
        .where(sql"age > ${18}")
        .where(sql"active = ${true}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE age > 18 AND active = TRUE"
      ) { "chained where" };
    }

    test("orderBy ASC") {
      let q = from(sid("users")).orderBy(sid("name"), true);
      assert(q.toSql().toString() == "SELECT * FROM users ORDER BY name ASC") { "order asc" };
    }

    test("orderBy DESC") {
      let q = from(sid("users")).orderBy(sid("created_at"), false);
      assert(
        q.toSql().toString() == "SELECT * FROM users ORDER BY created_at DESC"
      ) { "order desc" };
    }

    test("limit and offset") {
      let q = do { from(sid("users")).limit(10).offset(20) } orelse panic();
      assert(q.toSql().toString() == "SELECT * FROM users LIMIT 10 OFFSET 20") { "limit/offset" };
    }

    test("limit bubbles on negative") {
      let didBubble = do { from(sid("users")).limit(-1); false } orelse true;
      assert(didBubble) { "negative limit should bubble" };
    }

    test("offset bubbles on negative") {
      let didBubble = do { from(sid("users")).offset(-1); false } orelse true;
      assert(didBubble) { "negative offset should bubble" };
    }

    test("complex composed query") {
      let minAge = 21;
      let q = do {
        from(sid("users"))
          .select([sid("id"), sid("name"), sid("email")])
          .where(sql"age >= ${minAge}")
          .where(sql"active = ${true}")
          .orderBy(sid("name"), true)
          .limit(25)
          .offset(0)
      } orelse panic();
      assert(
        q.toSql().toString() ==
          "SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0"
      ) { "complex query" };
    }

    test("safeToSql applies default limit when none set") {
      let q = from(sid("users"));
      let s = (q.safeToSql(100) orelse panic()).toString();
      assert(s == "SELECT * FROM users LIMIT 100") { "should have limit: ${s}" };
    }

    test("safeToSql respects explicit limit") {
      let q = do { from(sid("users")).limit(5) } orelse panic();
      let s = (q.safeToSql(100) orelse panic()).toString();
      assert(s == "SELECT * FROM users LIMIT 5") { "explicit limit preserved: ${s}" };
    }

    test("safeToSql bubbles on negative defaultLimit") {
      let didBubble = do { from(sid("users")).safeToSql(-1); false } orelse true;
      assert(didBubble) { "negative defaultLimit should bubble" };
    }

    test("where with injection attempt in string value is escaped") {
      let evil = "'; DROP TABLE users; --";
      let q = from(sid("users")).where(sql"name = ${evil}");
      let s = q.toSql().toString();
      // The leading ' is doubled to '' — the DROP TABLE becomes a literal string value
      assert(s.indexOf("''") is StringIndex) { "quotes must be doubled: ${s}" };
      assert(s.indexOf("SELECT * FROM users WHERE name =") is StringIndex) { "structure intact: ${s}" };
    }

    test("safeIdentifier rejects user-supplied table name with metacharacters") {
      // A user can't supply a table name that passes safeIdentifier()
      let attack = "users; DROP TABLE users; --";
      let didBubble = do { safeIdentifier(attack); false } orelse true;
      assert(didBubble) { "metacharacter-containing name must be rejected at construction" };
    }

    test("innerJoin produces INNER JOIN") {
      let q = from(sid("users"))
        .innerJoin(sid("orders"), sql"users.id = orders.user_id");
      assert(
        q.toSql().toString() == "SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id"
      ) { "inner join" };
    }

    test("leftJoin produces LEFT JOIN") {
      let q = from(sid("users"))
        .leftJoin(sid("profiles"), sql"users.id = profiles.user_id");
      assert(
        q.toSql().toString() == "SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id"
      ) { "left join" };
    }

    test("rightJoin produces RIGHT JOIN") {
      let q = from(sid("orders"))
        .rightJoin(sid("users"), sql"orders.user_id = users.id");
      assert(
        q.toSql().toString() == "SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id"
      ) { "right join" };
    }

    test("fullJoin produces FULL OUTER JOIN") {
      let q = from(sid("users"))
        .fullJoin(sid("orders"), sql"users.id = orders.user_id");
      assert(
        q.toSql().toString() == "SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id"
      ) { "full join" };
    }

    test("chained joins") {
      let q = from(sid("users"))
        .innerJoin(sid("orders"), sql"users.id = orders.user_id")
        .leftJoin(sid("profiles"), sql"users.id = profiles.user_id");
      assert(
        q.toSql().toString() == "SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id"
      ) { "chained joins" };
    }

    test("join with where and orderBy") {
      let q = do {
        from(sid("users"))
          .innerJoin(sid("orders"), sql"users.id = orders.user_id")
          .where(sql"orders.total > ${100}")
          .orderBy(sid("name"), true)
          .limit(10)
      } orelse panic();
      assert(
        q.toSql().toString() == "SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10"
      ) { "join with where/order/limit" };
    }

    test("col helper produces qualified reference") {
      let c = col(sid("users"), sid("id"));
      assert(c.toString() == "users.id") { "col helper" };
    }

    test("join with col helper") {
      let onCond = col(sid("users"), sid("id"));
      let b = new SqlBuilder();
      b.appendFragment(onCond);
      b.appendSafe(" = ");
      b.appendFragment(col(sid("orders"), sid("user_id")));
      let q = from(sid("users"))
        .innerJoin(sid("orders"), b.accumulated);
      assert(
        q.toSql().toString() == "SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id"
      ) { "join with col" };
    }

    // --- Phase 1: WHERE Clause Enrichment Tests ---

    test("orWhere basic") {
      let q = from(sid("users")).orWhere(sql"status = ${"active"}");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE status = 'active'") { "orWhere basic" };
    }

    test("where then orWhere") {
      let q = from(sid("users"))
        .where(sql"age > ${18}")
        .orWhere(sql"vip = ${true}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE age > 18 OR vip = TRUE"
      ) { "where then orWhere" };
    }

    test("multiple orWhere") {
      let q = from(sid("users"))
        .where(sql"active = ${true}")
        .orWhere(sql"role = ${"admin"}")
        .orWhere(sql"role = ${"moderator"}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'"
      ) { "multiple orWhere" };
    }

    test("mixed where and orWhere") {
      let q = from(sid("users"))
        .where(sql"age > ${18}")
        .where(sql"active = ${true}")
        .orWhere(sql"vip = ${true}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE"
      ) { "mixed where and orWhere" };
    }

    test("whereNull") {
      let q = from(sid("users")).whereNull(sid("deleted_at"));
      assert(q.toSql().toString() == "SELECT * FROM users WHERE deleted_at IS NULL") { "whereNull" };
    }

    test("whereNotNull") {
      let q = from(sid("users")).whereNotNull(sid("email"));
      assert(q.toSql().toString() == "SELECT * FROM users WHERE email IS NOT NULL") { "whereNotNull" };
    }

    test("whereNull chained with where") {
      let q = from(sid("users"))
        .where(sql"active = ${true}")
        .whereNull(sid("deleted_at"));
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL"
      ) { "whereNull chained" };
    }

    test("whereNotNull chained with orWhere") {
      let q = from(sid("users"))
        .whereNull(sid("deleted_at"))
        .orWhere(sql"role = ${"admin"}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'"
      ) { "whereNotNull with orWhere" };
    }

    test("whereIn with int values") {
      let q = from(sid("users")).whereIn(sid("id"), [new SqlInt32(1), new SqlInt32(2), new SqlInt32(3)]);
      assert(q.toSql().toString() == "SELECT * FROM users WHERE id IN (1, 2, 3)") { "whereIn ints" };
    }

    test("whereIn with string values escaping") {
      let q = from(sid("users")).whereIn(sid("name"), [new SqlString("Alice"), new SqlString("Bob's")]);
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')") { "whereIn strings" };
    }

    test("whereIn with empty list produces 1=0") {
      let q = from(sid("users")).whereIn(sid("id"), []);
      assert(q.toSql().toString() == "SELECT * FROM users WHERE 1 = 0") { "whereIn empty" };
    }

    test("whereIn chained") {
      let q = from(sid("users"))
        .where(sql"active = ${true}")
        .whereIn(sid("role"), [new SqlString("admin"), new SqlString("user")]);
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')"
      ) { "whereIn chained" };
    }

    test("whereIn single element") {
      let q = from(sid("users")).whereIn(sid("id"), [new SqlInt32(42)]);
      assert(q.toSql().toString() == "SELECT * FROM users WHERE id IN (42)") { "whereIn single" };
    }

    test("whereNot basic") {
      let q = from(sid("users")).whereNot(sql"active = ${true}");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE NOT (active = TRUE)") { "whereNot" };
    }

    test("whereNot chained") {
      let q = from(sid("users"))
        .where(sql"age > ${18}")
        .whereNot(sql"banned = ${true}");
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)"
      ) { "whereNot chained" };
    }

    test("whereBetween integers") {
      let q = from(sid("users")).whereBetween(sid("age"), new SqlInt32(18), new SqlInt32(65));
      assert(q.toSql().toString() == "SELECT * FROM users WHERE age BETWEEN 18 AND 65") { "whereBetween ints" };
    }

    test("whereBetween chained") {
      let q = from(sid("users"))
        .where(sql"active = ${true}")
        .whereBetween(sid("age"), new SqlInt32(21), new SqlInt32(30));
      assert(
        q.toSql().toString() == "SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30"
      ) { "whereBetween chained" };
    }

    test("whereLike basic") {
      let q = from(sid("users")).whereLike(sid("name"), "John%");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name LIKE 'John%'") { "whereLike" };
    }

    test("whereILike basic") {
      let q = from(sid("users")).whereILike(sid("email"), "%@gmail.com");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE email ILIKE '%@gmail.com'") { "whereILike" };
    }

    test("whereLike with injection attempt") {
      let q = from(sid("users")).whereLike(sid("name"), "'; DROP TABLE users; --");
      let s = q.toSql().toString();
      assert(s.indexOf("''") is StringIndex) { "like injection escaped: ${s}" };
      assert(s.indexOf("LIKE") is StringIndex) { "like structure intact: ${s}" };
    }

    test("whereLike wildcard patterns") {
      let q = from(sid("users")).whereLike(sid("name"), "%son%");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name LIKE '%son%'") { "whereLike wildcard" };
    }
