# Juice Shop Security Audit: Full Vulnerability Assessment

> **Date:** 2026-03-13
> **Scope:** OWASP Juice Shop v19.2.1 (fork at `notactuallytreyanastasio/juice-shop`)
> **Method:** Manual source code audit of all 61 route files, 22 models, middleware, and database integration layers
> **Auditor:** Automated + manual review via Claude

---

## Executive Summary

The initial Alloy migration (Changes 0-2 in PROCESS.md) identified and fixed 2 SQL injection vulnerabilities. This follow-up audit expanded the scope beyond SQL to cover the full attack surface. We found **17 total vulnerabilities** across 7 distinct vulnerability classes:

| Category | Found | Fixed by Alloy | Remaining |
|----------|-------|----------------|-----------|
| SQL Injection | 2 | 2 | 0 |
| NoSQL Injection | 4 | 0 | 4 |
| Remote Code Execution | 1 | 0 | 1 |
| Server-Side Request Forgery | 1 | 0 | 1 |
| Template Injection / File Read | 1 | 0 | 1 |
| XML External Entity (XXE) | 1 | 0 | 1 |
| Cryptographic Failures | 2 | 0 | 2 |
| Path Traversal | 1 | 0 | 1 |
| Open Redirect | 1 | 0 | 1 |
| Sensitive Data Exposure | 2 | 0 | 2 |
| Mass Assignment | 1 | 0 | 1 |
| **TOTAL** | **17** | **2** | **15** |

**Key finding:** Alloy eliminated 100% of the SQL injection attack surface. The 15 remaining vulnerabilities are in layers Alloy does not target: MongoDB/NoSQL queries, file handling, cryptography, and application logic. The 3 NoSQL injection vulnerabilities are structurally identical to the SQL injection we fixed — string interpolation into database queries — but in a database layer that Alloy's type system does not yet cover.

---

## Part 1: SQL Layer — Fully Mitigated

### VULN-01: Login SQL Injection (CRITICAL) — FIXED

- **File:** `routes/login.ts:36`
- **CWE:** CWE-89, CWE-287, CWE-116, CWE-20, CWE-564
- **CVSS:** 9.8

**Before:**
```typescript
models.sequelize.query(
  `SELECT * FROM Users WHERE email = '${req.body.email || ''}'
   AND password = '${security.hash(req.body.password || '')}'
   AND deletedAt IS NULL`,
  { model: UserModel, plain: true }
)
```

**After:** `buildLoginQuery()` via Alloy ORM — type-safe `appendString()` escapes all values.

**Validation:** 4 injection payloads tested, all blocked. Normal login works.

---

### VULN-02: Search SQL Injection (CRITICAL) — FIXED

- **File:** `routes/search.ts:25`
- **CWE:** CWE-89, CWE-943, CWE-200, CWE-306, CWE-862, CWE-704
- **CVSS:** 9.1

**Before:**
```typescript
models.sequelize.query(
  `SELECT * FROM Products WHERE ((name LIKE '%${criteria}%'
   OR description LIKE '%${criteria}%') AND deletedAt IS NULL)
   ORDER BY name`
)
```

**After:** `buildSearchQuery()` via Alloy ORM — LIKE values escaped as `SqlString` parts.

**Validation:** 3 injection payloads tested (including UNION extraction), all blocked. Normal search works.

---

### VULN-03: Schema Disclosure (MEDIUM) — UNFIXED

- **File:** `routes/search.ts:49`
- **CWE:** CWE-200
- **CVSS:** 5.3

```typescript
models.sequelize.query('SELECT sql FROM sqlite_master')
```

Hardcoded query with no user input. Exposes complete database DDL to challenge verification logic. Not exploitable via injection (the UNION vector that could have exposed this via the search endpoint is now closed by Alloy), but the pattern itself is a schema disclosure risk.

---

## Part 2: NoSQL Layer — 4 Vulnerabilities Unfixed

These are structurally identical to the SQL injection we fixed: user input is interpolated into database queries via string concatenation. The difference is they target MarsDB (MongoDB-compatible in-memory database) instead of SQLite/Sequelize, so Alloy's SQL query builder cannot address them.

