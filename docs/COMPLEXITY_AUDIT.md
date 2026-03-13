# Complexity Audit — Phase 3

> APPEND ONLY — Do not edit or remove entries. Add new findings below existing ones.

**Date:** 2026-03-13
**Source files analyzed:** 6 (schema, sql_model, sql_builder, query, changeset, orm)
**Total source lines:** ~1,591

---

## Core vs. Edge Classification

### Core (Pure Functional — Should Be Simple, Highly Testable)

| Component | File | Lines | Assessment |
|-----------|------|-------|------------|
| SafeIdentifier validation | schema.temper.md | ~30 | **CLEAN** — Regex validation, sealed interface, private impl. Textbook "push validation to the edge." |
| SqlPart.formatTo() hierarchy | sql_model.temper.md | ~80 | **CLEAN** — Sealed interface with 8 simple implementations. Each formats a single type. No side effects. |
| SqlBuilder append methods | sql_builder.temper.md | ~80 | **CLEAN** — Append-only accumulator. Each method delegates to the appropriate SqlPart. |
| FieldType sealed interface | schema.temper.md | ~10 | **CLEAN** — 6 marker types, no behavior. |
| TableDef data class | schema.temper.md | ~20 | **CLEAN** — Simple data holder with field lookup. |

### Edge (Orchestration — Acceptable Complexity)

| Component | File | Lines | Assessment |
|-----------|------|-------|------------|
| Query.toSql() | query.temper.md | ~95 | **MODERATE** — Assembles 10 SQL clauses sequentially. Linear complexity but long. |
| Query.countSql() | query.temper.md | ~35 | **DUPLICATE** — Copy of WHERE/JOIN/GROUP BY/HAVING rendering from toSql() |
| UpdateQuery.toSql() | query.temper.md | ~30 | **DUPLICATE** — Copy of WHERE rendering from Query.toSql() |
| DeleteQuery.toSql() | query.temper.md | ~20 | **DUPLICATE** — Copy of WHERE rendering from Query.toSql() |
| Changeset.toInsertSql() | changeset.temper.md | ~50 | **MIXED** — Validation + SQL generation + default handling + virtual field skipping interleaved |
| Changeset.toUpdateSql() | changeset.temper.md | ~25 | **MODERATE** — Similar concerns but smaller |
| Changeset validators | changeset.temper.md | ~200 | **LEAKING** — Each validator repeats error construction boilerplate. Core validation logic is buried in edge plumbing. |

---

## Finding 1: WHERE Rendering Duplication (4 copies)

**Severity:** MEDIUM
**Files:** query.temper.md
**Impact:** Maintenance risk, inconsistency risk, testing cost

The WHERE clause rendering pattern appears in **4 separate methods**:

1. `Query.toSql()` (lines 416-425)
2. `Query.countSql()` (lines 493-501)
3. `UpdateQuery.toSql()` (lines 736-743)
4. `DeleteQuery.toSql()` (lines 792-798)

Each copy follows the identical pattern:
```
if (!conditions.isEmpty) {
  b.appendSafe(" WHERE ");
  b.appendFragment(conditions[0].condition);
  for (var i = 1; i < conditions.length; ++i) {
    b.appendSafe(" ");
    b.appendSafe(conditions[i].keyword());
    b.appendSafe(" ");
    b.appendFragment(conditions[i].condition);
  }
}
```

**Testability impact:** To verify WHERE rendering correctness, you must test it through 4 different public surfaces (Query, countSql, UpdateQuery, DeleteQuery). If a bug is fixed in one copy, the other three may be missed.

**Refactoring opportunity:** Extract a free function `renderWhere(b: SqlBuilder, conditions: List<WhereClause>): Void` that all four methods call. This would be a pure core function that can be tested once.

---

## Finding 2: JOIN + GROUP BY + HAVING Rendering Duplication (2 copies)

**Severity:** LOW
**Files:** query.temper.md
**Impact:** Maintenance risk

JOIN rendering appears in both `toSql()` (lines 404-414) and `countSql()` (lines 482-492). GROUP BY and HAVING rendering also appears in both. This is a 3x duplication (3 clause types x 2 methods).

**Refactoring opportunity:** Extract `renderJoins()`, `renderGroupBy()`, `renderHaving()` helpers. Combined with Finding 1, `countSql()` would reduce to:
```
renderFrom() + renderJoins() + renderWhere() + renderGroupBy() + renderHaving()
```

---

## Finding 3: Query Constructor God Object (12 Parameters)

**Severity:** MEDIUM
**Files:** query.temper.md
**Impact:** Every mutation method must copy all 12 fields

The `Query` class constructor takes 12 parameters. Every builder method (30+ methods) creates a `new Query(...)` copying all 12 fields, changing only 1-2. This means:

- Adding a new feature (e.g., `WINDOW` clause) requires modifying **every** builder method's `new Query(...)` call
- Easy to accidentally swap or drop a parameter
- No compiler help if you pass parameters in the wrong order (many are `List<...>`)

