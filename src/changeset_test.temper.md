# Changeset Tests

## Helpers

    let csid(name: String): SafeIdentifier {
      safeIdentifier(name) orelse panic()
    }

    let userTable(): TableDef {
      new TableDef(csid("users"), [
        new FieldDef(csid("name"),   new StringField(), false),
        new FieldDef(csid("email"),  new StringField(), false),
        new FieldDef(csid("age"),    new IntField(),    true),
        new FieldDef(csid("score"),  new FloatField(),  true),
        new FieldDef(csid("active"), new BoolField(),   true),
      ])
    }

## cast

    test("cast whitelists allowed fields") {
      let params = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("email", "alice@example.com"),
        new Pair("admin", "true"),
      ]);
      let cs = changeset(userTable(), params).cast([csid("name"), csid("email")]);
      assert(cs.changes.has("name"))   { "name should be in changes" };
      assert(cs.changes.has("email"))  { "email should be in changes" };
      assert(!cs.changes.has("admin")) { "admin must be dropped (not in whitelist)" };
      assert(cs.isValid)               { "should still be valid" };
    }

    test("cast is replacing not additive — second call resets whitelist") {
      let params = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("email", "alice@example.com"),
      ]);
      // First cast admits name; second cast admits only email — name must be gone
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .cast([csid("email")]);
      assert(!cs.changes.has("name"))  { "name must be excluded by second cast" };
      assert(cs.changes.has("email"))  { "email should be present" };
    }

    test("cast ignores empty string values") {
      let params = new Map<String, String>([
        new Pair("name", ""),
        new Pair("email", "bob@example.com"),
      ]);
      let cs = changeset(userTable(), params).cast([csid("name"), csid("email")]);
      assert(!cs.changes.has("name")) { "empty name should not be in changes" };
      assert(cs.changes.has("email")) { "email should be in changes" };
    }

## validateRequired

    test("validateRequired passes when field present") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateRequired([csid("name")]);
      assert(cs.isValid)            { "should be valid" };
      assert(cs.errors.length == 0) { "no errors expected" };
    }

    test("validateRequired fails when field missing") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateRequired([csid("name")]);
      assert(!cs.isValid)               { "should be invalid" };
      assert(cs.errors.length == 1)     { "should have one error" };
      assert(cs.errors[0].field == "name") { "error should name the field" };
    }

