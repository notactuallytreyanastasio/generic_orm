# Coverage Audit — Phase 1

> APPEND ONLY — Do not edit or remove entries. Add new findings below existing ones.

**Date:** 2026-03-13
**Test count at audit time:** 184 tests across 4 files
**Source files analyzed:** 6 (schema, sql_model, sql_builder, query, changeset, orm)
**Estimated branch coverage:** ~72%

---

## Source File: schema.temper.md

### Exported Symbols

| Symbol | Kind |
|--------|------|
| `SafeIdentifier` | sealed interface |
| `safeIdentifier()` | factory function |
| `FieldType` | sealed interface (6 subtypes) |
| `FieldDef` | class (5 params) |
| `TableDef` | class (3 params) |
| `timestamps()` | factory function |

### Code Path Coverage

#### `safeIdentifier(name: String) -> SafeIdentifier`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Empty string -> bubble | YES | schema_test: "rejects empty string" |
| 2 | Leading digit -> bubble | YES | schema_test: "rejects leading digit" |
| 3 | Invalid char (6 metacharacters) -> bubble | YES | schema_test: "rejects SQL metacharacters" |
| 4 | Valid name -> ValidatedIdentifier | YES | schema_test: "accepts valid names" |
| 5 | Single-char name ("a", "_") | **NOT TESTED** | |
| 6 | All-underscore name ("___") | **NOT TESTED** | |
| 7 | Unicode characters | **NOT TESTED** | |
| 8 | Null bytes / control characters | **NOT TESTED** | |
| 9 | Maximum-length identifier | **NOT TESTED** | |

#### `TableDef.field(name: String) -> FieldDef`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Field found -> return FieldDef | YES | schema_test: "field lookup - found" |
| 2 | Field not found -> bubble | YES | schema_test: "field lookup - not found bubbles" |
| 3 | Empty field list | **NOT TESTED** | |
| 4 | Duplicate field names | **NOT TESTED** | |
| 5 | First/last element in list | **NOT TESTED** | |

#### `TableDef.pkName() -> String`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | primaryKey is null -> "id" | YES | schema_test: "pkName defaults to id" |
| 2 | primaryKey is non-null -> pk.sqlValue | YES | schema_test: "pkName returns custom primary key" |

#### `timestamps() -> List<FieldDef>`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Returns 2 DateField defs with defaults | YES | schema_test: "timestamps returns two DateField defs" |

#### FieldType subtypes (StringField, IntField, Int64Field, FloatField, BoolField, DateField)

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Construction of each subtype | **NOT TESTED** | No test verifies each subclass individually |

---

## Source File: sql_model.temper.md

### Exported Symbols

| Symbol | Kind |
|--------|------|
| `SqlFragment` | class |
| `SqlPart` | sealed interface |
| `SqlSource` | class |
| `SqlBoolean` | class |
| `SqlDate` | class |
| `SqlFloat64` | class |
| `SqlInt32` | class |
| `SqlInt64` | class |
| `SqlDefault` | class |
| `SqlString` | class |

### Code Path Coverage

#### `SqlFragment.toString()`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Empty parts list | **NOT TESTED** | |
| 2 | Single part | YES | implicit in many tests |
| 3 | Multiple parts | YES | implicit in all sql tag tests |

#### `SqlFragment.toSource()`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Converts to SqlSource | YES | sql_tests: "nesting" |

#### `SqlSource.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Appends source string | YES | implicit in all tests |

#### `SqlBoolean.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | value=true -> "TRUE" | YES | sql_tests: "numbers and booleans" |
| 2 | value=false -> "FALSE" | YES | sql_tests: "numbers and booleans" |

#### `SqlDate.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Normal date -> quoted ISO string | YES | sql_tests: "SqlDate renders with quotes" |
| 2 | Date with single quote in toString() | **NOT TESTED** | defense-in-depth path |

