# JavaScript parsing delegate

With a JavaScript delegate, a web application server can provision scripts with data.
TODO: Allow a Json AST to inline nicely.

    test("values in JavaScript") {
      assert(
        html"""
          "<script>
          "  let currentUserId = ${"n-{1234}#Alice O'Reilly"};
          "</script>
        .toString() ==
        """
          "<script>
          "  let currentUserId = "n-\\x7b1234}#Alice O\\'Reilly";
          "</script>
      );
    }


## Delegate

*SafeJs* is a value that is known to contain JavaScript source constructed by safe means.

    export class SafeJs(public text: String) {
      public toString(): String { text }
    }

    let JsEscState = AutoescState<JsEscaperContext, HtmlEscaper>;

HtmlJsDelegate handles safe injection in HTML `<script>` element bodies and in event handler
attribute values like `onclick="..."`.

    export class HtmlJsDelegate extends ContextDelegate<JsEscaperContext, HtmlEscaper> & HtmlDelegate {
      private var _state: JsEscState = new JsEscState(new JsEscaperContext(0, 0, 0i64, 0), null);

      public get state(): JsEscState { _state }
      public set state(x: JsEscState): Void { _state = x; }
      public get sameState(): fn (JsEscState, JsEscState): Boolean {
        jsStatesEqual
      }

      public get contextPropagator(): ContextPropagator<JsEscaperContext, HtmlEscaper> {
        jsContextPropagator
      }

      public escaper(outer: HtmlEscaper, callout: DiagnosticCallout?): HtmlEscaper {
        let state = this.state;
        let jsEscaper: JsEscaper = when (state.context.jsState) {
          jsStateTop -> jsValueEscaper;
          jsStateBCmt,
          jsStateLCmt -> do {
            if (callout != null) {
              callout("Interpolation into a JavaScript comment has no effect", false);
            }
            jsSpaceEscaper
          };
          jsStateId -> jsIdentEscaper;
          jsStateDStr,
          jsStateSStr,
          jsStateBStr -> jsStringPartEscaper;
          jsStateRegx,
          jsStateChSet -> jsRegexPartEscaper;
          else -> panic();
        }

        let escaper = new HtmlJsEscaperAdapter(jsEscaper, outer);

        let subsidiary = state.subsidiary;
        if (subsidiary != null) {
          subsidiary.delegate.escaper(escaper, callout) orelse panic()
        } else {
          escaper
        }
      }

      public close(callout: DiagnosticCallout?): Void {
        let state = this.state;
        let sub = state.subsidiary;
        if (sub != null) {
          sub.delegate.close(callout);
        }
        if (callout != null) {
          let problem = when (state.context.jsState) {
            jsStateTop -> null;
            jsStateBCmt -> "JS ended inside a comment. Are you missing '*/'?";
            jsStateLCmt -> null;
            jsStateId -> null;
            jsStateDStr -> "JS ended inside a quoted string. Are you missing a quote (\")?";
            jsStateSStr -> "JS ended inside a quoted string. Are you missing a quote (')?";
            jsStateBStr -> "JS ended inside a quoted string. Are you missing a quote (`)?";
            jsStateRegx -> "JS ended inside a regular expression. Are you missing '/'?";
            jsStateChSet -> "JS ended inside a regular expression. Are you missing ']/'?";
            else -> panic();
          }
          if (problem != null) {
            callout(problem, true);
          }
        }
      }

      public toString(): String {
        "HtmlJsDelegate(${_state})"
      }
    }

## Contexts