## validateLength

    test("validateLength passes within range") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateLength(csid("name"), 2, 50);
      assert(cs.isValid) { "should be valid" };
    }

    test("validateLength fails when too short") {
      let params = new Map<String, String>([new Pair("name", "A")]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateLength(csid("name"), 2, 50);
      assert(!cs.isValid) { "should be invalid" };
    }

    test("validateLength fails when too long") {
      let params = new Map<String, String>([new Pair("name", "ABCDEFGHIJKLMNOPQRSTUVWXYZ")]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateLength(csid("name"), 2, 10);
      assert(!cs.isValid) { "should be invalid" };
    }

## validateInt / validateFloat

    test("validateInt passes for valid integer") {
      let params = new Map<String, String>([new Pair("age", "30")]);
      let cs = changeset(userTable(), params).cast([csid("age")]).validateInt(csid("age"));
      assert(cs.isValid) { "should be valid" };
    }

    test("validateInt fails for non-integer") {
      let params = new Map<String, String>([new Pair("age", "not-a-number")]);
      let cs = changeset(userTable(), params).cast([csid("age")]).validateInt(csid("age"));
      assert(!cs.isValid) { "should be invalid" };
    }

    test("validateFloat passes for valid float") {
      let params = new Map<String, String>([new Pair("score", "9.5")]);
      let cs = changeset(userTable(), params).cast([csid("score")]).validateFloat(csid("score"));
      assert(cs.isValid) { "should be valid" };
    }

## validateInt64

    test("validateInt64 passes for valid 64-bit integer") {
      let params = new Map<String, String>([new Pair("age", "9999999999")]);
      let cs = changeset(userTable(), params).cast([csid("age")]).validateInt64(csid("age"));
      assert(cs.isValid) { "should be valid" };
    }

    test("validateInt64 fails for non-integer") {
      let params = new Map<String, String>([new Pair("age", "not-a-number")]);
      let cs = changeset(userTable(), params).cast([csid("age")]).validateInt64(csid("age"));
      assert(!cs.isValid) { "should be invalid" };
    }

## validateBool

    test("validateBool accepts true/1/yes/on") {
      for (let v of ["true", "1", "yes", "on"]) {
        let params = new Map<String, String>([new Pair("active", v)]);
        let cs = changeset(userTable(), params).cast([csid("active")]).validateBool(csid("active"));
        assert(cs.isValid) { "should accept: ${v}" };
      }
    }

    test("validateBool accepts false/0/no/off") {
      for (let v of ["false", "0", "no", "off"]) {
        let params = new Map<String, String>([new Pair("active", v)]);
        let cs = changeset(userTable(), params).cast([csid("active")]).validateBool(csid("active"));
        assert(cs.isValid) { "should accept: ${v}" };
      }
    }

    test("validateBool rejects ambiguous values") {
      for (let v of ["TRUE", "Yes", "maybe", "2", "enabled"]) {
        let params = new Map<String, String>([new Pair("active", v)]);
        let cs = changeset(userTable(), params).cast([csid("active")]).validateBool(csid("active"));
        assert(!cs.isValid) { "should reject ambiguous: ${v}" };
      }
    }

## toInsertSql — SQL injection safety

    test("toInsertSql escapes Bobby Tables") {
      let params = new Map<String, String>([
        new Pair("name", "Robert'); DROP TABLE users;--"),
        new Pair("email", "bobby@evil.com"),
      ]);
      let cs = changeset(userTable(), params)
        .cast([csid("name"), csid("email")])
        .validateRequired([csid("name"), csid("email")]);
      let sqlFrag = cs.toInsertSql() orelse panic();
      let s = sqlFrag.toString();
      // The single-quote in "Robert'" is doubled to '' — the attack is inert
      assert(s.indexOf("''") is StringIndex) { "single quote must be doubled: ${s}" };
    }

    test("toInsertSql produces correct SQL for string field") {
      let params = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("email", "a@example.com"),
      ]);
      let cs = changeset(userTable(), params)
        .cast([csid("name"), csid("email")])
        .validateRequired([csid("name"), csid("email")]);
      let sqlFrag = cs.toInsertSql() orelse panic();
      let s = sqlFrag.toString();
      assert(s.indexOf("INSERT INTO users") is StringIndex) { "has INSERT INTO: ${s}" };
      assert(s.indexOf("'Alice'") is StringIndex)           { "has quoted name: ${s}" };
    }

    test("toInsertSql produces correct SQL for int field") {
      // age is nullable in userTable, so we can insert with just age
      let params = new Map<String, String>([
        new Pair("name", "Bob"),
        new Pair("email", "b@example.com"),
        new Pair("age", "25"),
      ]);
      let cs = changeset(userTable(), params)
        .cast([csid("name"), csid("email"), csid("age")])
        .validateRequired([csid("name"), csid("email")]);
      let sqlFrag = cs.toInsertSql() orelse panic();
      let s = sqlFrag.toString();
      assert(s.indexOf("25") is StringIndex) { "age rendered unquoted: ${s}" };
    }

    test("toInsertSql bubbles on invalid changeset") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateRequired([csid("name")]);
      let didBubble = do { cs.toInsertSql(); false } orelse true;
      assert(didBubble) { "invalid changeset should bubble" };
    }

    test("toInsertSql enforces non-nullable fields independently of isValid") {
      // Build a table where name is non-nullable
      let strictTable = new TableDef(csid("posts"), [
        new FieldDef(csid("title"), new StringField(), false),
        new FieldDef(csid("body"),  new StringField(), true),
      ]);
      // Provide body only — title is required but missing
      let params = new Map<String, String>([new Pair("body", "hello")]);
      let cs = changeset(strictTable, params).cast([csid("body")]);
      // cs.isValid is true (no validations failed), but title is missing
      assert(cs.isValid) { "changeset should appear valid (no explicit validation run)" };
      // toInsertSql must still bubble — independent nullable check catches it
      let didBubble = do { cs.toInsertSql(); false } orelse true;
      assert(didBubble) { "toInsertSql should enforce nullable regardless of isValid" };
    }