#### `SqlFloat64.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Normal value -> numeric string | YES | sql_tests: "normal values still work" |
| 2 | NaN -> "NULL" | YES | sql_tests: "NaN renders as NULL" |
| 3 | Infinity -> "NULL" | YES | sql_tests: "Infinity renders as NULL" |
| 4 | -Infinity -> "NULL" | YES | sql_tests: "negative Infinity renders as NULL" |
| 5 | Zero -> "0.0" | YES | sql_tests: tests 0.0 |
| 6 | Negative normal value | YES | sql_tests: tests -42.5 |

#### `SqlInt32.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Positive integer | YES | sql_tests: 42 |
| 2 | Negative integer | **NOT TESTED** | |
| 3 | Zero | **NOT TESTED** | |
| 4 | Int32.MIN_VALUE / Int32.MAX_VALUE | **NOT TESTED** | |

#### `SqlInt64.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Normal value | YES | sql_tests: 43i64 |
| 2 | Negative Int64 | **NOT TESTED** | |
| 3 | Large value near Int64 limits | **NOT TESTED** | |

#### `SqlDefault.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Renders "DEFAULT" | YES | changeset_test: "toInsertSql uses default value" |

#### `SqlString.formatTo(builder)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Normal string -> quoted | YES | sql_tests: "string escaping" |
| 2 | Single quotes -> doubled | YES | sql_tests: Bobby Tables |
| 3 | Empty string -> '' | YES | sql_tests: "string edge cases" |
| 4 | Already-doubled quotes | YES | sql_tests: "a''b" |
| 5 | Unicode string | YES | sql_tests: "Hello world" |
| 6 | Multiline string | YES | sql_tests: "Line1\nLine2" |
| 7 | Backslash characters | **NOT TESTED** | |
| 8 | Null byte (\0) | **NOT TESTED** | |

---

## Source File: sql_builder.temper.md

### Exported Symbols

| Symbol | Kind |
|--------|------|
| `sql` | let alias to SqlBuilder (tag function) |
| `SqlBuilder` | class |

### Code Path Coverage

| # | Method | Tested? | Test |
|---|--------|---------|------|
| 1 | `appendSafe` | YES | implicit in all sql tag tests |
| 2 | `appendFragment` | YES | sql_tests: "nesting" |
| 3 | `appendPart` | YES | sql_tests: "nesting" with SqlPart list |
| 4 | `appendPartList` | YES | sql_tests: "nesting" with List<SqlPart> |
| 5 | `appendBoolean` | YES | sql_tests: "numbers and booleans" |
| 6 | `appendBooleanList` | YES | sql_tests: "lists" |
| 7 | `appendDate` | YES | sql_tests: "numbers and booleans" |
| 8 | `appendDateList` | YES | sql_tests: "lists" |
| 9 | `appendFloat64` | YES | sql_tests: "numbers and booleans" |
| 10 | `appendFloat64List` | YES | sql_tests: "lists" |
| 11 | `appendInt32` | YES | sql_tests: "numbers and booleans" |
| 12 | `appendInt32List` | YES | sql_tests: "lists" |
| 13 | `appendInt64` | YES | sql_tests: "numbers and booleans" |
| 14 | `appendInt64List` | YES | sql_tests: "lists" |
| 15 | `appendString` | YES | sql_tests: "string escaping" |
| 16 | `appendStringList` | YES | sql_tests: "lists" |
| 17 | `appendList` empty list | **NOT TESTED** | |
| 18 | `appendList` single element | **NOT TESTED** | all list tests have 2+ elements |
| 19 | `accumulated` getter | YES | implicit in all tests |

---

## Source File: query.temper.md

### Exported Symbols