JsEscaperContext tracks parsing in trusted JavaScript.

    export class JsEscaperContext(

jsState allows for streaming parsing of JavaScript in script elements
and event handler attributes.  This state requires being able to track
whether a `/` starts a regular expression or a division operator.

jsState tracks states in the [JS2.0 lexical grammar][js20-lexer-grammar],
adjusted to deal with more modern constructs like backtick strings.
That lexical grammar was an experiment. It is approximate but highly accurate,
and sufficient since our threat model excludes obfuscated, trusted string parts.
*jsState* captures the kind of token we're in along with the (re, div, unit) bits.

      public jsState: Int32,

jsTau gives info about the next kind of token that can appear.

      public jsAllow: Int32,

jsStack provides additional JavaScript parsing context.  Handling
nested interpolations in backtick strings requires knowing whether a
`}` ends an interpolation or is a regular token.

      public jsStack: Int64,

jsDepth is the number of bits on jsStack in play.
jsStack is limited to 63 levels of `{` bracket nesting so this
shouldn't exceed that.  Our threat model excludes malicious developers
of fixed string parts so just don't.

      public jsDepth: Int32,
    ) extends Context {
      public toString(): String { "JsEscaperContext(${jsStateStr(jsState)}, ${jsAllowStr(jsAllow)}, ${jsStack}, ${jsDepth})" }
    }

    /** Expect more JavaScript token content. */
    let jsStateTop = 0;
    /** Block comment */
    let jsStateBCmt = 1;
    /** Line comment */
    let jsStateLCmt = 2;
    /** Identifier */
    let jsStateId = 3;
    /** Double quoted string */
    let jsStateDStr = 4;
    /** Single quoted string */
    let jsStateSStr = 5;
    /** Backtick quoted string */
    let jsStateBStr = 6;
    /** Regular expression constructor */
    let jsStateRegx = 7;
    /** Regular expression inside a [...] charset */
    let jsStateChSet = 8;

    /** any `/` could be interpreted as the beinning of a regular expression. */
    let jsAllowRe = 0;
    /** any `/` could be interpreted as part of a division or division-assignment operator. */
    let jsAllowDiv = 1;

    let jsDepthZero = 0;

## Computations

These let the transition table formula manipulate stack and depth when we see a `${`, `{`, or `}` token.

    let computeJsShl(jsStackValue: Int64): Int64 { jsStackValue * 2i64 }
    let computeJsShlp(jsStackValue: Int64): Int64 { jsStackValue * 2i64 + 1i64 }
    let computeJsShr(jsStackValue: Int64): Int64 { jsStackValue / 2i64 }
    let computeJsIsProg(jsStackValue: Int64): Boolean { (jsStackValue & 1i64) == 0i64 }
    let computeJsIsBStr(jsStackValue: Int64): Boolean { (jsStackValue & 1i64) != 0i64 }
    let computeJsIncr(jsDepth: Int32): Int32 { jsDepth + 1 }
    let computeJsDecr(jsDepth: Int32): Int32 { if (jsDepth > 0) { jsDepth - 1 } else { 0 } }

## URL Transition table

<!-- TRANSITION_TABLE: Js; jsState, jsAllow, jsStack, jsDepth -->

{{nl-chars}} = `\r\n\u2028\u2029`
{{nl}} = `[{{nl-chars}}]`
{{sp-chars}} = `\u0009\u000c \u00a0\u2000-\u200B\u3000\uFEFF`
{{sp}} = `[{{sp-chars}}]`
{{hex}} = `[0-9A-Fa-f]`
{{id-start}} = `(?:[A-Za-z$_\u0080-\u{10ffff}]/\\u{{hex}}{4})`
{{id-cont}} = `(?:[A-Za-z0-9$_\u0080-\u{10ffff}]/\\u{{hex}}{4})`
{{num-literal}} = `(?:0[Xx][0-9A-Fa-f]+/(?:(?:(?:0/[1-9][0-9]*)(?:[.][0-9]+?)?/[.][0-9]+)(?:[Ee][+-]?[0-9]+)?))n?`
{{pre-re-kwds}} = `(?:abstract/await/break/case/catch/const/continue/debugger/default/delete/do/else/enum/export/extends/final/finally/for/function/goto/if/implements/import/in/instanceof/interface/is/namespace/native/new/package/return/static/switch/synchronized/throw/throws/transient/try/typeof/use/var/volatile/while/with/yield)`
{{pre-re-punc}} = `(?:!={0,2}/#/%=?/&&?=?/\(/\*\*?=?/\+=?/,/-=?/=>/[.](?:[.][.])?/:/;/<<?=?/={1,3}/>{1,3}=?/\?(?:\?=?/[.]?)/@/\[/^=?/[/]{1,2}=?/~)`
{{pre-div-punc}} = `\)|\]|\+\+|--`

