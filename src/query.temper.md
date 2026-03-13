# Query

Composable, immutable SELECT query builder.

## Security model

- `tableName`, `selectedFields`, and `orderBy` fields all require
  `SafeIdentifier` — validated against `[a-zA-Z_][a-zA-Z0-9_]*` before
  they can be passed here. There is no path for a raw user string to reach
  `appendSafe` except through a `SafeIdentifier`.
- `appendSafe` is only ever called with: (a) hardcoded SQL keyword string
  literals, or (b) `safeIdentifier.sqlValue`. Never with runtime arbitrary
  strings.
- User values only enter SQL via `SqlFragment` conditions already built
  with the `sql` tag. The `where()` method takes a `SqlFragment`, not a
  raw `String`.
- `safeToSql(defaultLimit)` is provided as the production-safe variant
  that always applies an upper bound on result set size (CWE-400).

## Imports

All types (SqlFragment, SqlBuilder, sql, SafeIdentifier) are available
from other files in the same module without explicit imports.

## JoinType

    export sealed interface JoinType {
      public keyword(): String;
    }

    export class InnerJoin() extends JoinType {
      // keyword
      public keyword(): String { "INNER JOIN" }
    }

    export class LeftJoin() extends JoinType {
      // keyword
      public keyword(): String { "LEFT JOIN" }
    }

    export class RightJoin() extends JoinType {
      // keyword
      public keyword(): String { "RIGHT JOIN" }
    }

    export class FullJoin() extends JoinType {
      // keyword
      public keyword(): String { "FULL OUTER JOIN" }
    }

## JoinClause

    export class JoinClause(
      public joinType: JoinType,
      public table: SafeIdentifier,
      public onCondition: SqlFragment,
    ) {}

## OrderClause

    export class OrderClause(
      public field: SafeIdentifier,
      public ascending: Boolean,
    ) {}

## Query

    export class Query(
      public tableName: SafeIdentifier,
      public conditions: List<SqlFragment>,
      public selectedFields: List<SafeIdentifier>,
      public orderClauses: List<OrderClause>,
      public limitVal: Int?,
      public offsetVal: Int?,
      public joinClauses: List<JoinClause>,
    ) {

      // where: condition must be a SqlFragment built via the sql tag
      public where(condition: SqlFragment): Query {
        let nb = conditions.toListBuilder();
        nb.add(condition);
        new Query(tableName, nb.toList(), selectedFields, orderClauses, limitVal, offsetVal, joinClauses)
      }

      // select: field names must be SafeIdentifier values
      public select(fields: List<SafeIdentifier>): Query {
        new Query(tableName, conditions, fields, orderClauses, limitVal, offsetVal, joinClauses)
      }

      // orderBy
      public orderBy(field: SafeIdentifier, ascending: Boolean): Query {
        let nb = orderClauses.toListBuilder();
        nb.add(new OrderClause(field, ascending));
        new Query(tableName, conditions, selectedFields, nb.toList(), limitVal, offsetVal, joinClauses)
      }

      // limit: bubbles on negative values
      public limit(n: Int): Query throws Bubble {
        if (n < 0) { bubble() }
        new Query(tableName, conditions, selectedFields, orderClauses, n, offsetVal, joinClauses)
      }

      // offset: bubbles on negative values
      public offset(n: Int): Query throws Bubble {
        if (n < 0) { bubble() }
        new Query(tableName, conditions, selectedFields, orderClauses, limitVal, n, joinClauses)
      }

      // join: generic join method
      public join(joinType: JoinType, table: SafeIdentifier, onCondition: SqlFragment): Query {
        let nb = joinClauses.toListBuilder();
        nb.add(new JoinClause(joinType, table, onCondition));
        new Query(tableName, conditions, selectedFields, orderClauses, limitVal, offsetVal, nb.toList())
      }

      // innerJoin
      public innerJoin(table: SafeIdentifier, onCondition: SqlFragment): Query {
        join(new InnerJoin(), table, onCondition)
      }

      // leftJoin
      public leftJoin(table: SafeIdentifier, onCondition: SqlFragment): Query {
        join(new LeftJoin(), table, onCondition)
      }

      // rightJoin
      public rightJoin(table: SafeIdentifier, onCondition: SqlFragment): Query {
        join(new RightJoin(), table, onCondition)
      }

      // fullJoin
      public fullJoin(table: SafeIdentifier, onCondition: SqlFragment): Query {
        join(new FullJoin(), table, onCondition)
      }

      // toSql: assembles the final SqlFragment
      public toSql(): SqlFragment {
        let b = new SqlBuilder();

        b.appendSafe("SELECT ");
        if (selectedFields.isEmpty) {
          b.appendSafe("*");
        } else {
          b.appendSafe(selectedFields.join(", ") { f => f.sqlValue });
        }

        b.appendSafe(" FROM ");
        b.appendSafe(tableName.sqlValue);

        for (let jc of joinClauses) {
          b.appendSafe(" ");
          b.appendSafe(jc.joinType.keyword());
          b.appendSafe(" ");
          b.appendSafe(jc.table.sqlValue);
          b.appendSafe(" ON ");
          b.appendFragment(jc.onCondition);
        }

        if (!conditions.isEmpty) {
          b.appendSafe(" WHERE ");
          b.appendFragment(conditions[0]);
          for (var i = 1; i < conditions.length; ++i) {
            b.appendSafe(" AND ");
            b.appendFragment(conditions[i]);
          }
        }

        if (!orderClauses.isEmpty) {
          b.appendSafe(" ORDER BY ");
          var first = true;
          for (let oc of orderClauses) {
            if (!first) { b.appendSafe(", "); }
            first = false;
            b.appendSafe(oc.field.sqlValue);
            b.appendSafe(if (oc.ascending) { " ASC" } else { " DESC" });
          }
        }

        let lv = limitVal;
        if (lv != null) {
          b.appendSafe(" LIMIT ");
          b.appendInt32(lv);
        }
        let ov = offsetVal;
        if (ov != null) {
          b.appendSafe(" OFFSET ");
          b.appendInt32(ov);
        }

        b.accumulated
      }

      // safeToSql: production-safe variant, applies defaultLimit if none set (CWE-400)
      public safeToSql(defaultLimit: Int): SqlFragment throws Bubble {
        if (defaultLimit < 0) { bubble() }
        if (limitVal != null) { toSql() } else { this.limit(defaultLimit).toSql() }
      }

    }

## from

Entry point. `tableName` must be a `SafeIdentifier`.

    export let from(tableName: SafeIdentifier): Query {
      new Query(tableName, [], [], [], null, null, [])
    }

## col

Qualified column reference helper. Both `table` and `column` must be
`SafeIdentifier` values, so the result is always safe.

    export let col(table: SafeIdentifier, column: SafeIdentifier): SqlFragment {
      let b = new SqlBuilder();
      b.appendSafe(table.sqlValue);
      b.appendSafe(".");
      b.appendSafe(column.sqlValue);
      b.accumulated
    }