| Symbol | Kind |
|--------|------|
| `JoinType` | sealed interface (5 subtypes) |
| `JoinClause` | class |
| `NullsPosition` | sealed interface (2 subtypes) |
| `OrderClause` | class |
| `LockMode` | sealed interface (2 subtypes) |
| `WhereClause` | sealed interface (2 subtypes) |
| `Query` | class (12 params, 30+ methods) |
| `from()` | factory function |
| `col()` | helper function |
| `countAll/countCol/sumCol/avgCol/minCol/maxCol` | aggregate functions |
| `unionSql/unionAllSql/intersectSql/exceptSql` | set operations |
| `subquery/existsSql` | subquery helpers |
| `SetClause` | class |
| `UpdateQuery` | class |
| `DeleteQuery` | class |
| `update()/deleteFrom()` | factory functions |

### Code Path Coverage — Query

#### `Query.toSql()`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | SELECT * (no selectedFields, no selectExprs) | YES | query_test: "bare from" |
| 2 | SELECT with selectedFields | YES | query_test: "select restricts columns" |
| 3 | SELECT with selectExprs (overrides selectedFields) | YES | query_test: "selectExpr overrides" |
| 4 | SELECT DISTINCT | YES | query_test: "distinct basic" |
| 5 | FROM table | YES | all query tests |
| 6 | JOIN (all 5 types: INNER, LEFT, RIGHT, FULL, CROSS) | YES | query_test: phases 0+5 |
| 7 | CROSS JOIN with null onCondition | YES | query_test: "crossJoin" |
| 8 | WHERE with AND conditions | YES | query_test: "chained where" |
| 9 | WHERE with OR conditions | YES | query_test: "orWhere" |
| 10 | WHERE mixed AND/OR | YES | query_test: "mixed where and orWhere" |
| 11 | WHERE IS NULL / IS NOT NULL | YES | query_test |
| 12 | WHERE IN (values) | YES | query_test |
| 13 | WHERE IN (empty) -> 1=0 | YES | query_test: "whereIn empty list" |
| 14 | WHERE NOT() | YES | query_test |
| 15 | WHERE BETWEEN | YES | query_test |
| 16 | WHERE LIKE / ILIKE | YES | query_test |
| 17 | WHERE IN (subquery) | YES | query_test |
| 18 | GROUP BY | YES | query_test: "groupBy" |
| 19 | HAVING / OR HAVING | YES | query_test: "having/orHaving" |
| 20 | ORDER BY ASC/DESC | YES | query_test |
| 21 | ORDER BY NULLS FIRST/LAST | YES | query_test: phase 5 |
| 22 | LIMIT | YES | query_test |
| 23 | OFFSET | YES | query_test |
| 24 | FOR UPDATE / FOR SHARE | YES | query_test: phase 5 |
| 25 | Empty select([]) | **NOT TESTED** | |
| 26 | Empty selectExpr([]) | **NOT TESTED** | |
| 27 | limit(0) | **NOT TESTED** | |
| 28 | offset(0) without limit | **NOT TESTED** (only tested in complex query) | |
| 29 | HAVING without GROUP BY | **NOT TESTED** | |
| 30 | Multiple distinct() calls | **NOT TESTED** | |
| 31 | Multiple lock() calls | **NOT TESTED** | |

#### `Query.safeToSql(defaultLimit)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | No limit set -> applies defaultLimit | YES | query_test: "safeToSql applies default limit" |
| 2 | Explicit limit set -> keeps it | YES | query_test: "safeToSql respects explicit limit" |
| 3 | Negative defaultLimit -> bubble | YES | query_test: "safeToSql bubbles on negative" |
| 4 | defaultLimit = 0 | **NOT TESTED** | |

#### `Query.countSql()`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Bare count | YES | query_test: "countSql bare" |
| 2 | Count with WHERE | YES | query_test: "countSql with WHERE" |
| 3 | Count with JOIN | YES | query_test: "countSql with JOIN" |
| 4 | Drops orderBy/limit/offset | YES | query_test: "countSql drops orderBy" |
| 5 | Count with HAVING | **NOT TESTED** | |
| 6 | Count with CROSS JOIN | **NOT TESTED** | |