Current builder method pattern (repeated 30+ times):
```
new Query(tableName, nb.toList(), selectedFields, orderClauses, limitVal, offsetVal, joinClauses, groupByFields, havingConditions, isDistinct, selectExprs, lockMode)
```

**Testability impact:** Low — the pattern is mechanical. But any new constructor parameter requires touching 30+ lines.

**Refactoring opportunity:** A `copy()` method that defaults to all current values, or a builder pattern. However, Temper may not support named/default parameters, which limits options. This finding is an observation, not necessarily actionable now.

---

## Finding 4: Changeset Validator Boilerplate (15 copies)

**Severity:** HIGH
**Files:** changeset.temper.md
**Impact:** Edge complexity leaking into core validation logic

Every validator follows the same boilerplate pattern (15 instances):

```temper
public validateX(field: SafeIdentifier, ...): Changeset {
  if (!_isValid) { return this; }           // Guard 1: already invalid
  if (!_changes.has(field.sqlValue)) { ... } // Guard 2: field absent
  let val = _changes.getOr(field.sqlValue, "");
  // ... actual validation logic (1-5 lines) ...
  if (/* invalid */) {
    let eb = _errors.toListBuilder();       // Error construction (3 lines, repeated 15x)
    eb.add(new ChangesetError(field.sqlValue, "..."));
    return new ChangesetImpl(_tableDef, _params, _changes, eb.toList(), false);
  }
  this
}
```

The error construction block (`let eb = ...; eb.add(...); return new ChangesetImpl(...)`) appears **15 times** with identical structure. The actual validation logic (the "core") is 1-5 lines buried inside 8-15 lines of boilerplate (the "edge").

**Testability impact:** HIGH — The boilerplate cannot be tested independently. Each validator must be tested as a whole, which means testing the same boilerplate 15 times (already-invalid guard, field-absent guard, error construction). The actual validation logic is a tiny fraction of what's tested.

**Refactoring opportunity:** Extract a private `addError(field, message)` method:
```temper
private addError(field: String, message: String): Changeset {
  let eb = _errors.toListBuilder();
  eb.add(new ChangesetError(field, message));
  new ChangesetImpl(_tableDef, _params, _changes, eb.toList(), false)
}
```

This reduces each validator's error path from 3 lines to 1 line: `return addError(field.sqlValue, "...")`. Would eliminate ~30 lines of duplication.

---

## Finding 5: validateNumber Is a 50-Line Method With 5 Independent Checks

**Severity:** MEDIUM
**Files:** changeset.temper.md (lines 260-312)
**Impact:** Largest single method, tests only cover 3 of 5 check paths

`validateNumber` has 5 nullable option fields, each checked independently:
1. `greaterThan` — tested
2. `lessThan` — tested
3. `greaterThanOrEqual` — tested (boundary only, not failure)
4. `lessThanOrEqual` — **NOT TESTED**
5. `equalTo` — **NOT TESTED**

Each check follows the same pattern: null-check the option, compare, add error. The method could be decomposed into 5 individual checks, but that's over-engineering for 5 simple comparisons. The real issue is the **untested code paths**, not the complexity.

**Testability impact:** MEDIUM — The method is long but linear. Each path is independent.

---

## Finding 6: toInsertSql Mixed Responsibilities

**Severity:** MEDIUM
**Files:** changeset.temper.md (lines 437-484)
**Impact:** 4 concerns interleaved in one method

`toInsertSql()` performs four tasks in one method:
1. **Validation**: Check `_isValid`, check non-nullable fields without defaults (lines 438-446)
2. **Change processing**: Iterate changes, skip virtual fields, convert values via `valueToSqlPart` (lines 450-457)
3. **Default handling**: Add fields with defaults not in changes (lines 459-469)
4. **SQL generation**: Build the INSERT INTO statement (lines 471-483)

**Testability impact:** To test default handling, you must also set up a valid changeset with non-nullable fields satisfied. To test virtual field skipping, you must set up the entire pipeline. The concerns are coupled.

**Refactoring opportunity:** The four steps could be extracted:
- `validateNonNullableFields()` — pure validation
- `collectColumnValues()` — returns column/value pairs
- `renderInsert(cols, vals)` — pure SQL generation

This would allow testing each concern independently.

---

## Finding 7: No Shared Base Between Query, UpdateQuery, DeleteQuery

**Severity:** LOW
**Files:** query.temper.md
**Impact:** Code duplication (Finding 1), no polymorphic testing

`Query`, `UpdateQuery`, and `DeleteQuery` are completely independent classes. They share:
- WHERE clause management (conditions field, where/orWhere methods)
- WHERE rendering (Finding 1)
- Limit handling

They don't implement a shared interface or trait. This means:
- WHERE-related tests must be duplicated for each query type
- There's no `Queryable` interface for generic code

