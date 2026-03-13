# Juice Shop x Alloy ORM: SQL Security Migration Plan

## Executive Summary

OWASP Juice Shop is an intentionally vulnerable web application used for security training. Its database layer uses Sequelize ORM (v6) with SQLite, but **deliberately introduces raw SQL injection vulnerabilities** in key routes. This report:

1. Catalogs every SQL touchpoint in the application
2. Performs a MITRE CWE analysis on the current SQL attack surface
3. Proposes replacing the raw SQL query layer with Alloy (Temper-compiled, type-safe ORM)
4. Analyzes what vulnerabilities Alloy structurally eliminates

---

## Part 1: Current State — SQL Vulnerability Audit

### 1.1 Database Architecture

| Component | Technology | Purpose |
|-----------|-----------|---------|
| SQL Database | SQLite via Sequelize v6.37.3 | Primary data store (22 models) |
| NoSQL Database | MarsDB (in-memory) | Reviews & orders (2 collections) |
| Connection | `sqlite3` v5.1.7 | SQLite driver |

### 1.2 SQL Injection Vulnerabilities Found

#### VULN-01: Login Authentication Bypass (CRITICAL)

- **File**: `routes/login.ts:34`
- **CWE**: CWE-89 (SQL Injection)
- **CVSS**: 9.8 (Critical)

```typescript
// CURRENT VULNERABLE CODE
models.sequelize.query(
  `SELECT * FROM Users WHERE email = '${req.body.email || ''}'
   AND password = '${security.hash(req.body.password || '')}'
   AND deletedAt IS NULL`,
  { model: UserModel, plain: true }
)
```

**Attack vectors:**
- `admin' OR '1'='1' --` in email field bypasses authentication entirely
- `' UNION SELECT 1,2,3,4,5,6,7,8,9,10,11,12 --` extracts arbitrary data
- `admin@juice-sh.op'--` comments out password check

**User input flow:** `req.body.email` → string interpolation → raw SQL → `sequelize.query()`

---

#### VULN-02: Product Search UNION Injection (CRITICAL)

- **File**: `routes/search.ts:23`
- **CWE**: CWE-89 (SQL Injection)
- **CVSS**: 9.1 (Critical)

```typescript
// CURRENT VULNERABLE CODE
models.sequelize.query(
  `SELECT * FROM Products WHERE ((name LIKE '%${criteria}%'
   OR description LIKE '%${criteria}%') AND deletedAt IS NULL)
   ORDER BY name`
)
```

**Attack vectors:**
- `%')) UNION SELECT id,email,password,4,5,6,7,8,9 FROM Users--` exfiltrates user credentials
- `%')) UNION SELECT sql,2,3,4,5,6,7,8,9 FROM sqlite_master--` dumps database schema
- `%' OR 1=1--` returns all products including soft-deleted

**User input flow:** `req.query.q` → `criteria` variable → string interpolation → raw SQL

---

#### VULN-03: Schema Information Disclosure (MEDIUM)

- **File**: `routes/search.ts:47`
- **CWE**: CWE-200 (Information Exposure)
- **CVSS**: 5.3 (Medium)

```typescript
// Hardcoded but exposes schema
models.sequelize.query('SELECT sql FROM sqlite_master')
```

**Impact:** Exposes complete database schema definitions to challenge verification logic. While not directly exploitable via user input, the query result is used in challenge detection and the pattern encourages schema exposure.

---

### 1.3 Safe SQL Patterns (Current)

The remaining ~35+ route handlers use Sequelize's parameterized ORM methods correctly:

| Pattern | Example | Files Using It |
|---------|---------|---------------|
| `Model.findByPk(id)` | `UserModel.findByPk(userId)` | changePassword, resetPassword, updateUserProfile, 2fa |
| `Model.findOne({ where })` | `BasketModel.findOne({ where: { id } })` | order, basket, wallet, payment, securityQuestion |
| `Model.findAll({ where })` | `CardModel.findAll({ where: { UserId } })` | payment, address, memory, authenticatedUsers |
| `Model.create({})` | `PrivacyRequestModel.create({})` | dataErasure, memory |
| `Model.update({}, { where })` | `user.update({ password })` | changePassword, deluxe |
| `Model.destroy({ where })` | `CardModel.destroy({ where: { id } })` | payment, address |
| `instance.save()` | `basketItem.save()` | basketItems, captcha |
| `Model.findOrCreate()` | `BasketModel.findOrCreate()` | login, 2fa |
| `Model.increment/decrement()` | `WalletModel.increment('balance')` | wallet, order |

