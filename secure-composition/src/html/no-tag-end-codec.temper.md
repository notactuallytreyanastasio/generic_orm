# No Tag End Codec

NoTagEndCodec ensures that content embedded in special HTML tags does not introduce unexpected close tags.

These paths should never be triggered because the subsidiaries should adjust content.

    export class NoTagEndCodec extends Codec {
      public decode(x: String): String { x }

      public encode(x: String): String {
        var replacement: StringBuilder? = null;
        var upTo = String.begin;
        var i = String.begin;
        let end = x.end;
        while (i < end) {
          let c = x[i];
          var nextI = x.next(i);
          if (c == char'<' && nextI < end) {
            let nextC = x[nextI];
            if (nextC == char'/') {
              let sbOrNull = replacement;
              let sb = sbOrNull ?? new StringBuilder();
              replacement = sb;
              sb.appendBetween(x, upTo, i);
              sb.append("< /");
              nextI = x.next(nextI);
              upTo = nextI;
            }
          }
          i = nextI;
        }
        let sbOrNull = replacement;
        if (sbOrNull == null) {
          x
        } else {
          sbOrNull.appendBetween(x, upTo, end);
          sbOrNull.toString()
        }
      }
    }

    export let noTagEndCodec: NoTagEndCodec = doPure { (): NoTagEndCodec => new NoTagEndCodec() };

## Problematic contexts in web languages

Here are some examples of contexts, and how subsidiaries should work around them.

### JavaScript

Comparison to match index of regular expression.
Ouer layers should insert a space between `<` and `/` outside any string context.

```js
123</script/.exec(myString).index

// Sanitize to

123< /script/.exec(myString).index


x <// A line comment
  y

// Sanitize to

x < // A line comment
  y

// Similarly for block comments.
```

Any interpolation of a regular expression constructor expression can
be adjusted to have a space at the start.

In a string, any `</` sequence can be rewritten to `<\/` and a
`<` right before an interpolation of a string part can be escaped to
a `\u003c` though this may affect semantics of tagged backtick strings
that require raw content.

```js

"</foo"

// Sanitize to

"<\/foo"


"Some string content <
//                    ^ INTERPOLATION HERE

// Sanitize to

"Some string content \u003c
//                         ^ INTERPOLATION HERE
```

In a comment body, we can insert a space after any `<` without affecting semantics.

### CSS

CSS allows the `</` sequence in several contexts that we might encounter in templates from trusted developers:

- comments `/* ... */`
- inside a quoted string or `url(...)` construct

CSS has a `>` selector operator but not a corresponding `<` operator.
Media queries do have a `<` operator, but the `</` sequence can be adjusted to `< /` without semantic risk.


```cs
#foo:before { content: "</" }

// Sanitize to

#foo:before { content: "\3c /" }


#foo:before { content: "\</" }

// Sanitize to

#foo:before { content: "\3c /" }


/* CSS comment 1</0 */

// Ok if the codec converts to

/* CSS comment 1< /0 */
```
