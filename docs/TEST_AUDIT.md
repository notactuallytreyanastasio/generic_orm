# Test Audit — Phase 2

> APPEND ONLY — Do not edit or remove entries. Add new findings below existing ones.

**Date:** 2026-03-13
**Test count at audit time:** 184 tests across 4 files
**Rating scale:** STRONG (tests boundary/error/security) | ADEQUATE (correct happy-path) | WEAK (trivial property check) | USELESS (no value)

---

## Statistical Summary

| Category | Count | Percentage |
|----------|-------|------------|
| STRONG | ~58 | ~31% |
| ADEQUATE | ~108 | ~59% |
| WEAK | ~5 | ~3% |
| USELESS | 0 | 0% |
| **Total** | **184** | **100%** |

---

## File: schema_test.temper.md (12 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 1 | safeIdentifier accepts valid names | ADEQUATE | Tests one valid name. Missing: max-length, underscores-only, unicode |
| 2 | safeIdentifier rejects empty string | STRONG | Critical boundary |
| 3 | safeIdentifier rejects leading digit | STRONG | Important validation rule |
| 4 | safeIdentifier rejects SQL metacharacters | STRONG | 6 injection vectors in a loop |
| 5 | TableDef field lookup - found | ADEQUATE | Happy path only |
| 6 | TableDef field lookup - not found bubbles | STRONG | Error path |
| 7 | FieldDef nullable flag | WEAK | Just checks constructor param storage |
| 8 | pkName defaults to id when null | STRONG | Null-default behavior |
| 9 | pkName returns custom primary key | STRONG | Non-null case |
| 10 | timestamps returns two DateField defs | STRONG | Thorough: checks count, names, nullable, defaults |
| 11 | FieldDef defaultValue field | WEAK | Trivial constructor verification |
| 12 | FieldDef virtual flag | WEAK | Trivial constructor verification |

**Missing tests:**
- safeIdentifier with single-char names ("a", "_")
- safeIdentifier with unicode characters
- safeIdentifier with null bytes or control characters
- TableDef with empty field list
- TableDef.field() with duplicate field names
- FieldType sealed interface subclass construction

---

## File: sql_tests.temper.md (10 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 1 | string escaping (Bobby Tables) | STRONG | Core security test |
| 2 | string edge cases | STRONG | Empty, doubled quotes, unicode, newlines |
| 3 | numbers and booleans | ADEQUATE | Tests Int32, Int64, Float64, Boolean, Date in one assertion |
| 4 | lists | STRONG | All 6 types with proper escaping |
| 5 | SqlFloat64 NaN -> NULL | STRONG | ORM-3 regression test |
| 6 | SqlFloat64 Infinity -> NULL | STRONG | ORM-3 regression test |
| 7 | SqlFloat64 -Infinity -> NULL | STRONG | ORM-3 regression test |
| 8 | SqlFloat64 normal values | STRONG | Tests 3.14, 0.0, -42.5 |
| 9 | SqlDate renders with quotes | ADEQUATE | Happy path only |
| 10 | nesting | STRONG | Tests composability: appendFragment, toSource, appendPartList |

**Missing tests:**
- SqlString with backslash characters
- SqlString with null bytes
- SqlInt32 with MIN_VALUE / MAX_VALUE
- SqlInt64 with MIN_VALUE / MAX_VALUE
- SqlFloat64 with -0.0, very small/large numbers
- SqlDefault rendering (only tested indirectly)
- SqlBuilder.accumulated with no appends (empty fragment)
- Empty list rendering
- Single-element list rendering

---

## File: query_test.temper.md (96 tests)