These are **not vulnerable** — Sequelize generates parameterized queries for all ORM method calls.

---

## Part 2: MITRE CWE Analysis — Current State

### CWE Mapping

| CWE ID | Name | Severity | Files Affected | Status |
|--------|------|----------|---------------|--------|
| **CWE-89** | SQL Injection | Critical | `login.ts:34`, `search.ts:23` | **VULNERABLE** |
| **CWE-943** | Improper Neutralization of Special Elements in Data Query Logic | Critical | `login.ts:34`, `search.ts:23` | **VULNERABLE** |
| **CWE-564** | SQL Injection: Hibernate (ORM Bypass) | Critical | `login.ts:34`, `search.ts:23` | **VULNERABLE** — raw queries bypass ORM |
| **CWE-200** | Exposure of Sensitive Information | Medium | `search.ts:47` | **VULNERABLE** — schema disclosure |
| **CWE-287** | Improper Authentication | Critical | `login.ts:34` | **VULNERABLE** — auth bypass via SQLi |
| **CWE-306** | Missing Authentication for Critical Function | High | Via UNION injection | **VULNERABLE** — data exfiltration |
| **CWE-862** | Missing Authorization | High | Via UNION injection | **VULNERABLE** — cross-user data access |
| **CWE-209** | Generation of Error Message Containing Sensitive Information | Medium | SQL error messages | **PARTIAL** — Sequelize may leak schema info |
| **CWE-20** | Improper Input Validation | High | `login.ts`, `search.ts` | **VULNERABLE** — no input sanitization |
| **CWE-116** | Improper Encoding or Escaping of Output | Critical | `login.ts:34`, `search.ts:23` | **VULNERABLE** — no escaping in interpolation |
| **CWE-704** | Incorrect Type Conversion | Medium | `search.ts:23` | **VULNERABLE** — string used directly in SQL |

### Attack Surface Summary

```
Total SQL query touchpoints:  ~40+
Raw SQL queries (vulnerable):   2  (login.ts, search.ts)
Schema disclosure queries:      1  (search.ts)
Safe ORM queries:             ~37+ (parameterized via Sequelize)
Vulnerability rate:            7.5% of SQL touchpoints
```

### MITRE ATT&CK Techniques Enabled

| Technique | ID | Enabled By |
|-----------|----|-----------|
| Exploit Public-Facing Application | T1190 | VULN-01, VULN-02 |
| Valid Accounts (via SQLi auth bypass) | T1078 | VULN-01 |
| Data from Information Repositories | T1213 | VULN-02 (UNION extraction) |
| Account Discovery | T1087 | VULN-02 (Users table dump) |
| Credential Access | T1555 | VULN-02 (password hash extraction) |

---

## Part 3: Alloy ORM Replacement Plan

### 3.1 Integration Architecture

```
Current:
  req.body.email → string interpolation → raw SQL string → sequelize.query()

Proposed:
  req.body.email → SqlBuilder.appendString() → SqlFragment → .toSql().toString()
                                                                    ↓
                                              sequelize.query(safeSQL, { type: SELECT })
```

**Key insight:** Alloy builds complete, escaped SQL strings — not placeholder-based queries. Values are embedded directly but **safely escaped** (single quotes doubled to `''`). This is safe for SQLite.

### 3.2 Vendor Setup

```
juice-shop/
├── vendor/
│   ├── orm/          ← from temper.out/js/orm/
│   ├── std/          ← from temper.out/js/std/
│   └── temper-core/  ← from temper.out/js/temper-core/
├── package.json      ← add file: dependencies
└── lib/
    └── alloy.ts      ← wrapper module for Alloy ORM imports
```