## toUpdateSql

    test("toUpdateSql produces correct SQL") {
      let params = new Map<String, String>([new Pair("name", "Bob")]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateRequired([csid("name")]);
      let sqlFrag = cs.toUpdateSql(42) orelse panic();
      let s = sqlFrag.toString();
      assert(s == "UPDATE users SET name = 'Bob' WHERE id = 42") { "got: ${s}" };
    }

    test("toUpdateSql bubbles on invalid changeset") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params).cast([csid("name")]).validateRequired([csid("name")]);
      let didBubble = do { cs.toUpdateSql(1); false } orelse true;
      assert(didBubble) { "invalid changeset should bubble" };
    }

## putChange

    test("putChange adds a new field") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .putChange(csid("email"), "alice@example.com");
      assert(cs.changes.has("email")) { "email should be in changes" };
      assert(cs.changes.getOr("email", "") == "alice@example.com") { "email value" };
    }

    test("putChange overwrites existing field") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .putChange(csid("name"), "Bob");
      assert(cs.changes.getOr("name", "") == "Bob") { "name should be overwritten" };
    }

    test("putChange value appears in toInsertSql") {
      let params = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("email", "a@example.com"),
      ]);
      let cs = changeset(userTable(), params)
        .cast([csid("name"), csid("email")])
        .putChange(csid("name"), "Bob");
      let s = (cs.toInsertSql() orelse panic()).toString();
      assert(s.indexOf("'Bob'") is StringIndex) { "should use putChange value: ${s}" };
    }

## getChange

    test("getChange returns value for existing field") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params).cast([csid("name")]);
      let val = cs.getChange(csid("name")) orelse panic();
      assert(val == "Alice") { "should return Alice" };
    }

    test("getChange bubbles on missing field") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params).cast([csid("name")]);
      let didBubble = do { cs.getChange(csid("email")); false } orelse true;
      assert(didBubble) { "should bubble for missing field" };
    }

## deleteChange

    test("deleteChange removes field") {
      let params = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("email", "a@example.com"),
      ]);
      let cs = changeset(userTable(), params)
        .cast([csid("name"), csid("email")])
        .deleteChange(csid("email"));
      assert(!cs.changes.has("email")) { "email should be removed" };
      assert(cs.changes.has("name")) { "name should remain" };
    }

    test("deleteChange on nonexistent field is no-op") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .deleteChange(csid("email"));
      assert(cs.changes.has("name")) { "name should still be present" };
      assert(cs.isValid) { "should still be valid" };
    }