#### Error paths (bubble)

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | limit(-1) | YES | query_test |
| 2 | offset(-1) | YES | query_test |
| 3 | safeToSql(-1) | YES | query_test |

### Code Path Coverage — UpdateQuery

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Basic UPDATE with SET and WHERE | YES | query_test |
| 2 | Multiple SET clauses | YES | query_test |
| 3 | Multiple WHERE (AND) | YES | query_test |
| 4 | OR WHERE | YES | query_test |
| 5 | WITH LIMIT | YES | query_test |
| 6 | Bubbles without WHERE | YES | query_test: security-critical |
| 7 | Bubbles without SET | YES | query_test |
| 8 | SQL escaping in SET | YES | query_test: "UpdateQuery escaping" |
| 9 | Negative limit -> bubble | **NOT TESTED** | |

### Code Path Coverage — DeleteQuery

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Basic DELETE | YES | query_test |
| 2 | Multiple WHERE (AND) | YES | query_test |
| 3 | OR WHERE | YES | query_test |
| 4 | WITH LIMIT | YES | query_test |
| 5 | Bubbles without WHERE | YES | query_test: security-critical |
| 6 | Negative limit -> bubble | **NOT TESTED** | |

---

## Source File: changeset.temper.md

### Exported Symbols

| Symbol | Kind |
|--------|------|
| `ChangesetError` | class |
| `NumberValidationOpts` | class (5 nullable Float64 params) |
| `Changeset` | sealed interface (20+ methods) |
| `changeset()` | factory function |

### Code Path Coverage — ChangesetImpl

#### `cast(allowedFields: List<SafeIdentifier>)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Whitelists only allowed fields | YES | changeset_test: "cast whitelists" |
| 2 | Replaces previous cast (not additive) | YES | changeset_test: "cast is replacing" |
| 3 | Ignores empty string values | YES | changeset_test: "cast ignores empty strings" |
| 4 | Empty allowedFields list | **NOT TESTED** | |

#### `validateRequired(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Field present -> pass | YES | changeset_test |
| 2 | Field missing -> fail | YES | changeset_test |

#### `validateLength(field, min, max)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Within range -> pass | YES | changeset_test |
| 2 | Below min -> fail | YES | changeset_test |
| 3 | Above max -> fail | YES | changeset_test |
| 4 | Exactly min length | **NOT TESTED** | |
| 5 | Exactly max length | **NOT TESTED** | |
| 6 | min == max | **NOT TESTED** | |
| 7 | min > max | **NOT TESTED** | |
| 8 | Field not in changes | **NOT TESTED** | |

#### `validateInt(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Valid integer | YES | changeset_test |
| 2 | Non-integer string | YES | changeset_test |
| 3 | Negative integer | **NOT TESTED** | |
| 4 | Zero | **NOT TESTED** | |
| 5 | Int32 boundary values | **NOT TESTED** | |

#### `validateInt64(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Valid int64 | YES | changeset_test |
| 2 | Non-integer string | YES | changeset_test |
| 3 | Int64 boundary values | **NOT TESTED** | |

#### `validateFloat(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Valid float | YES | changeset_test |
| 2 | Non-float string | **NOT TESTED** | |
| 3 | Edge cases (NaN, Infinity, -0.0) | **NOT TESTED** | |

#### `validateBool(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Truthy values (true/1/yes/on) | YES | changeset_test |
| 2 | Falsy values (false/0/no/off) | YES | changeset_test |
| 3 | Ambiguous values (5 patterns) | YES | changeset_test |