**package.json additions:**
```json
{
  "dependencies": {
    "orm": "file:./vendor/orm",
    "@temperlang/core": "file:./vendor/temper-core",
    "@temperlang/std": "file:./vendor/std"
  }
}
```

### 3.3 Alloy Wrapper Module (`lib/alloy.ts`)

```typescript
// lib/alloy.ts — Alloy ORM bridge for Juice Shop
import {
  from, update, deleteFrom, safeIdentifier, changeset,
  col, countAll, countCol, sumCol, avgCol,
  SqlBuilder, SqlFragment, SqlString, SqlInt32, SqlBoolean,
  SafeIdentifier, TableDef, FieldDef, StringField, IntField,
  BoolField, FloatField, DateField, NullsFirst, NullsLast,
  ForUpdate, ForShare
} from 'orm/src'

// Table definitions matching Juice Shop's Sequelize models
const Users = new TableDef(safeIdentifier('Users'), [
  new FieldDef(safeIdentifier('id'),         IntField(),    false, null, false),
  new FieldDef(safeIdentifier('username'),   StringField(), true,  null, false),
  new FieldDef(safeIdentifier('email'),      StringField(), false, null, false),
  new FieldDef(safeIdentifier('password'),   StringField(), false, null, false),
  new FieldDef(safeIdentifier('role'),       StringField(), false, null, false),
  new FieldDef(safeIdentifier('deluxeToken'),StringField(), true,  null, false),
  new FieldDef(safeIdentifier('lastLoginIp'),StringField(), true,  null, false),
  new FieldDef(safeIdentifier('profileImage'),StringField(),true,  null, false),
  new FieldDef(safeIdentifier('totpSecret'), StringField(), true,  null, false),
  new FieldDef(safeIdentifier('isActive'),   BoolField(),   false, null, false),
  new FieldDef(safeIdentifier('deletedAt'),  StringField(), true,  null, false),
])

const Products = new TableDef(safeIdentifier('Products'), [
  new FieldDef(safeIdentifier('id'),          IntField(),    false, null, false),
  new FieldDef(safeIdentifier('name'),        StringField(), false, null, false),
  new FieldDef(safeIdentifier('description'), StringField(), true,  null, false),
  new FieldDef(safeIdentifier('price'),       FloatField(),  false, null, false),
  new FieldDef(safeIdentifier('deluxePrice'), FloatField(),  true,  null, false),
  new FieldDef(safeIdentifier('image'),       StringField(), true,  null, false),
  new FieldDef(safeIdentifier('deletedAt'),   StringField(), true,  null, false),
])

export { Users, Products, from, update, deleteFrom, safeIdentifier,
         changeset, col, SqlBuilder, SqlFragment, SqlString, SqlInt32 }
```

### 3.4 File-by-File Migration Plan

#### Migration 1: `routes/login.ts` (CRITICAL — VULN-01)

**Current (vulnerable):**
```typescript
models.sequelize.query(
  `SELECT * FROM Users WHERE email = '${req.body.email || ''}'
   AND password = '${security.hash(req.body.password || '')}'
   AND deletedAt IS NULL`,
  { model: UserModel, plain: true }
)
```

**Proposed (safe via Alloy):**
```typescript
import { from, safeIdentifier, SqlBuilder } from '../lib/alloy'

// Build query with type-safe escaping
const emailCond = new SqlBuilder()
emailCond.appendSafe('email = ')
emailCond.appendString(req.body.email || '')

const passCond = new SqlBuilder()
passCond.appendSafe('password = ')
passCond.appendString(security.hash(req.body.password || ''))

const query = from(safeIdentifier('Users'))
  .where(emailCond.accumulated)
  .where(passCond.accumulated)
  .whereNull(safeIdentifier('deletedAt'))

const sql = query.toSql().toString()
const [user] = await models.sequelize.query(sql, {
  model: UserModel, plain: true
})
```

**What Alloy prevents:**
- `admin' OR '1'='1'--` → becomes `email = 'admin'' OR ''1''=''1''--'` (escaped, no injection)
- `' UNION SELECT...` → single quotes escaped, UNION treated as literal string data
- **CWE-89, CWE-287, CWE-116 eliminated**