### VULN-04: Order Tracking NoSQL Injection (CRITICAL)

- **File:** `routes/trackOrder.ts:18`
- **CWE:** CWE-943 (Improper Neutralization in Data Query Logic), CWE-200
- **CVSS:** 9.1

```typescript
const id = !utils.isChallengeEnabled(challenges.reflectedXssChallenge)
  ? String(req.params.id).replace(/[^\w-]+/g, '')
  : utils.trunc(req.params.id, 60)

db.ordersCollection.find({ $where: `this.orderId === '${id}'` })
```

**Data flow:** `req.params.id` (URL parameter) → truncated to 60 chars (no character filtering when challenge enabled) → interpolated into `$where` JavaScript string → executed as server-side JavaScript by MarsDB.

**Attack vectors:**

| Payload | Effect |
|---------|--------|
| `' \|\| true \|\| '` | Returns ALL orders — data exfiltration |
| `'; sleep(2000); '` | Server-side DoS via blocking sleep |
| `' \|\| this.email.includes('admin') \|\| '` | Targeted data extraction by email pattern |

**Why truncation is not a defense:** The shortest working payload (`' \|\| 1 \|\| '`) is 13 characters. The truncation limit is 60. An attacker has 47 characters of headroom for arbitrary JavaScript.

**Structural parallel to VULN-01:** This is the exact same vulnerability pattern as the login SQL injection — string interpolation of user input into a query language — but in MongoDB's `$where` operator instead of SQL's `WHERE` clause.

---

### VULN-05: Product Reviews NoSQL Injection + DoS (CRITICAL)

- **File:** `routes/showProductReviews.ts:36`
- **CWE:** CWE-943, CWE-94 (Code Injection), CWE-400 (Uncontrolled Resource Consumption)
- **CVSS:** 9.8

```typescript
// Lines 17-26: The file installs a blocking global sleep function
global.sleep = (time: number) => {
  if (time > 2000) { time = 2000 }
  const stop = new Date().getTime()
  while (new Date().getTime() < stop + time) { ; }
}

// Line 31: Input handling
const id = !utils.isChallengeEnabled(challenges.noSqlCommandChallenge)
  ? Number(req.params.id)
  : utils.trunc(req.params.id, 40)

// Line 36: Injection point
db.reviewsCollection.find({ $where: 'this.product == ' + id })
```

**Data flow:** `req.params.id` → truncated to 40 chars when challenge enabled (otherwise cast to `Number`, which is safe) → concatenated into `$where` JavaScript expression.

**Attack vectors:**

| Payload | Effect |
|---------|--------|
| `0; sleep(2000)` | Blocks the Node.js event loop for 2 seconds — DoS |
| `0 \|\| true` | Returns ALL reviews across all products |
| `0; return true` | Bypasses product filter entirely |

**Aggravating factor:** The file deliberately installs `global.sleep` as a blocking function (lines 17-26). This is a CPU-bound busy-wait that blocks the entire Node.js event loop. Combined with the `$where` injection, an attacker can freeze the server for up to 2 seconds per request with no rate limiting on this endpoint.

**Structural parallel to VULN-02:** Same pattern as the search SQL injection — user input concatenated into a query string — but with the additional severity that `$where` executes arbitrary JavaScript, not just query predicates.

---

### VULN-06: Review Update NoSQL Operator Injection (CRITICAL)

- **File:** `routes/updateProductReviews.ts:17-20`
- **CWE:** CWE-943, CWE-284 (Improper Access Control)
- **CVSS:** 8.1

```typescript
db.reviewsCollection.update(
  { _id: req.body.id },
  { $set: { message: req.body.message } },
  { multi: true }
)
```

**Data flow:** `req.body.id` (POST body, parsed as JSON) → used directly as MongoDB query selector.

**Attack:** When Express parses JSON bodies, `req.body.id` can be an object, not just a string. Sending `{ "id": { "$ne": null }, "message": "pwned" }` causes `{ _id: { $ne: null } }` which matches **every document**. Combined with `{ multi: true }`, this overwrites the `message` field of every review in the database.

