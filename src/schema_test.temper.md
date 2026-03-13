# Schema Tests

    test("safeIdentifier accepts valid names") {
      let id = safeIdentifier("user_name") orelse panic();
      assert(id.sqlValue == "user_name") { "value should round-trip" };
    }

    test("safeIdentifier rejects empty string") {
      let didBubble = do { safeIdentifier(""); false } orelse true;
      assert(didBubble) { "empty string should bubble" };
    }

    test("safeIdentifier rejects leading digit") {
      let didBubble = do { safeIdentifier("1col"); false } orelse true;
      assert(didBubble) { "leading digit should bubble" };
    }

    test("safeIdentifier rejects SQL metacharacters") {
      let cases = ["name); DROP TABLE", "col'", "a b", "a-b", "a.b", "a;b"];
      for (let c of cases) {
        let didBubble = do { safeIdentifier(c); false } orelse true;
        assert(didBubble) { "should reject: ${c}" };
      }
    }

    test("TableDef field lookup - found") {
      let td = new TableDef(
        safeIdentifier("users") orelse panic(),
        [
          new FieldDef(safeIdentifier("name") orelse panic(), new StringField(), false, null, false),
          new FieldDef(safeIdentifier("age")  orelse panic(), new IntField(),    false, null, false),
        ],
        null,
      );
      let f = td.field("age") orelse panic();
      assert(f.name.sqlValue == "age") { "should find age field" };
    }

    test("TableDef field lookup - not found bubbles") {
      let td = new TableDef(
        safeIdentifier("users") orelse panic(),
        [new FieldDef(safeIdentifier("name") orelse panic(), new StringField(), false, null, false)],
        null,
      );
      let didBubble = do { td.field("nonexistent"); false } orelse true;
      assert(didBubble) { "unknown field should bubble" };
    }

    test("FieldDef nullable flag") {
      let required = new FieldDef(safeIdentifier("email") orelse panic(), new StringField(), false, null, false);
      let optional = new FieldDef(safeIdentifier("bio")   orelse panic(), new StringField(), true, null, false);
      assert(!required.nullable) { "required field should not be nullable" };
      assert(optional.nullable)  { "optional field should be nullable" };
    }

## Phase 7: Schema Enrichment Tests

    test("pkName defaults to id when primaryKey is null") {
      let td = new TableDef(
        safeIdentifier("users") orelse panic(),
        [new FieldDef(safeIdentifier("name") orelse panic(), new StringField(), false, null, false)],
        null,
      );
      assert(td.pkName() == "id") { "default pk should be id" };
    }

    test("pkName returns custom primary key") {
      let td = new TableDef(
        safeIdentifier("users") orelse panic(),
        [new FieldDef(safeIdentifier("user_id") orelse panic(), new IntField(), false, null, false)],
        safeIdentifier("user_id") orelse panic(),
      );
      assert(td.pkName() == "user_id") { "custom pk should be user_id" };
    }

    test("timestamps returns two DateField defs") {
      let ts = timestamps() orelse panic();
      assert(ts.length == 2) { "should return 2 fields" };
      assert(ts[0].name.sqlValue == "inserted_at") { "first should be inserted_at" };
      assert(ts[1].name.sqlValue == "updated_at") { "second should be updated_at" };
      assert(ts[0].nullable) { "inserted_at should be nullable" };
      assert(ts[1].nullable) { "updated_at should be nullable" };
      assert(ts[0].defaultValue != null) { "inserted_at should have default" };
      assert(ts[1].defaultValue != null) { "updated_at should have default" };
    }

    test("FieldDef defaultValue field") {
      let withDefault = new FieldDef(
        safeIdentifier("status") orelse panic(), new StringField(), false, new SqlDefault(), false,
      );
      let withoutDefault = new FieldDef(
        safeIdentifier("name") orelse panic(), new StringField(), false, null, false,
      );
      assert(withDefault.defaultValue != null) { "should have default" };
      assert(withoutDefault.defaultValue == null) { "should not have default" };
    }

    test("FieldDef virtual flag") {
      let normal = new FieldDef(safeIdentifier("name") orelse panic(), new StringField(), false, null, false);
      let virt = new FieldDef(safeIdentifier("full_name") orelse panic(), new StringField(), true, null, true);
      assert(!normal.virtual) { "normal field should not be virtual" };
      assert(virt.virtual) { "virtual field should be virtual" };
    }