---

#### Migration 2: `routes/search.ts` (CRITICAL — VULN-02)

**Current (vulnerable):**
```typescript
models.sequelize.query(
  `SELECT * FROM Products WHERE ((name LIKE '%${criteria}%'
   OR description LIKE '%${criteria}%') AND deletedAt IS NULL)
   ORDER BY name`
)
```

**Proposed (safe via Alloy):**
```typescript
import { from, safeIdentifier } from '../lib/alloy'

const query = from(safeIdentifier('Products'))
  .whereLike(safeIdentifier('name'), `%${criteria}%`)
  .whereNull(safeIdentifier('deletedAt'))
  .orderBy(safeIdentifier('name'), true)

// Add description OR condition
const descBuilder = new SqlBuilder()
descBuilder.appendSafe('description LIKE ')
descBuilder.appendString(`%${criteria}%`)
const fullQuery = query.orWhere(descBuilder.accumulated)

const sql = fullQuery.toSql().toString()
const results = await models.sequelize.query(sql)
```

**What Alloy prevents:**
- `%')) UNION SELECT...` → `%'' )) UNION SELECT...` inside a quoted string (no breakout)
- `' OR 1=1--` → escaped, treated as literal search text
- **CWE-89, CWE-943, CWE-306 eliminated**

---

#### Migration 3: `routes/search.ts:47` (MEDIUM — VULN-03)

**Current:**
```typescript
models.sequelize.query('SELECT sql FROM sqlite_master')
```

**Proposed:** Remove entirely or replace with application-level challenge detection that doesn't query sqlite_master. This is a hardcoded query (no injection vector) but exposes schema metadata.

---

### 3.5 Routes NOT Requiring Migration

These routes already use safe Sequelize ORM methods and do **not** need Alloy replacement:

| Route File | Operations | Why Safe |
|-----------|-----------|---------|
| `order.ts` | findOne, update, destroy, increment | Parameterized via Sequelize |
| `basket.ts` | findOne with include | Parameterized |
| `basketItems.ts` | build, save, findOne | Parameterized |
| `payment.ts` | findAll, findOne, destroy | Parameterized |
| `address.ts` | findAll, findOne, destroy | Parameterized |
| `delivery.ts` | findAll, findOne | Parameterized |
| `wallet.ts` | findOne, increment | Parameterized |
| `changePassword.ts` | findByPk, update | Parameterized |
| `resetPassword.ts` | findOne, findByPk, update | Parameterized |
| `updateUserProfile.ts` | findByPk, update | Parameterized |
| `securityQuestion.ts` | findOne, findByPk | Parameterized |
| `dataErasure.ts` | findOne, findByPk, create | Parameterized |
| `dataExport.ts` | findAll | Parameterized |
| `coupon.ts` | findByPk, update | Parameterized |
| `deluxe.ts` | findOne, decrement, update | Parameterized |
| `memory.ts` | create, findAll | Parameterized |
| `captcha.ts` | build, save, findOne | Parameterized |
| `imageCaptcha.ts` | build, save, findAll | Parameterized |
| `recycles.ts` | findAll | Parameterized |
| `continueCode.ts` | findAll | Parameterized |
| `authenticatedUsers.ts` | findAll | Parameterized |
| `2fa.ts` | findByPk, findOrCreate | Parameterized |

---

## Part 4: MITRE CWE Analysis — Post-Migration (with Alloy)

### CWE Status After Alloy Integration