### Core SELECT/WHERE/ORDER (16 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 1 | bare from produces SELECT * | ADEQUATE | Basic smoke test |
| 2 | select restricts columns | ADEQUATE | Happy path. Missing: empty column list |
| 3 | where with int value | ADEQUATE | Missing: null, negative, max-int |
| 4 | where with bool value | ADEQUATE | |
| 5 | chained where uses AND | STRONG | Composition test |
| 6 | orderBy ASC | ADEQUATE | |
| 7 | orderBy DESC | ADEQUATE | |
| 8 | limit and offset | ADEQUATE | Happy path |
| 9 | limit bubbles on negative | STRONG | Error path |
| 10 | offset bubbles on negative | STRONG | Error path |
| 11 | complex composed query | STRONG | 6-method chain |
| 12 | safeToSql default limit | STRONG | CWE-400 mitigation |
| 13 | safeToSql respects explicit | STRONG | Override path |
| 14 | safeToSql bubbles negative | STRONG | Error path |
| 15 | where injection attempt | STRONG | SQL injection security |
| 16 | safeIdentifier rejects metacharacters | STRONG | Defense-in-depth |

### JOIN tests (8 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 17-20 | innerJoin/leftJoin/rightJoin/fullJoin | ADEQUATE each |
| 21 | chained joins | ADEQUATE |
| 22 | join with where and orderBy | STRONG (integration) |
| 23-24 | col helper / col in join | ADEQUATE each |

### Phase 1: WHERE Enrichment (19 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 25 | orWhere basic | ADEQUATE | |
| 26 | where then orWhere | STRONG | AND/OR precedence |
| 27 | multiple orWhere | ADEQUATE | |
| 28 | mixed where and orWhere | STRONG | **NOTE:** Generates `WHERE a AND b OR c` without parens. SQL AND binds tighter than OR, so this means `(a AND b) OR c`. Behavior is correct but not documented. |
| 29-32 | whereNull/NotNull variants | ADEQUATE each |
| 33 | whereIn with ints | ADEQUATE |
| 34 | whereIn with strings | STRONG | Tests escaping with Bob's |
| 35 | whereIn empty list -> 1=0 | STRONG | Critical degenerate case |
| 36-37 | whereIn chained/single | ADEQUATE each |
| 38-39 | whereNot basic/chained | ADEQUATE each |
| 40-41 | whereBetween ints/chained | ADEQUATE | Missing: reversed bounds, equal bounds, negative numbers |
| 42-43 | whereLike/whereILike basic | ADEQUATE each |
| 44 | whereLike injection attempt | STRONG | Security test |
| 45 | whereLike wildcards | ADEQUATE | Missing: underscore wildcard, escape chars |

### Phase 2: Aggregation (17 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 46-51 | countAll/Col/sum/avg/min/max | ADEQUATE each | Trivial format tests |
| 52-53 | selectExpr single/multiple | ADEQUATE each |
| 54 | selectExpr overrides selectedFields | STRONG | Override interaction |
| 55-56 | groupBy single/multiple | ADEQUATE each |
| 57-58 | having/orHaving | ADEQUATE each |
| 59-60 | distinct basic/with where | ADEQUATE each |
| 61-63 | countSql bare/WHERE/JOIN | ADEQUATE each |
| 64 | countSql drops orderBy/limit/offset | STRONG | Correctness property |
| 65 | full aggregation query | STRONG | Comprehensive composition |

### Phase 3: Set Operations (10 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 66-69 | union/unionAll/intersect/except | ADEQUATE each |
| 70-71 | subquery/existsSql | ADEQUATE each |
| 72 | whereInSubquery | ADEQUATE |
| 73 | set op with WHERE on each side | ADEQUATE |
| 74 | whereInSubquery chained | ADEQUATE |
| 75 | existsSql in where | STRONG (composition) |

### Phase 4: UPDATE/DELETE (13 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 76-79 | UpdateQuery basic/multi SET/multi WHERE/orWhere | ADEQUATE each |
| 80 | UpdateQuery bubbles without WHERE | STRONG | CWE-284 safety guard |
| 81 | UpdateQuery bubbles without SET | STRONG | Empty-SET guard |
| 82 | UpdateQuery with limit | ADEQUATE | |
| 83 | UpdateQuery escaping | STRONG | Single-quote escaping in SET |
| 84-85 | DeleteQuery basic/multi WHERE | ADEQUATE each |
| 86 | DeleteQuery bubbles without WHERE | STRONG | CWE-284 safety guard |
| 87-88 | DeleteQuery orWhere/limit | ADEQUATE each |

