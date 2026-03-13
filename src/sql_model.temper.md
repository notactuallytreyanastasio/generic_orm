# SQL Data Model

We build an abstract model that can be rendered to different dialects.

## SqlFragment

The fragment class supports semi-structured raw SQL text and data values, which
then can be escaped differently for different dbs. And while called a
"fragment", an instance can also represent a full statement.

    export class SqlFragment(
      public parts: List<SqlPart>,
    ) {

      // toSource: freeze to string source content marked as safe SQL
      public toSource(): SqlSource {
        new SqlSource(toString())
      }

      // toString
      public toString(): String {
        let builder = new StringBuilder();
        for (var i = 0; i < parts.length; ++i) {
          parts[i].formatTo(builder);
        }
        builder.toString()
      }

    }

## SqlPart

Each part of a SQL fragment is either raw known-safe SQL source or else a value
needing escaped and/or represented properly for a particular DB dialect.

    export sealed interface SqlPart {

      // formatTo: enables using a single StringBuilder across multiple parts
      public formatTo(builder: StringBuilder): Void;

    }

## SqlSource

`SqlSource` represents known-safe SQL source code that doesn't need escaped.

    export class SqlSource(public source: String) extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append(source);
      }

    }

## SqlBoolean

    export class SqlBoolean(public value: Boolean) extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append(if (value) { "TRUE" } else { "FALSE" });
      }

    }

## SqlDate

    export class SqlDate(public value: Date) extends SqlPart {

      // formatTo: quote-wraps with escaping (defense-in-depth against future Date formats)
      public formatTo(builder: StringBuilder): Void {
        builder.append("'");
        for (let c of value.toString()) {
          if (c == char'\'') {
            builder.append("''");
          } else {
            builder.appendCodePoint(c) orelse panic();
          }
        }
        builder.append("'");
      }

    }

## SqlFloat64

    export class SqlFloat64(public value: Float64) extends SqlPart {

      // formatTo: rejects NaN/Infinity which are not valid SQL literals (CWE-20)
      public formatTo(builder: StringBuilder): Void {
        let s = value.toString();
        if (s == "NaN" || s == "Infinity" || s == "-Infinity") {
          builder.append("NULL");
        } else {
          builder.append(s);
        }
      }

    }

## SqlInt32

    export class SqlInt32(public value: Int32) extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append(value.toString());
      }

    }

## SqlInt64

    export class SqlInt64(public value: Int64) extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append(value.toString());
      }

    }

## SqlDefault

`SqlDefault` renders the literal SQL keyword `DEFAULT`, used for columns
with server-side default values (e.g., `NOW()` for timestamps).

    export class SqlDefault() extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append("DEFAULT");
      }

    }

## SqlString

`SqlString` represents text data that needs escaped.

    export class SqlString(public value: String) extends SqlPart {

      // formatTo
      public formatTo(builder: StringBuilder): Void {
        builder.append("'");
        for (let c of value) {
          if (c == char'\'') {
            builder.append("''");
          } else {
            builder.appendCodePoint(c) orelse panic();
          }
        }
        builder.append("'");
      }

    }