| CWE ID | Name | Before | After | Delta |
|--------|------|--------|-------|-------|
| **CWE-89** | SQL Injection | VULNERABLE | **MITIGATED** | Alloy's SqlBuilder escapes all string values; SafeIdentifier validates identifiers |
| **CWE-943** | Improper Neutralization in Query Logic | VULNERABLE | **MITIGATED** | Query structure is built via API, not string concatenation |
| **CWE-564** | SQL Injection: ORM Bypass | VULNERABLE | **MITIGATED** | No raw SQL bypass — all queries go through Alloy's typed builder |
| **CWE-200** | Information Exposure | VULNERABLE | **MITIGATED** | sqlite_master query removed; schema not queryable via Alloy |
| **CWE-287** | Improper Authentication | VULNERABLE | **MITIGATED** | Login query cannot be short-circuited; `OR '1'='1'` is escaped |
| **CWE-306** | Missing Auth for Critical Function | VULNERABLE | **MITIGATED** | UNION injection impossible; data exfiltration blocked |
| **CWE-862** | Missing Authorization | VULNERABLE | **MITIGATED** | Cannot inject cross-table queries |
| **CWE-209** | Error Message Info Leak | PARTIAL | **PARTIAL** | Alloy doesn't address error handling (Sequelize concern) |
| **CWE-20** | Improper Input Validation | VULNERABLE | **MITIGATED** | Alloy type system enforces value types at query build time |
| **CWE-116** | Improper Encoding/Escaping | VULNERABLE | **MITIGATED** | All values go through typed `append*()` methods with escaping |
| **CWE-704** | Incorrect Type Conversion | VULNERABLE | **MITIGATED** | SqlInt32, SqlString, SqlBoolean enforce correct types |

### Structural Defenses Alloy Provides

| Defense Layer | Mechanism | CWEs Addressed |
|--------------|-----------|---------------|
| **1. SafeIdentifier** | Validates table/column names match `[a-zA-Z_][a-zA-Z0-9_]*` — rejects `"users; DROP TABLE"` | CWE-89, CWE-943 |
| **2. SqlPart Type Hierarchy** | Sealed type system: SqlString, SqlInt32, SqlBoolean, etc. — each renders with correct escaping | CWE-89, CWE-116, CWE-704 |
| **3. SqlBuilder Separation** | `appendSafe()` for SQL keywords only; `appendString()` for values — structural separation of code from data | CWE-89, CWE-943 |
| **4. Changeset Pipeline** | Field whitelisting via `cast()`, type validation before SQL generation | CWE-20, CWE-89 |
| **5. No String Interpolation** | Query API has no method that accepts raw SQL with embedded values | CWE-89, CWE-564 |

### Attack Surface Comparison

```
                        BEFORE (Current)    AFTER (Alloy)
                        ─────────────────   ──────────────
Raw SQL queries:              3                  0
String interpolation:         2                  0
Parameterized (ORM):        ~37                ~37 (unchanged)
Alloy-built queries:          0                  2-3
Schema disclosure:            1                  0
                        ─────────────────   ──────────────
SQL injection vectors:        2                  0
CWEs vulnerable:             11                  1 (CWE-209 partial)
MITRE ATT&CK enabled:        5                  0
```

---

## Part 5: Implementation Phases

### Documentation Protocol: PROCESS.md

Every change is documented in real-time in `docs/PROCESS.md`. This is not optional — it is the primary deliverable alongside the code changes. The document serves as:

1. **Live audit trail** — each change is logged as it happens, not retroactively
2. **Before/after evidence** — vulnerable code shown alongside the Alloy replacement
3. **CWE-level proof** — each fix explicitly maps to the CWEs it eliminates and WHY
4. **Pitch deck source material** — refined into a final narrative showing that building with Temper removes liability incrementally, regardless of pace or workflow

**PROCESS.md structure per change:**
```
## Change N: [Title]
### The Problem
  - Vulnerable code (exact file:line)
  - What an attacker can do (concrete payloads)
  - CWEs triggered
### The Fix
  - New code using Alloy
  - What changed structurally
### Why Temper Absolves This Completely
  - Which Alloy defense layer prevents the attack
  - Proof: the payload rendered through Alloy's escaping
  - Why this isn't just "a better library" but a structural guarantee
### Liability Removed
  - What compliance/audit boxes this checks
  - What this means for the business
```

At the end of implementation, PROCESS.md is refined into a final report: **"How Temper Eliminates SQL Liability By Construction"** — the thesis being that Temper's compile-time type safety makes SQL injection structurally impossible, not just unlikely. You don't audit for it. You don't train against it. The type system won't let you write it.

---