| In                      | Regex                            | Substitution | Out                         | Problem |
| ----------------------- | -------------------------------- | ------------ | --------------------------- | ------- |
| Top,   _,    _,    _    | `\/\*`                           |              | BCmt,  _,    _,    _        |         |
| Top,   _,    _,    _    | `\/\/`                           |              | LCmt,  _,    _,    _        |         |
| Top,   _,    _,    _    | `[{{nl-chars}}{{sp-chars}}]+`    |              | _,     _,    _,    _        |         |
| Top,   _,    _,    _    | `"`                              |              | DStr,  Div,  _,    _        |         |
| Top,   _,    _,    _    | `'`                              |              | SStr,  Div,  _,    _        |         |
| Top,   _,    _,    _    | `\u0060`                         |              | BStr,  Div,  _,    _        |         |
| Top,   _,    _,    _    | `{{num-literal}}`                |              | _,     Div,  _,    _        |         |
| Top,   Re,   _,    _    | `\/`                             |              | Regx,  Div,  _,    _        |         |
| Top,   _,    _,    _    | `\/=?`                           |              | _,     Re,   _,    _        |         |
| Top,   _,    _,    _    | `{{pre-re-kwds}}(?!{{id-cont}})` |              | _,     Re,   _,    _        |         |
| Top,   _,    _,    _    | `{{id-start}}{{id-cont}}*`       |              | _,     Div,  _,    _        |         |
| Top,   _,    _,    _    | `{{pre-div-punc}}`               |              | _,     Div,  _,    _        |         |
| Top,   _,    _,    _    | `{{pre-re-punc}}`                |              | _,     Re,   _,    _        |         |
| Top,   _,    _,    _    | `\{`                             |              | _,     Re, #shl,   #incr    |         |
| Top,   _,#isProg,  _    | `\}`                             |              | _,     Re, #shr,   #decr    |         |
| Top,   _,    _,    _    | `.`                              |              | _,     _,    _,    _        |         |
| Top,   _,    _,    _    | ${}                              |              | _,     Div,  _,    _        |         |
| BCmt,  _,    _,    _    | `\*+\/`                          |              | Top,   _,    _,    _        |         |
| BCmt,  _,    _,    _    | `(?:[^*]/\*+[^*\/])+`            |              | _,     _,    _,    _        |         |
| LCmt,  _,    _,    _    | `{{nl}}`                         |              | Top,   _,    _,    _        |         |
| LCmt,  _,    _,    _    | `[^{{nl-chars}}]+`               |              | _,     _,    _,    _        |         |
| DStr,  _,    _,    _    | `"`                              |              | Top,   Div,  _,    _        |         |
| DStr,  _,    _,    _    | `(?={{nl}})`                     | `"`          | Top,   Div,  _,    _        | E: Missing close quote for JavaScript string |
| DStr,  _,    _,    _    | `(?:[^"\\{{nl-chars}}]/\\.?)+`   |              | _,     _,    _,    _        |         |
| SStr,  _,    _,    _    | `'`                              |              | Top,   Div,  _,    _        |         |
| SStr,  _,    _,    _    | `(?={{nl}})`                     | `'`          | Top,   Div,  _,    _        | E: Missing close quote for JavaScript string |
| SStr,  _,    _,    _    | `(?:[^'\\{{nl-chars}}]/\\.?)+`   |              | _,     _,    _,    _        |         |
| BStr,  _,    _,    _    | `\u0060`                         |              | Top,   Div,  _,    _        |         |
| BStr,  _,    _,    _    | `[$]\{`                          |              | Top,   Re, #shlp,  #incr    |         |
| Top,   _,#isBStr,  _    | `\}`                             |              | BStr,  Re, #shr,   #decr    |         |
| BStr,  _,    _,    _    | `(?:[^\u0060\\$]/\\.?)+`         |              | _,     _,    _,    _        |         |
| BStr,  _,    _,    _    | `[$]`                            |              | _,     _,    _,    _        |         |
| Regx,  _,    _,    _    | `\/[dgimsuvy]*`                  |              | Top,   Div,  _,    _        |         |
| Regx,  _,    _,    _    | `(?={{nl}})`                     | `/`          | Top,   Div,  _,    _        | E: Missing close '/' for JavaScript RegExp |
| Regx,  _,    _,    _    | `\[`                             |              | ChSet, _,    _,    _        |         |
| Regx,  _,    _,    _    | `[^{{nl-chars}}\/\[\\]/\\.?]`    |              | _,     _,    _,    _        |         |
| ChSet, _,    _,    _    | `]`                              |              | Regx,  _,    _,    _        |         |
| ChSet, _,    _,    _    | `[^{{nl-chars}}\]\\]/\\.?]`      |              | _,     _,    _,    _        |         |
| _,     _,    _,    _    | ${}                              |              | _,     _,    _,    _        |         |