## validateInclusion

    test("validateInclusion passes when value in list") {
      let params = new Map<String, String>([new Pair("name", "admin")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateInclusion(csid("name"), ["admin", "user", "guest"]);
      assert(cs.isValid) { "should be valid" };
    }

    test("validateInclusion fails when value not in list") {
      let params = new Map<String, String>([new Pair("name", "hacker")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateInclusion(csid("name"), ["admin", "user", "guest"]);
      assert(!cs.isValid) { "should be invalid" };
      assert(cs.errors[0].field == "name") { "error on name" };
    }

    test("validateInclusion skips when field not in changes") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateInclusion(csid("name"), ["admin", "user"]);
      assert(cs.isValid) { "should be valid when field absent" };
    }

## validateExclusion

    test("validateExclusion passes when value not in list") {
      let params = new Map<String, String>([new Pair("name", "Alice")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateExclusion(csid("name"), ["root", "admin", "superuser"]);
      assert(cs.isValid) { "should be valid" };
    }

    test("validateExclusion fails when value in list") {
      let params = new Map<String, String>([new Pair("name", "admin")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateExclusion(csid("name"), ["root", "admin", "superuser"]);
      assert(!cs.isValid) { "should be invalid" };
      assert(cs.errors[0].field == "name") { "error on name" };
    }

    test("validateExclusion skips when field not in changes") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateExclusion(csid("name"), ["root", "admin"]);
      assert(cs.isValid) { "should be valid when field absent" };
    }

## validateNumber

    test("validateNumber greaterThan passes") {
      let params = new Map<String, String>([new Pair("age", "25")]);
      let cs = changeset(userTable(), params)
        .cast([csid("age")])
        .validateNumber(csid("age"), new NumberValidationOpts(18.0, null, null, null, null));
      assert(cs.isValid) { "25 > 18 should pass" };
    }

    test("validateNumber greaterThan fails") {
      let params = new Map<String, String>([new Pair("age", "15")]);
      let cs = changeset(userTable(), params)
        .cast([csid("age")])
        .validateNumber(csid("age"), new NumberValidationOpts(18.0, null, null, null, null));
      assert(!cs.isValid) { "15 > 18 should fail" };
    }

    test("validateNumber lessThan passes") {
      let params = new Map<String, String>([new Pair("score", "8.5")]);
      let cs = changeset(userTable(), params)
        .cast([csid("score")])
        .validateNumber(csid("score"), new NumberValidationOpts(null, 10.0, null, null, null));
      assert(cs.isValid) { "8.5 < 10 should pass" };
    }

    test("validateNumber lessThan fails") {
      let params = new Map<String, String>([new Pair("score", "12.0")]);
      let cs = changeset(userTable(), params)
        .cast([csid("score")])
        .validateNumber(csid("score"), new NumberValidationOpts(null, 10.0, null, null, null));
      assert(!cs.isValid) { "12 < 10 should fail" };
    }

    test("validateNumber greaterThanOrEqual boundary") {
      let params = new Map<String, String>([new Pair("age", "18")]);
      let cs = changeset(userTable(), params)
        .cast([csid("age")])
        .validateNumber(csid("age"), new NumberValidationOpts(null, null, 18.0, null, null));
      assert(cs.isValid) { "18 >= 18 should pass" };
    }

    test("validateNumber combined options") {
      let params = new Map<String, String>([new Pair("score", "5.0")]);
      let cs = changeset(userTable(), params)
        .cast([csid("score")])
        .validateNumber(csid("score"), new NumberValidationOpts(0.0, 10.0, null, null, null));
      assert(cs.isValid) { "5 > 0 and < 10 should pass" };
    }

    test("validateNumber non-numeric value") {
      let params = new Map<String, String>([new Pair("age", "abc")]);
      let cs = changeset(userTable(), params)
        .cast([csid("age")])
        .validateNumber(csid("age"), new NumberValidationOpts(0.0, null, null, null, null));
      assert(!cs.isValid) { "non-numeric should fail" };
      assert(cs.errors[0].message == "must be a number") { "correct error message" };
    }

    test("validateNumber skips when field not in changes") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params)
        .cast([csid("age")])
        .validateNumber(csid("age"), new NumberValidationOpts(0.0, null, null, null, null));
      assert(cs.isValid) { "should be valid when field absent" };
    }

## validateAcceptance

    test("validateAcceptance passes for true values") {
      for (let v of ["true", "1", "yes", "on"]) {
        let params = new Map<String, String>([new Pair("active", v)]);
        let cs = changeset(userTable(), params)
          .cast([csid("active")])
          .validateAcceptance(csid("active"));
        assert(cs.isValid) { "should accept: ${v}" };
      }
    }

    test("validateAcceptance fails for non-true values") {
      let params = new Map<String, String>([new Pair("active", "false")]);
      let cs = changeset(userTable(), params)
        .cast([csid("active")])
        .validateAcceptance(csid("active"));
      assert(!cs.isValid) { "false should not be accepted" };
      assert(cs.errors[0].message == "must be accepted") { "correct message" };
    }

## validateConfirmation

    test("validateConfirmation passes when fields match") {
      let tbl = new TableDef(csid("users"), [
        new FieldDef(csid("password"), new StringField(), false),
        new FieldDef(csid("password_confirmation"), new StringField(), true),
      ]);
      let params = new Map<String, String>([
        new Pair("password", "secret123"),
        new Pair("password_confirmation", "secret123"),
      ]);
      let cs = changeset(tbl, params)
        .cast([csid("password"), csid("password_confirmation")])
        .validateConfirmation(csid("password"), csid("password_confirmation"));
      assert(cs.isValid) { "matching fields should pass" };
    }

    test("validateConfirmation fails when fields differ") {
      let tbl = new TableDef(csid("users"), [
        new FieldDef(csid("password"), new StringField(), false),
        new FieldDef(csid("password_confirmation"), new StringField(), true),
      ]);
      let params = new Map<String, String>([
        new Pair("password", "secret123"),
        new Pair("password_confirmation", "wrong456"),
      ]);
      let cs = changeset(tbl, params)
        .cast([csid("password"), csid("password_confirmation")])
        .validateConfirmation(csid("password"), csid("password_confirmation"));
      assert(!cs.isValid) { "mismatched fields should fail" };
      assert(cs.errors[0].field == "password_confirmation") { "error on confirmation field" };
    }

    test("validateConfirmation fails when confirmation missing") {
      let tbl = new TableDef(csid("users"), [
        new FieldDef(csid("password"), new StringField(), false),
        new FieldDef(csid("password_confirmation"), new StringField(), true),
      ]);
      let params = new Map<String, String>([
        new Pair("password", "secret123"),
      ]);
      let cs = changeset(tbl, params)
        .cast([csid("password")])
        .validateConfirmation(csid("password"), csid("password_confirmation"));
      assert(!cs.isValid) { "missing confirmation should fail" };
    }

## validateContains

    test("validateContains passes when substring found") {
      let params = new Map<String, String>([new Pair("email", "alice@example.com")]);
      let cs = changeset(userTable(), params)
        .cast([csid("email")])
        .validateContains(csid("email"), "@");
      assert(cs.isValid) { "should pass when @ present" };
    }

    test("validateContains fails when substring not found") {
      let params = new Map<String, String>([new Pair("email", "alice-example.com")]);
      let cs = changeset(userTable(), params)
        .cast([csid("email")])
        .validateContains(csid("email"), "@");
      assert(!cs.isValid) { "should fail when @ absent" };
    }

    test("validateContains skips when field not in changes") {
      let params = new Map<String, String>([]);
      let cs = changeset(userTable(), params)
        .cast([csid("email")])
        .validateContains(csid("email"), "@");
      assert(cs.isValid) { "should be valid when field absent" };
    }

## validateStartsWith

    test("validateStartsWith passes") {
      let params = new Map<String, String>([new Pair("name", "Dr. Smith")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateStartsWith(csid("name"), "Dr.");
      assert(cs.isValid) { "should pass for Dr. prefix" };
    }

    test("validateStartsWith fails") {
      let params = new Map<String, String>([new Pair("name", "Mr. Smith")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateStartsWith(csid("name"), "Dr.");
      assert(!cs.isValid) { "should fail for Mr. prefix" };
    }

## validateEndsWith

    test("validateEndsWith passes") {
      let params = new Map<String, String>([new Pair("email", "alice@example.com")]);
      let cs = changeset(userTable(), params)
        .cast([csid("email")])
        .validateEndsWith(csid("email"), ".com");
      assert(cs.isValid) { "should pass for .com suffix" };
    }

    test("validateEndsWith fails") {
      let params = new Map<String, String>([new Pair("email", "alice@example.org")]);
      let cs = changeset(userTable(), params)
        .cast([csid("email")])
        .validateEndsWith(csid("email"), ".com");
      assert(!cs.isValid) { "should fail for .org when expecting .com" };
    }

    test("validateEndsWith handles repeated suffix correctly") {
      let params = new Map<String, String>([new Pair("name", "abcabc")]);
      let cs = changeset(userTable(), params)
        .cast([csid("name")])
        .validateEndsWith(csid("name"), "abc");
      assert(cs.isValid) { "abcabc should end with abc" };
    }