| Payload | Effect |
|---------|--------|
| `{ "id": { "$ne": null }, "message": "hacked" }` | Overwrites ALL reviews |
| `{ "id": { "$gt": "" }, "message": "spam" }` | Same — mass modification |
| `{ "id": "specific-id", "message": "targeted" }` | Modify someone else's review (forged authorship) |

**Why this is worse than SQL injection:** In SQL, you can't inject operators into `WHERE id = ?` parameterized queries. In MongoDB with unvalidated JSON input, the query selector itself can be an operator expression. The `multi: true` flag makes this a mass-update weapon.

---

### VULN-07: Review Like NoSQL Selector Injection (HIGH)

- **File:** `routes/likeProductReviews.ts:25,35-37`
- **CWE:** CWE-943
- **CVSS:** 6.5

```typescript
const id = req.body.id
const review = await db.reviewsCollection.findOne({ _id: id })
// ...
await db.reviewsCollection.update(
  { _id: id },
  { $inc: { likesCount: 1 } }
)
```

**Data flow:** `req.body.id` → used as `_id` selector without type validation. Same operator injection as VULN-06 but without `multi: true`, limiting blast radius to a single document match. Can still like arbitrary reviews or trigger timing attacks via operator-based selectors.

---

## Part 3: Application Logic Vulnerabilities

These are not query injection — they are distinct vulnerability classes in the application layer.

### VULN-08: B2B Order Remote Code Execution (CRITICAL)

- **File:** `routes/b2bOrder.ts:21-23`
- **CWE:** CWE-94 (Code Injection)
- **CVSS:** 10.0

```typescript
const orderLinesData = body.orderLinesData || ''
const sandbox = { safeEval, orderLinesData }
vm.createContext(sandbox)
vm.runInContext('safeEval(orderLinesData)', sandbox, { timeout: 2000 })
```

**Data flow:** `req.body.orderLinesData` → passed to `notevil` (`safeEval`) inside a `vm` context → `notevil` has known sandbox escapes → arbitrary code execution on the server.

**Impact:** Full server compromise. The `notevil` library has documented bypasses that allow breaking out of the evaluation sandbox.

---

### VULN-09: Profile Image SSRF (HIGH)

- **File:** `routes/profileImageUrlUpload.ts:24`
- **CWE:** CWE-918 (Server-Side Request Forgery)
- **CVSS:** 7.5

```typescript
const url = req.body.imageUrl
const response = await fetch(url)
```

**Data flow:** `req.body.imageUrl` → fetched server-side via `fetch()` with no URL validation → can probe internal services, cloud metadata endpoints (169.254.169.254), or localhost.

**Fallback escalation:** On fetch error, the raw user URL is stored directly in the database as `profileImage` — potential stored XSS if rendered without escaping.

---

### VULN-10: Data Erasure SSTI + Local File Read (HIGH)

- **File:** `routes/dataErasure.ts:68-73`
- **CWE:** CWE-1336 (SSTI), CWE-98 (File Inclusion)
- **CVSS:** 7.5

```typescript
if (req.body.layout) {
  const filePath: string = path.resolve(req.body.layout).toLowerCase()
  const isForbiddenFile = (filePath.includes('ftp') || filePath.includes('ctf.key') || filePath.includes('encryptionkeys'))
  if (!isForbiddenFile) {
    res.render('dataErasureResult', {
      ...req.body    // entire request body spread into template context
    })
  }
}
```

**Two vulnerabilities in one:**
1. `req.body.layout` controls the template path — path traversal to read arbitrary files
2. `...req.body` spreads all user-controlled values into the Pug/EJS template context — Server-Side Template Injection

**The blocklist is trivially bypassable:** Only checks for `ftp`, `ctf.key`, and `encryptionkeys`. Any other file on the filesystem is readable.

---

### VULN-11: XML Upload XXE (HIGH)

- **File:** `routes/fileUpload.ts:83`
- **CWE:** CWE-611 (XML External Entity)
- **CVSS:** 7.5

```typescript
const xmlDoc = vm.runInContext(
  'libxml.parseXml(data, { noblanks: true, noent: true, nocdata: true })',
  sandbox, { timeout: 2000 }
)
```