<!-- /TRANSITION_TABLE -->

## Escapers

    export sealed interface JsEscaper extends Escaper {
      @overload("apply")
      applySafeHtml(x: SafeHtml): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applySafeUrl(x: SafeUrl): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applySafeCss(x: SafeCss): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applySafeJs(x: SafeJs): SafeJs;
      @overload("apply")
      applyInt32(x: Int32): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applyInt64(x: Int64): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applyFloat64(x: Float64): SafeJs { applyString(x.toString()) }
      @overload("apply")
      applyString(x: String): SafeJs;

      toString(): String;
    }

    export class HtmlJsEscaperAdapter(
      public first: JsEscaper,
      public second: HtmlEscaper,
    ) extends HtmlEscaper {
      @overload("apply")
      public applySafeHtml(x: SafeHtml): String {
        second.applySafeJs(first.applyString(x.text))
      }
      @overload("apply")
      public applySafeUrl(x: SafeUrl): String {
        second.applySafeJs(first.applySafeUrl(x))
      }
      @overload("apply")
      public applySafeCss(x: SafeCss): String {
        second.applySafeJs(first.applySafeCss(x))
      }
      @overload("apply")
      public applySafeJs(x: SafeJs): String {
        second.applySafeJs(first.applySafeJs(x))
      }
      @overload("apply")
      public applyInt32(x: Int32): String {
        second.applySafeJs(first.applyInt32(x))
      }
      @overload("apply")
      public applyInt64(x: Int64): String {
        second.applySafeJs(first.applyInt64(x))
      }
      @overload("apply")
      public applyFloat64(x: Float64): String {
        second.applySafeJs(first.applyFloat64(x))
      }
      @overload("apply")
      public applyString(x: String): String {
        second.applySafeJs(first.applyString(x))
      }

      public toString(): String { "HtmlJsEscaperAdapter(${first}, ${second})" }
    }

    export class JsValueEscaper extends JsEscaper {
      public static instance = new JsValueEscaper();

      @overload("apply")
      public applySafeJs(x: SafeJs): SafeJs {
        x
      }
      @overload("apply")
      public applyInt32(x: Int32): SafeJs {
        new SafeJs(x.toString())
      }
      @overload("apply")
      public applyInt64(x: Int64): SafeJs {
        new SafeJs(x.toString())
      }
      @overload("apply")
      public applyFloat64(x: Float64): SafeJs {
        new SafeJs(x.toString())
      }
      @overload("apply")
      public applyString(x: String): SafeJs {
        new SafeJs("\"${jsStringPartEscaper.applyString(x).text}\"")
      }
      public toString(): String { "JsValueEscaper" }
    }
    let jsValueEscaper = doPure { new JsValueEscaper() };

    let jsSpace = doPure { new SafeJs(" ") };
    export class JsSpaceEscaper extends JsEscaper {
      public static instance = new JsSpaceEscaper();

      @overload("apply")
      public applySafeHtml(x: SafeHtml): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applySafeUrl(x: SafeUrl): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applySafeCss(x: SafeCss): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applySafeJs(x: SafeJs): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applyInt32(x: Int32): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applyInt64(x: Int64): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applyFloat64(x: Float64): SafeJs {
        jsSpace
      }
      @overload("apply")
      public applyString(x: String): SafeJs {
        jsSpace
      }
      public toString(): String { "JsSpaceEscaper" }
    }
    let jsSpaceEscaper = doPure { new JsSpaceEscaper() };

    export class JsIdentEscaper extends JsEscaper {
      public static instance = new JsIdentEscaper();

      @overload("apply")
      public applySafeJs(x: SafeJs): SafeJs {
        x
      }
      @overload("apply")
      public applyString(x: String): SafeJs {
        new SafeJs(jsEscape(x, jsIdentSafe, false))
      }
      public toString(): String { "JsIdentEscaper" }
    }
    let jsIdentEscaper = doPure { new JsIdentEscaper() };

    export class JsStringPartEscaper extends JsEscaper {
      public static instance = new JsStringPartEscaper();

      @overload("apply")
      public applySafeJs(x: SafeJs): SafeJs {
        applyString(x.toString())
      }
      @overload("apply")
      public applyString(x: String): SafeJs {
        new SafeJs(jsEscape(x, jsStringSafe, true))
      }
      public toString(): String { "JsStringPartEscaper" }
    }
    let jsStringPartEscaper = doPure { new JsStringPartEscaper() };

