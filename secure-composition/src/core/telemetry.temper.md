# Compile-time Telemetry

Since we're doing analysis of the syntax of templates, it's nice to be able
to surface obvious syntactic errors.

```temper inert
html"<a href=${ x }>foo</a"
//                        ┃
//     Forgot the '>'━━━━━┛
```

Diagnostic callbacks allow the macro simplifiers to invoke operators like
the below, letting them convey information back to the compiler which it
can then expose as error messages.

- Context propagation functions can indicate that a transition is being
  taken that indicates a malformed template.
- Context merging functions can indicate that they're joining contexts
  that are incompatible and so making worst case assumptions.
- Escaper pickers can convey that no escaper makes sense in this context,
  so they're falling back to an escaper that ignores the interpolated value.

Diagnostics are conveyed via an optional extra argument to those operators.

    export let DiagnosticCallout = fn (String, Boolean): Void;

The String is the diagnostic, and the Boolean is true when it is
probably not fit for production code, i.e. when it probably warrants a
compiler error message instead of a warning message.

Other diagnostics can be conveyed via overload choices.
For example, an escaper might have multiple overloads for different input
types, but some are silly: why would you interpolate *SafeCss* into an HTML
Pcdata context?