### Phase 1: Vendor & Wire (Day 1)
- [ ] Copy `temper.out/js/{orm,std,temper-core}` → `juice-shop/vendor/`
- [ ] Add `file:` dependencies to `package.json`
- [ ] Create `lib/alloy.ts` wrapper with TableDef definitions for Users and Products
- [ ] Verify Alloy imports work in Juice Shop's TypeScript build
- [ ] **Document in PROCESS.md:** Baseline state, what we're replacing and why

### Phase 2: Migrate Login (Day 2)
- [ ] Replace `routes/login.ts:34` raw SQL with Alloy query builder
- [ ] Verify login still works with valid credentials
- [ ] Verify SQL injection payloads are now escaped/neutralized
- [ ] Run existing Juice Shop test suite
- [ ] **Document in PROCESS.md:** Login vulnerability, exact fix, escaping proof, CWE closure

### Phase 3: Migrate Search (Day 2-3)
- [ ] Replace `routes/search.ts:23` raw SQL with Alloy `.whereLike()` + `.orWhere()`
- [ ] Handle the OR condition for name + description LIKE search
- [ ] Remove or refactor `search.ts:47` sqlite_master query
- [ ] Verify search functionality works correctly
- [ ] Verify UNION injection payloads are neutralized
- [ ] **Document in PROCESS.md:** Search vulnerability, UNION/OR bypass proof, schema disclosure removal

### Phase 4: Security Validation (Day 3)
- [ ] Run OWASP ZAP or sqlmap against login endpoint — confirm injection blocked
- [ ] Run sqlmap against search endpoint — confirm injection blocked
- [ ] Document before/after comparison with specific payloads
- [ ] Generate final MITRE CWE diff report
- [ ] **Document in PROCESS.md:** Validation results, tool output, before/after diff

### Phase 5: Report & Demo (Day 4)
- [ ] Side-by-side demo: original Juice Shop (vulnerable) vs Alloy-hardened (safe)
- [ ] Both running simultaneously on different ports
- [ ] Live SQL injection attempts showing blocked vs successful
- [ ] Final security report with all 6 language demos running
- [ ] **Refine PROCESS.md into final report:** "How Temper Eliminates SQL Liability By Construction"

---

## Appendix A: Alloy SQL Escaping Proof

### Test: Bobby Tables Attack

```javascript
// Input: Robert'); DROP TABLE Users;--
const builder = new SqlBuilder()
builder.appendSafe("email = ")
builder.appendString("Robert'); DROP TABLE Users;--")
builder.accumulated.toString()
// Output: "email = 'Robert''); DROP TABLE Users;--'"
//                   ^^
//          Quote escaped — entire input treated as string literal
```

### Test: UNION Injection

```javascript
// Input: %' UNION SELECT password FROM Users--
const query = from(safeIdentifier("Products"))
  .whereLike(safeIdentifier("name"), "%' UNION SELECT password FROM Users--")
query.toSql().toString()
// Output: "SELECT * FROM Products WHERE name LIKE '%'' UNION SELECT password FROM Users--'"
//                                                  ^^
//           Quote escaped — UNION is inside the string literal, not SQL structure
```

### Test: OR-Based Auth Bypass

```javascript
// Input: admin' OR '1'='1
const builder = new SqlBuilder()
builder.appendSafe("email = ")
builder.appendString("admin' OR '1'='1")
builder.accumulated.toString()
// Output: "email = 'admin'' OR ''1''=''1'"
//                   ^^     ^^  ^^  ^^
//           All quotes escaped — boolean logic is inside the string literal
```

---

## Appendix B: Files Modified Summary

| File | Change | Risk |
|------|--------|------|
| `package.json` | Add vendor dependencies | Low |
| `lib/alloy.ts` | New file — Alloy wrapper | Low |
| `routes/login.ts` | Replace raw SQL with Alloy builder | **High** (auth path) |
| `routes/search.ts` | Replace raw SQL with Alloy builder | Medium |
| `vendor/` | New directory — Alloy compiled output | Low |

**Total files modified:** 4 (+ 1 new + vendor directory)
**Total lines changed:** ~50-70 estimated