#### `validateNumber(field, opts: NumberValidationOpts)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | greaterThan pass | YES | changeset_test |
| 2 | greaterThan fail | YES | changeset_test |
| 3 | greaterThan at exact threshold | **NOT TESTED** | |
| 4 | lessThan pass | YES | changeset_test |
| 5 | lessThan fail | YES | changeset_test |
| 6 | lessThan at exact threshold | **NOT TESTED** | |
| 7 | greaterThanOrEqual boundary | YES | changeset_test |
| 8 | greaterThanOrEqual fail | **NOT TESTED** | |
| 9 | lessThanOrEqual pass | **NOT TESTED** | code path exists, never tested |
| 10 | lessThanOrEqual fail | **NOT TESTED** | code path exists, never tested |
| 11 | equalTo pass | **NOT TESTED** | code path exists, never tested |
| 12 | equalTo fail | **NOT TESTED** | code path exists, never tested |
| 13 | Combined options | YES | changeset_test |
| 14 | Non-numeric value | YES | changeset_test |
| 15 | Field not in changes | YES | changeset_test |

#### `validateInclusion(field, allowed)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Value in list -> pass | YES | changeset_test |
| 2 | Value not in list -> fail | YES | changeset_test |
| 3 | Field not in changes -> skip | YES | changeset_test |
| 4 | Empty allowed list | **NOT TESTED** | |

#### `validateExclusion(field, disallowed)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Value not in list -> pass | YES | changeset_test |
| 2 | Value in list -> fail | YES | changeset_test |
| 3 | Field not in changes -> skip | YES | changeset_test |
| 4 | Empty disallowed list | **NOT TESTED** | |

#### `validateAcceptance(field)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Truthy values pass | YES | changeset_test |
| 2 | Non-truthy values fail | YES | changeset_test |
| 3 | Field not in changes -> skip | **NOT TESTED** | |

#### `validateConfirmation(field, confirmationField)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Fields match -> pass | YES | changeset_test |
| 2 | Fields differ -> fail | YES | changeset_test |
| 3 | Confirmation missing -> fail | YES | changeset_test |
| 4 | Both fields missing | **NOT TESTED** | |

#### `validateContains(field, substring)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Substring found -> pass | YES | changeset_test |
| 2 | Substring not found -> fail | YES | changeset_test |
| 3 | Field not in changes -> skip | YES | changeset_test |
| 4 | Empty substring | **NOT TESTED** | |

#### `validateStartsWith(field, prefix)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Starts with prefix -> pass | YES | changeset_test |
| 2 | Does not start -> fail | YES | changeset_test |
| 3 | Empty prefix | **NOT TESTED** | |
| 4 | Prefix longer than value | **NOT TESTED** | |
| 5 | Prefix == value | **NOT TESTED** | |
| 6 | Field not in changes | **NOT TESTED** | |

#### `validateEndsWith(field, suffix)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Ends with suffix -> pass | YES | changeset_test |
| 2 | Does not end -> fail | YES | changeset_test |
| 3 | Repeated suffix | YES | changeset_test |
| 4 | Empty suffix | **NOT TESTED** | |
| 5 | Suffix longer than value | **NOT TESTED** | |

#### `putChange / getChange / deleteChange`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | putChange adds new field | YES | changeset_test |
| 2 | putChange overwrites existing | YES | changeset_test |
| 3 | putChange appears in toInsertSql | YES | changeset_test |
| 4 | getChange exists | YES | changeset_test |
| 5 | getChange missing -> bubble | YES | changeset_test |
| 6 | deleteChange removes field | YES | changeset_test |
| 7 | deleteChange nonexistent no-op | YES | changeset_test |

#### `toInsertSql()`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Invalid changeset -> bubble | YES | changeset_test |
| 2 | Non-nullable field missing -> bubble | YES | changeset_test |
| 3 | Bobby Tables escaping | YES | changeset_test: security-critical |
| 4 | StringField type dispatch | YES | changeset_test |
| 5 | IntField type dispatch | YES | changeset_test |
| 6 | Int64Field type dispatch | **NOT TESTED** | |
| 7 | FloatField type dispatch | **NOT TESTED** | |
| 8 | BoolField type dispatch | **NOT TESTED** | |
| 9 | DateField type dispatch | **NOT TESTED** | |
| 10 | Default values (SqlDefault) | YES | changeset_test |
| 11 | Change overrides default | YES | changeset_test |
| 12 | Timestamps use DEFAULT | YES | changeset_test |
| 13 | Virtual fields skipped | YES | changeset_test |
| 14 | Non-nullable virtual field not enforced | YES | changeset_test |
| 15 | Empty changes (no columns) -> bubble | **NOT TESTED** | |