### Phase 5: Extended Features (8 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 89-91 | orderByNulls FIRST/LAST/mixed | ADEQUATE each |
| 92-93 | crossJoin/combined | ADEQUATE each |
| 94-96 | lock FOR UPDATE/FOR SHARE/full | ADEQUATE each |

**Missing query tests:**
- limit(0) — is zero valid?
- limit(INT_MAX) — ORM-6 boundary
- Empty select([]) / selectExpr([])
- HAVING without GROUP BY
- Multiple distinct() / lock() calls — idempotency
- UpdateQuery.limit(-1) / DeleteQuery.limit(-1) — bubble paths
- Set operations with mismatched column counts
- Nested subqueries
- ORDER BY + LIMIT + OFFSET + lock combined
- Query builder immutability (derive two from same base)

---

## File: changeset_test.temper.md (66 tests)

### Cast/Required/Length (8 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 1 | cast whitelists | STRONG | Core whitelist logic |
| 2 | cast is replacing | STRONG | Critical semantic — catches double-cast bug |
| 3 | cast ignores empty strings | STRONG | |
| 4-5 | validateRequired pass/fail | STRONG each | |
| 6-8 | validateLength pass/short/long | STRONG each | Missing: exact min, exact max |

### Type Validators (8 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 9-10 | validateInt pass/fail | ADEQUATE each | Missing: boundary values |
| 11 | validateFloat pass | WEAK | Only one valid case, no invalid/edge |
| 12-13 | validateInt64 pass/fail | ADEQUATE each | Missing: boundaries |
| 14-16 | validateBool true/false/ambiguous | STRONG each | Thorough (13 values total) |

### SQL Generation (7 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 17 | toInsertSql Bobby Tables | STRONG | Security-critical |
| 18-19 | toInsertSql string/int field | ADEQUATE each | Uses indexOf, weaker than exact match |
| 20 | toInsertSql bubbles invalid | STRONG | Error path |
| 21 | toInsertSql non-nullable enforcement | STRONG | Security-critical defense-in-depth |
| 22-23 | toUpdateSql correct/bubbles | ADEQUATE/STRONG | |

### Data Manipulation (7 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 24-26 | putChange add/overwrite/in SQL | STRONG each |
| 27-28 | getChange found/missing | STRONG each |
| 29-30 | deleteChange remove/no-op | STRONG each |

### Inclusion/Exclusion Validators (6 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 31-33 | validateInclusion pass/fail/skip | STRONG each |
| 34-36 | validateExclusion pass/fail/skip | STRONG each |

### Number Validation (8 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 37-38 | greaterThan pass/fail | STRONG each | |
| 39-40 | lessThan pass/fail | STRONG each | |
| 41 | greaterThanOrEqual boundary | STRONG | Tests exact threshold (18 >= 18) |
| 42 | combined options | STRONG | |
| 43 | non-numeric | STRONG | |
| 44 | skips when absent | STRONG | |

### Other Validators (12 tests)

| # | Test Name | Rating | Notes |
|---|-----------|--------|-------|
| 45-46 | validateAcceptance pass/fail | ADEQUATE each | Missing: skip-when-absent |
| 47-49 | validateConfirmation match/differ/missing | STRONG each | |
| 50-52 | validateContains pass/fail/skip | STRONG each | |
| 53-54 | validateStartsWith pass/fail | ADEQUATE each | Missing: empty prefix, skip |
| 55-57 | validateEndsWith pass/fail/repeated suffix | STRONG each | |

### Schema Enrichment SQL (9 tests)

| # | Test Name | Rating |
|---|-----------|--------|
| 58-59 | toInsertSql default/override | STRONG each |
| 60 | timestamps use DEFAULT | STRONG |
| 61-62 | virtual field exclusion (insert/non-nullable) | STRONG each |
| 63 | toUpdateSql skips virtual | STRONG |
| 64 | toUpdateSql custom PK | STRONG |
| 65-66 | deleteSql custom/default PK | STRONG each |