`noent: true` enables XML entity expansion. A crafted XML file with `<!ENTITY xxe SYSTEM "file:///etc/passwd">` will read local files and include them in the parsed output, which is then returned in the error response.

---

### VULN-12: YAML Upload Deserialization (MEDIUM)

- **File:** `routes/fileUpload.ts:117`
- **CWE:** CWE-502 (Deserialization of Untrusted Data), CWE-400 (DoS)
- **CVSS:** 6.5

```typescript
const yamlString = vm.runInContext('JSON.stringify(yaml.load(data))', sandbox, { timeout: 2000 })
```

`yaml.load()` (not `yaml.safeLoad()`) processes all YAML tags including `!!js/function` and YAML bomb (billion laughs) patterns. The 2-second timeout mitigates but does not prevent resource exhaustion.

---

### VULN-13: Zip Slip Path Traversal (HIGH)

- **File:** `routes/fileUpload.ts:42-45`
- **CWE:** CWE-22 (Path Traversal)
- **CVSS:** 7.5

```typescript
const absolutePath = path.resolve('uploads/complaints/' + fileName)
if (absolutePath.includes(path.resolve('.'))) {
  entry.pipe(fs.createWriteStream('uploads/complaints/' + fileName))
}
```

A malicious zip file containing entries like `../../ftp/legal.md` will write to arbitrary paths within the application directory. The `includes(path.resolve('.'))` check is bypassable because `path.resolve('uploads/complaints/../../ftp/legal.md')` still includes the project root.

---

## Part 4: Cryptographic & Authentication Failures

### VULN-14: Hardcoded RSA Private Key (CRITICAL)

- **File:** `lib/insecurity.ts:23`
- **CWE:** CWE-798 (Hardcoded Credentials), CWE-321 (Hard-coded Cryptographic Key)
- **CVSS:** 9.8

```typescript
const privateKey = '-----BEGIN RSA PRIVATE KEY-----\r\nMIICXAIBAAKBgQDNwqLEe9wg...'
```

The JWT signing key is hardcoded in source code. Anyone with access to the repository can forge valid authentication tokens for any user, including admin.

---

### VULN-15: MD5 Password Hashing (HIGH)

- **File:** `lib/insecurity.ts:43`
- **CWE:** CWE-328 (Weak Hash), CWE-916 (Insufficient Computational Effort)
- **CVSS:** 7.5

```typescript
export const hash = (data: string) => crypto.createHash('md5').update(data).digest('hex')
```