#### `toUpdateSql(id)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Invalid changeset -> bubble | YES | changeset_test |
| 2 | Single change, default PK | YES | changeset_test |
| 3 | Virtual fields skipped | YES | changeset_test |
| 4 | Custom primary key | YES | changeset_test |
| 5 | All virtual fields -> setCount 0 -> bubble | **NOT TESTED** | |
| 6 | Multiple non-virtual changes | **NOT TESTED** | |
| 7 | SQL escaping in update values | **NOT TESTED** | |
| 8 | Empty changes -> bubble | **NOT TESTED** | |

#### Already-invalid early return (`!_isValid -> return this`)

| # | Validator | Early return tested? |
|---|-----------|---------------------|
| 1 | validateRequired | **NOT TESTED** |
| 2 | validateLength | **NOT TESTED** |
| 3 | validateInt | **NOT TESTED** |
| 4 | validateInt64 | **NOT TESTED** |
| 5 | validateFloat | **NOT TESTED** |
| 6 | validateBool | **NOT TESTED** |
| 7 | validateNumber | **NOT TESTED** |
| 8 | validateInclusion | **NOT TESTED** |
| 9 | validateExclusion | **NOT TESTED** |
| 10 | validateAcceptance | **NOT TESTED** |
| 11 | validateConfirmation | **NOT TESTED** |
| 12 | validateContains | **NOT TESTED** |
| 13 | validateStartsWith | **NOT TESTED** |
| 14 | validateEndsWith | **NOT TESTED** |
| 15 | putChange | **NOT TESTED** |

> **Pattern note:** 15 validators have `if (!_isValid) { return this; }` at the top. NONE of these early-return paths are tested. A single composition test chaining 3+ validators on an invalid changeset would cover this class of paths.

---

## Source File: orm.temper.md

### Code Path Coverage

#### `deleteSql(tableDef, id)`

| # | Path | Tested? | Test |
|---|------|---------|------|
| 1 | Custom PK | YES | changeset_test: "deleteSql uses custom primary key" |
| 2 | Default PK "id" | YES | changeset_test: "deleteSql uses default id" |
| 3 | id = 0 | **NOT TESTED** | |
| 4 | Negative id | **NOT TESTED** | |

---

## Gap Summary

### By Severity

| Severity | Count | Description |
|----------|-------|-------------|
| CRITICAL | 4 | `valueToSqlPart` type dispatch for Int64Field/FloatField/BoolField/DateField |
| HIGH | 4 | `validateNumber` with equalTo/lessThanOrEqual (2 untested code paths), `validateFloat` invalid input, already-invalid early return pattern (15 validators) |
| MEDIUM | 12 | Boundary values (limit(0), validateLength at min/max, validateInt boundaries, etc.) |
| LOW | 15 | Edge cases (empty lists, single-char identifiers, unicode, null bytes, etc.) |

### Total Untested Code Paths: ~55 of ~195 mapped paths

### Paths tested: ~140 (~72%)

---

## Appendix: Files Analyzed

| File | Lines | Functions/Methods | Tests |
|------|-------|-------------------|-------|
| schema.temper.md | ~90 | 5 exported | 12 |
| sql_model.temper.md | ~120 | 10 exported classes | 10 |
| sql_builder.temper.md | ~110 | 18 methods + 1 alias | 10 (indirect) |
| query.temper.md | ~820 | 30+ methods, 6 aggregate fns, 4 set ops, 2 subquery helpers, UpdateQuery, DeleteQuery | 96 |
| changeset.temper.md | ~531 | 20+ methods via sealed interface | 66 |
| orm.temper.md | ~20 | 1 function | 2 (in changeset_test) |