**Missing changeset tests:**
- validateFloat invalid input (no test at all)
- validateFloat edge cases: NaN, Infinity, -0.0
- validateLength at exact min/max boundaries
- validateInt with boundary values (0, -1, INT_MAX, INT_MAX+1)
- validateNumber with equalTo option (untested code path)
- validateNumber with lessThanOrEqual option (untested code path)
- validateNumber at exact greaterThan threshold (18 > 18 should fail)
- validateNumber at exact lessThan threshold
- validateAcceptance skip-when-absent
- validateStartsWith skip-when-absent, empty prefix
- toInsertSql with all 6 FieldType variants
- toUpdateSql with multiple fields
- toUpdateSql with all virtual fields -> bubble
- deleteChange + toInsertSql interaction (non-nullable deleted)
- Already-invalid changeset early return (15 validators, none tested)
- Multiple validator chaining (3+ validators)
- Changeset immutability verification
- putChange satisfying validateRequired

---

## Cross-Cutting Analysis

### Missing Composition Tests

These verify components work correctly **together** — where real bugs live:

1. **Changeset + Query on same TableDef**: No test creates a changeset INSERT and then a SELECT on the same table to verify schema consistency.

2. **Query + UpdateQuery WHERE equivalence**: No test verifies that a SELECT WHERE and UPDATE WHERE produce identical SQL for the same conditions (read-modify-write pattern).

3. **toInsertSql with all 6 field types**: No single test exercises all type dispatch paths (`StringField`, `IntField`, `Int64Field`, `FloatField`, `BoolField`, `DateField`).

4. **putChange + toUpdateSql**: No test uses putChange then verifies the UPDATE SQL reflects the put value.

5. **deleteChange + toInsertSql**: What happens if you delete a non-nullable field? Should bubble but never tested.

6. **Multiple validator stacking**: No test chains 3+ validators (e.g. cast + validateRequired + validateLength + validateInt + validateNumber).

7. **Query builder immutability**: No test derives two different queries from the same base and verifies independence.

### Missing ORM Finding Regression Tests

| Finding | Has Regression Test? | Notes |
|---------|---------------------|-------|
| ORM-1: pair.key passed to appendSafe | Partial | Tests produce correct SQL but don't verify column names went through SafeIdentifier |
| ORM-2: SqlDate missing quote escaping | No | Current Date.toString() won't produce quotes, but the escaping code path is untested |
| ORM-3: SqlFloat64 NaN/Infinity | **Yes** | Well-tested |
| ORM-4: No parameterized queries | N/A | Design-level |
| ORM-5: SqlSource escape hatch | No | No test guards against misuse |
| ORM-6: No upper bound on limit | No | No test for limit(INT_MAX) |

### AND/OR Precedence Documentation Gap

The test "mixed where and orWhere" generates `WHERE a AND b OR c` without parentheses. In SQL, AND binds tighter than OR, so this means `(a AND b) OR c`, not `a AND (b OR c)`. The test confirms this behavior but **does not document whether it's intentional**. This could be a correctness issue for users who expect `a AND (b OR c)` semantics.

---

## Top 10 Priority Missing Tests

| Priority | Missing Test | Severity | Category |
|----------|-------------|----------|----------|
| 1 | Already-invalid changeset early return (15 validators) | HIGH | Error path |
| 2 | validateNumber with equalTo / lessThanOrEqual | HIGH | Untested code paths |
| 3 | toInsertSql with all 6 field types | CRITICAL | Type dispatch completeness |
| 4 | Query builder immutability | HIGH | Core safety property |
| 5 | limit(0) behavior | MEDIUM | Semantic boundary |
| 6 | validateInt boundary values (0, -1, MAX, overflow) | MEDIUM | Boundary |
| 7 | deleteChange + toInsertSql non-nullable | MEDIUM | Composition |
| 8 | UpdateQuery.limit(-1) / DeleteQuery.limit(-1) | MEDIUM | Error path |
| 9 | validateFloat invalid input | MEDIUM | Missing error path |
| 10 | Multiple validator chaining (3+) | MEDIUM | Composition |