**Testability impact:** LOW — each class is small enough to test independently. The duplication is a maintenance concern, not a testability concern.

---

## Finding 8: putChange Rebuilds Map From Scratch

**Severity:** LOW
**Files:** changeset.temper.md (lines 197-205)
**Impact:** Performance only, no correctness or testability concern

`putChange()` iterates all pairs of the existing map and rebuilds it from scratch, then adds the new key. This is O(n) per put. For typical ORM usage (small number of fields), this is fine. But it's notable as a design choice.

`deleteChange()` has the same pattern (lines 214-223).

---

## Edge-to-Core Leakage Summary

| Leakage | Where | Core Logic | Edge Plumbing | Ratio |
|---------|-------|-----------|---------------|-------|
| Validator boilerplate | changeset.temper.md | 1-5 lines per validator | 8-15 lines per validator | ~1:3 |
| WHERE rendering | query.temper.md | 8 lines (the rendering) | 4 copies x 8 = 32 lines | 1:4 |
| toInsertSql | changeset.temper.md | ~15 lines (value conversion) | ~35 lines (validation + defaults + SQL assembly) | ~1:2.3 |
| Error construction | changeset.temper.md | 0 (pure boilerplate) | 3 lines x 15 = 45 lines | N/A |

**Total duplicated/boilerplate code:** ~107 lines across 1,591 total (~6.7%)

---

## Testability Barriers

### Barrier 1: Sealed ChangesetImpl

`ChangesetImpl` is private. All testing must go through the `changeset()` factory -> `cast()` -> validator pipeline. You cannot construct a Changeset in a specific state for targeted testing (e.g., "already invalid with 2 errors").

**Impact:** HIGH for testing the already-invalid early return paths. You must first create an invalid changeset by failing a validator, then chain another validator to test the early return. This is indirect.

**Mitigation:** Not a refactoring target (the sealed interface is a security feature). But tests should be written to explicitly exercise the cast -> fail -> chain pattern.

### Barrier 2: No Direct SqlPart Construction Tests

`SqlPart` implementations are constructed and used inline. There's no test that constructs a `new SqlInt32(-1)` directly and calls `formatTo()`. All testing goes through the `sql` tag or changeset pipeline.

**Impact:** LOW — the `sql` tag exercises all paths. But direct unit tests would be more targeted.

### Barrier 3: toInsertSql Type Dispatch Coverage

The `valueToSqlPart` method dispatches on all 6 `FieldType` subtypes, but tests only exercise `StringField` and `IntField` through the full INSERT pipeline. `Int64Field`, `FloatField`, `BoolField`, `DateField` are never tested through `toInsertSql()`.

**Impact:** HIGH — these are real code paths that could have bugs (e.g., wrong parsing, wrong SqlPart type) that would only be caught in production.

---

## Recommendations (Priority Order)

### Must Do (Before Production)

1. **Test `valueToSqlPart` for all 6 field types** through `toInsertSql()` — 4 untested type dispatch paths
2. **Test `validateNumber` with `equalTo` and `lessThanOrEqual`** — 2 completely untested code paths in production code
3. **Test already-invalid early return** with at least 1 composition test chaining 3+ validators
4. **Test `validateFloat` invalid input** — zero tests for the error path

### Should Do (Improves Testability)

5. **Extract `addError()` helper** in ChangesetImpl — eliminates 30 lines of duplication, simplifies all validators
6. **Extract `renderWhere()` helper** in Query — eliminates 4 copies, enables single-point testing
7. **Test query builder immutability** — derive two queries from same base, verify independence

### Nice to Have (Reduces Maintenance Burden)

8. **Extract `renderJoins()`, `renderGroupBy()`, `renderHaving()`** helpers
9. **Decompose `toInsertSql()`** into validation + collection + rendering
10. **Add boundary tests** for limit(0), validateLength at min/max, validateInt boundaries

---

## Appendix: Method Complexity Ranking

| Rank | Method | Lines | Cyclomatic | Notes |
|------|--------|-------|-----------|-------|
| 1 | Query.toSql() | 95 | 12 | 10 conditional clauses, linear flow |
| 2 | validateNumber() | 52 | 11 | 5 nullable checks + parse check |
| 3 | toInsertSql() | 48 | 8 | 4 loops + 3 conditions |
| 4 | countSql() | 36 | 6 | Subset of toSql() |
| 5 | toUpdateSql() | 26 | 5 | Loop + conditions |
| 6 | validateEndsWith() | 33 | 5 | Cursor-based comparison |
| 7 | UpdateQuery.toSql() | 30 | 4 | SET + WHERE |
| 8 | DeleteQuery.toSql() | 20 | 3 | WHERE only |
| 9 | cast() | 10 | 2 | Simple filter |
| 10 | whereIn() | 16 | 2 | Empty check + loop |

All other methods are 1-8 lines with cyclomatic complexity 1-2.
