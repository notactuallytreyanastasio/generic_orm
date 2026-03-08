# Secure Composition

*Secure composition* allows combining trusted and untrusted pieces
into a whole that is trustworthy in some context.

The goal if this library is to allow developers to focus on building
apps and systems and delegate securing the system against injection
attacks to code designed and maintained by professional security
engineers who focus on the nuances of secure composition.

For example,
in an HTML context, the *html* tag produces *SafeHtml*.

```temper inert
let inputX = "<script>alert(document.origin)</script>";
// Imagine `input` is an untrusted input

// The `html` tag composes safe HTML from a trusted and untrusted parts.
let safeHtml: SafeHtml = html"<b>${ inputX }</b>";
//                            ┗━┛   ┗━━━━┛  ┗━━┛
//                        Trusted Untrusted Trusted

// The input is defanged by escaping its plain text value as HTML PCDATA.
console.log(safeHtml.text);
// <b>&lt;script&gt;alert(document.origin)&lt;/script&gt;</b>
```

A developer can easily create *SafeHtml* values and then use them with
APIs that leverage *type safety* to ensure *system safety*, for example,
with *HttpResponse.setHtmlContentBody*.

This library defines several contexts:

| module  | languages  | tags                | types                 |
| ------- | ---------- | ------------------- | --------------------- |
| ./path  | File paths | *path*              | *FilePath*            |
| ./html  | HTML       | *html*              | *SafeHtml*, *SafeUrl* |
| ./sql   | SQL        | *mysql*, *postgres* | *SafeSql*             |
| ./shell | UNIX Shell | *bash*              | *SafeShell*           |