JsRegexPartEscaper operates equivalently to JavaScript's [RegExp.escape].

    export class JsRegexPartEscaper extends JsEscaper {
      public static instance = new JsRegexPartEscaper();

      @overload("apply")
      public applySafeJs(x: SafeJs): SafeJs {
        applyString(x.toString())
      }
      @overload("apply")
      public applyString(x: String): SafeJs {
        var sb = new StringBuilder();
        var upTo = String.begin;
        let end = x.end;

        for (var i = String.begin, next = String.begin; i < end; i = next) {
          let c = x[i];
          next = x.next(i);

          if (c < 0x80) {
            // This logic follows the description of the RegExp.escape return value description.
            // It combines the first character alphanumeric and the later escaping that also
            // uses \x style escapes.
            if (
              (
                c == 0 &&
                (
                  (char'a' <= (c | 32) && (c | 32) <= char'z') ||
                  (char'0' <= c && c <= char'9')
                )
              ) || jsEscapeWith2XInRegex[c]
            ) {
              sb.appendBetween(x, upTo, i);
              appendXHex2(c, sb);
              upTo = next;
            } else if (jsEscapeWithBackslashInRegex[c]) {
              sb.appendBetween(x, upTo, i);
              sb.append("\\");
              sb.appendCodePoint(c) orelse panic();
              upTo = next;
            } else if (c <= 0x20) {
              let esc = when (c) {
                char'\t' -> "\\t";
                0xB -> "\\v";
                0xC -> "\\f";
                char'\r' -> "\\r";
                char'\n' -> "\\n";
                else -> null;
              };
              if (esc != null) {
                sb.appendBetween(x, upTo, i);
                sb.append(esc);
                upTo = next;
              }
            }
          } else {
            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#white_space
            // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGeneral_Category%3DSpace_Separator%7D
            if (c == 0xA0 || c == 0x1680 || 0x2000 <= c && c <= 0x200A ||
                 c == 0x202F || c == 0x205F || c == 0x3000 || c == 0xFEFF) {
              sb.appendBetween(x, upTo, i);
              sb.append("\\u");
              appendHex4(c, sb);
              upTo = next;
            }
          }
        }

        new SafeJs(
          if (upTo == String.begin) {
            x
          } else {
            sb.appendBetween(x, upTo, end);
            sb.toString()
          }
        )
      }
      public toString(): String { "JsRegexPartEscaper" }
    }
    let jsRegexPartEscaper = doPure { new JsRegexPartEscaper() };

    let jsIdentSafe = doPure {
      let lb = new ListBuilder<Boolean>();
      for (var i = 0; i < 128; ++i) {
        lb.add(
          (char'0' <= i && i <= char'9') ||
          (char'A' <= i && i <= char'Z') ||
          (char'a' <= i && i <= char'z') ||
          i == char'_' || i == char'$'
        );
      }
      lb.toList()
    };

    let jsStringSafe = doPure {
      let lb = new ListBuilder<Boolean>();
      for (var i = 0; i < 128; ++i) { lb.add(i >= char' '); }
      lb[char'\\'] = false;
      lb[char'$'] = false;
      lb[char'{'] = false;
      lb[char'"'] = false;
      lb[char'\''] = false;
      lb[char'`'] = false;
      lb.toList()
    };

    let jsEscapeWith2XInRegex = doPure {
      let lb = new ListBuilder<Boolean>();
      for (var i = 0; i < 128; ++i) { lb.add(false); }
      lb[char','] = true;
      lb[char'-'] = true;
      lb[char'='] = true;
      lb[char'<'] = true;
      lb[char'>'] = true;
      lb[char'#'] = true;
      lb[char'&'] = true;
      lb[char'!'] = true;
      lb[char'%'] = true;
      lb[char':'] = true;
      lb[char';'] = true;
      lb[char'@'] = true;
      lb[char'~'] = true;
      lb[char'\''] = true;
      lb[char'`'] = true;
      lb[char'"'] = true;
      lb.toList()
    };

    let jsEscapeWithBackslashInRegex = doPure {
      let lb = new ListBuilder<Boolean>();
      for (var i = 0; i < 128; ++i) { lb.add(false); }
      lb[char'^'] = true;
      lb[char'$'] = true;
      lb[char'\\'] = true;
      lb[char'.'] = true;
      lb[char'*'] = true;
      lb[char'+'] = true;
      lb[char'?'] = true;
      lb[char'('] = true;
      lb[char')'] = true;
      lb[char'['] = true;
      lb[char']'] = true;
      lb[char'{'] = true;
      lb[char'}'] = true;
      lb[char'|'] = true;
      lb[char'/'] = true;
      lb.toList()
    };

    let jsEscape(x: String, safes: List<Boolean>, areNonBasicPlaneSafe: Boolean): String {
      // JS identifiers can use \u escapes.
      // Just escape any of those we don't recognize as identifier
      // characters.
      var sbOrNull: StringBuilder? = null;
      var upTo = String.begin;
      let end = x.end;
      for (var i = String.begin, next = String.begin; i < end; i = next) {
        next = x.next(i);
        let c = x[i];
        if (c < safes.length) {
          if (safes[c]) { continue }
        } else if (areNonBasicPlaneSafe) {
          continue;
        }
        let sbNow = sbOrNull;
        let sb = sbNow ?? new StringBuilder();
        sbOrNull = sb;
        sb.appendBetween(x, upTo, i);
        jsEscapeOnto(c, sb);
        upTo = x.next(i);
      }
      let sbAtEnd = sbOrNull;
      if (sbAtEnd != null) {
        sbAtEnd.appendBetween(x, upTo, end);
        sbAtEnd.toString()
      } else {
        x
      }
    }

    let jsEscapeOnto(c: Int32, sb: StringBuilder): Void {
      if (c < 0x100) {
        when (c) {
          char'\t' -> sb.append("\\t");
          char'\n' -> sb.append("\\n");
          char'\r' -> sb.append("\\r");
          char'\"' -> sb.append("\\\"");
          char'\'' -> sb.append("\\\'");
          char'\\' -> sb.append("\\\\");
          else -> appendXHex2(c, sb);
        }
      } else if (c < 0x1_0000) {
        sb.append("\\u")
        appendHex4(c, sb);
      } else {
        let u = c - 0x1_0000;
        let leading = (u / 0x400) + 0xD800;
        let trailing = (u & 0x3FF) + 0xDC00;
        sb.append("\\u")
        appendHex4(leading, sb);
        sb.append("\\u")
        appendHex4(trailing, sb);
      }
    }

    let appendHex4(i: Int32, sb: StringBuilder): Void {
      appendHex((i / 0x100) & 0xF, sb);
      appendHex((i / 0x40) & 0xF, sb);
      appendHex((i / 0x10) & 0xF, sb);
      appendHex(i & 0xF, sb);
    }

    let appendXHex2(i: Int32, sb: StringBuilder): Void {
      sb.append("\\x");
      appendHex((i / 0x10) & 0xF, sb);
      appendHex(i & 0xF, sb);
    }

    let { appendHex } = import("../url");

[js20-lexer-grammar]: https://www-archive.mozilla.org/js/language/js20-2002-04/rationale/syntax.html
[RegExp.escape]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/escape#return_value