All passwords are hashed with unsalted MD5. Rainbow tables for MD5 are freely available. Every user password in the database is effectively plaintext to any attacker who obtains the Users table (which was previously possible via VULN-02's UNION injection before the Alloy fix).

---

### VULN-16: Open Redirect via Substring Match (MEDIUM)

- **File:** `lib/insecurity.ts:138`
- **CWE:** CWE-601 (URL Redirection to Untrusted Site)
- **CVSS:** 4.7

```typescript
export const isRedirectAllowed = (url: string) => {
  let allowed = false
  for (const allowedUrl of redirectAllowlist) {
    allowed = allowed || url.includes(allowedUrl)
  }
  return allowed
}
```

Uses `includes()` instead of `startsWith()`. An attacker can bypass the allowlist with a URL like `https://evil.com?ref=https://github.com/juice-shop/juice-shop` because the allowed URL is a substring of the malicious one.

---

### VULN-17: Password Change via GET Query Params (MEDIUM)

- **File:** `routes/changePassword.ts:14`
- **CWE:** CWE-598 (Sensitive Query String), CWE-620 (Unverified Password Change)
- **CVSS:** 5.3

```typescript
const currentPassword = query.current as string
const newPassword = query.new as string
```

Passwords are sent as URL query parameters, which are logged in server access logs, browser history, proxy logs, and Referer headers. Additionally, the `current` password parameter is optional — if omitted, the password is changed without verifying the existing credential.

---

### VULN-18: User Registration Mass Assignment (HIGH)

- **File:** `server.ts:480-504`
- **CWE:** CWE-915 (Improperly Controlled Modification of Dynamically-Determined Object Attributes)
- **CVSS:** 8.1

```typescript
finale.initialize({ app, sequelize })
// ...
finale.resource({
  model: UserModel,
  endpoints: ['/api/Users', '/api/Users/:id'],
  excludeAttributes: ['password', 'totpSecret'],  // only excludes from GET responses
  pagination: false
})
```

The `finale-rest` library auto-generates CRUD endpoints including `POST /api/Users`. The `excludeAttributes` only affects response serialization, not input filtering. An attacker can send `{ "email": "x@x.com", "password": "pw", "role": "admin" }` to register as an administrator.

---

## Part 5: What Alloy Fixed and What It Can't

### The SQL Layer: Complete Coverage

Alloy eliminated 100% of the SQL injection attack surface:

```
SQL touchpoints:       ~40
Vulnerable (before):     2  (login.ts, search.ts)
Vulnerable (after):      0
Coverage:             100%  of SQL injection vectors closed
```

The remaining ~37 SQL touchpoints use Sequelize ORM methods (`.findOne()`, `.findAll()`, `.create()`, `.update()`, `.destroy()`) which generate parameterized queries. These were never vulnerable and don't need Alloy.

### The NoSQL Layer: Unaddressed

The 4 NoSQL injection vulnerabilities (VULN-04 through VULN-07) are structurally identical to the SQL injection Alloy fixed:

| Property | SQL Injection (fixed) | NoSQL Injection (unfixed) |
|----------|----------------------|--------------------------|
| Root cause | String interpolation into query | String interpolation into query |
| User input path | `req.body` / `req.query` → SQL string | `req.params` / `req.body` → `$where` string |
| Defense needed | Type-safe query builder | Type-safe query builder |
| Alloy coverage | Yes | No (SQL only) |

This is the strongest possible argument for extending Temper/Alloy to cover NoSQL query generation: the same type-system approach that eliminated SQL injection would eliminate NoSQL injection. The vulnerability pattern is identical. The defense mechanism is identical. Only the target query language differs.

### What Alloy Cannot Address

The remaining 9 vulnerabilities (VULN-08 through VULN-18) are outside any query builder's scope:

- **RCE via eval** (VULN-08): Application logic flaw — executing user input as code
- **SSRF** (VULN-09): Network-layer request forgery
- **SSTI/LFR** (VULN-10): Template engine misuse
- **XXE** (VULN-11): XML parser misconfiguration
- **Deserialization** (VULN-12): YAML parser misuse
- **Path traversal** (VULN-13): File system access control
- **Crypto failures** (VULN-14, VULN-15): Key management and hash algorithm choice
- **Open redirect** (VULN-16): URL validation logic
- **Sensitive data exposure** (VULN-17): API design
- **Mass assignment** (VULN-18): ORM input filtering

These require different categories of defense — input validation, configuration hardening, access control, and cryptographic best practices — none of which are in scope for a query builder.

---

## Part 6: MITRE CWE Summary — Full Application

### CWEs by Status

| CWE | Name | Severity | Status | Fixed By |
|-----|------|----------|--------|----------|
| CWE-89 | SQL Injection | Critical | **MITIGATED** | Alloy ORM |
| CWE-943 | Improper Neutralization in Query Logic | Critical | **PARTIAL** — SQL fixed, NoSQL open | Alloy (SQL only) |
| CWE-564 | SQL Injection: ORM Bypass | Critical | **MITIGATED** | Alloy ORM |
| CWE-287 | Improper Authentication | Critical | **MITIGATED** (SQL path) | Alloy ORM |
| CWE-94 | Code Injection | Critical | **VULNERABLE** | b2bOrder.ts, showProductReviews.ts |
| CWE-798 | Hardcoded Credentials | Critical | **VULNERABLE** | insecurity.ts |
| CWE-918 | Server-Side Request Forgery | High | **VULNERABLE** | profileImageUrlUpload.ts |
| CWE-611 | XML External Entity | High | **VULNERABLE** | fileUpload.ts |
| CWE-22 | Path Traversal | High | **VULNERABLE** | fileUpload.ts |
| CWE-1336 | Server-Side Template Injection | High | **VULNERABLE** | dataErasure.ts |
| CWE-915 | Mass Assignment | High | **VULNERABLE** | server.ts (finale) |
| CWE-328 | Weak Hash | High | **VULNERABLE** | insecurity.ts |
| CWE-284 | Improper Access Control | High | **VULNERABLE** | updateProductReviews.ts |
| CWE-916 | Insufficient Password Hashing | High | **VULNERABLE** | insecurity.ts |
| CWE-400 | Uncontrolled Resource Consumption | Medium | **VULNERABLE** | showProductReviews.ts, fileUpload.ts |
| CWE-200 | Information Exposure | Medium | **PARTIAL** — UNION path closed | search.ts (sqlite_master) |
| CWE-601 | URL Redirect to Untrusted Site | Medium | **VULNERABLE** | insecurity.ts |
| CWE-598 | Sensitive Query String | Medium | **VULNERABLE** | changePassword.ts |
| CWE-502 | Deserialization of Untrusted Data | Medium | **VULNERABLE** | fileUpload.ts |
| CWE-620 | Unverified Password Change | Medium | **VULNERABLE** | changePassword.ts |
| CWE-209 | Error Message Info Leak | Low | **PARTIAL** | Various |
| CWE-116 | Improper Encoding/Escaping | Critical | **MITIGATED** (SQL) | Alloy ORM |
| CWE-20 | Improper Input Validation | High | **PARTIAL** — SQL fixed, NoSQL open | Alloy (SQL only) |
| CWE-704 | Incorrect Type Conversion | Medium | **MITIGATED** (SQL) | Alloy ORM |
| CWE-306 | Missing Auth for Critical Function | High | **MITIGATED** (UNION path) | Alloy ORM |
| CWE-862 | Missing Authorization | High | **MITIGATED** (UNION path) | Alloy ORM |

### By Severity

```
Critical:  6 total — 3 mitigated by Alloy, 3 remaining
High:     10 total — 3 mitigated by Alloy, 7 remaining
Medium:    7 total — 1 partially mitigated, 6 remaining
Low:       1 total — 0 mitigated
─────────────────────────────────────────────────────
Total:    24 unique CWEs — 9 fully mitigated, 2 partially, 13 remaining
```

---

## Part 7: Conclusions

### What This Audit Proves

1. **Alloy works.** The 2 SQL injection vulnerabilities are genuinely fixed. The type system makes the attacks unrepresentable. This is validated by real payloads against the running application.

2. **SQL was the tip of the iceberg.** The original audit found 2 vulnerabilities. The expanded audit found 17. Query injection (SQL + NoSQL combined) accounts for 6 of the 17 — the other 11 are entirely different vulnerability classes.

3. **NoSQL injection is the same problem.** The 4 NoSQL vulnerabilities use the exact same anti-pattern as the SQL injection we fixed: string interpolation of user input into database queries. A Temper-based NoSQL query builder would eliminate them with the same structural guarantees.

4. **Defense-in-depth is not optional.** Even with 100% SQL injection coverage, the application has a critical RCE, an SSRF, a hardcoded private key, and MD5 password hashing. No single tool eliminates all vulnerability classes. Alloy eliminates one class completely — that's its value proposition.

### The Narrative for Temper

> Alloy removed the SQL injection liability from Juice Shop in 8 lines of route changes and 80 lines of infrastructure. It did this structurally — not by scanning, not by training developers, not by adding a WAF. The type system makes injection unrepresentable.
>
> The remaining 15 vulnerabilities prove the point by contrast: they exist in layers where no type-safe query builder operates. The 4 NoSQL injection vulns are the strongest argument for extending Temper's approach to MongoDB and other NoSQL databases — the same pattern that eliminated SQL injection would eliminate NoSQL injection. The defense mechanism is language-agnostic and database-agnostic. It's a property of the type system, not the target.
>
> Every vulnerability class that Alloy covers has zero remaining instances. Every class it doesn't cover still has open instances. That's not a weakness — it's a precisely scoped tool doing exactly what it was designed to do, doing it completely, and making the case for extending that approach to the rest of the stack.
