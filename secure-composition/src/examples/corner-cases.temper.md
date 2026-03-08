# Corner cases

Here are some corner cases thar establish behaviour but which are not
as valuable as documentation as the ones in the main
safe-html.temper.md.

## HTML comments

HTML comment syntax is a bit complicated, so we check that we've reduced the attack surface.

Some oddities of HTML comment syntax:

- `<!-->` and `<!--->` are fully formed content since the opener, `<!--` can share dashes with the closer, `-->`.
- `<!doctype...>` and `<!ducktype...>` are "bogus comments" as are `<?not-an-xml-processing-instruction...>`

By recognizing these we avoid interpolation attacks like
`html"<!--${left}->${right}-->"` where the intent might be to document
for debugging that *left* points to *right* but by causing left to be
the empty string an attacker could merge the `->` with the preceding
dashes and then get right to be interpreted as tags.

We could support interpolation into IE conditional comments, but have
not yet added rules for that so our interpolation just puts in spaces
that avoid token merging conflicts.

    test("interpolation into comments") {
      let left = "";
      let right = "payload";
      assert(html"<!--${left}->${right}-->".toString()
             ==  "<!-- -> -->");
    }

    test("end tag prefixes not allowed in string content") {
      // We want to avoid injecting </style> or a substring
      // that combines with other content.
      // If an attacker can inject a style end tag, then they
      // can probably inject a script start tag.
      assert(html"<style>body { background: '<${"/style/"}' }</style>".toString() ==
                 "<style>body { background: '\\3c /style/' }</style>");
      assert(html"<style>body { background: '\<${"/style/"}' }</style>".toString() ==
                 "<style>body { background: '\\3c /style/' }</style>");
    }

    test("quotes inserted around urls properly") {
      assert(html"<style>a{background-image: url( ${"foo/bar.png"} )}</style>".toString() ==
                 "<style>a{background-image: url( \"foo/bar.png\" )}</style>");
      assert(html"<style>a{background-image: url('${"foo/bar.png"}')}</style>".toString() ==
                 "<style>a{background-image: url('foo/bar.png')}</style>");
      assert(html"""
                 "<style>a{background-image: url("${"foo/bar.png"}")}</style>
                 .toString() ==
                 "<style>a{background-image: url(\"foo/bar.png\")}</style>");
    }

    test("quotes fixed on line break") {
      assert(
        html"""
          "<style>#foo:before {
          "  content: '${"foo"}
          "}</style>
        .toString() ==
        """
          "<style>#foo:before {
          "  content: 'foo'
          "}</style>
      );
      assert(
        html"""
          "<style>#foo:before {
          "  content: "${"foo"}
          "}</style>
        .toString() ==
        """
          "<style>#foo:before {
          "  content: "foo"
          "}</style>
      );
      assert(
        html"""
          "<style>#foo {
          "  background-image: url(${"foo"}
          ")}</style>
        .toString() ==
        """
          "<style>#foo {
          "  background-image: url("foo"
          ")}</style>
      );
      assert(
        html"""
          "<style>#foo {
          "  background-image: url("${"foo"}
          ")}</style>
        .toString() ==
        """
          "<style>#foo {
          "  background-image: url("foo"
          ")}</style>
      );
      assert(
        html"""
          "<style>#foo {
          "  background-image: url('${"foo"}
          ")}</style>
        .toString() ==
        """
          "<style>#foo {
          "  background-image: url('foo'
          ")}</style>
      );
    }

    test("double quotes in attribute value with inserted quotes") {
      assert(
          html"""
            "<div id=a"b>
            .text
          == '<div id="a&#34;b">');
    }

    test("colons in URLs not at the beginning") {
      let linkFromTwoParts(a: String, b: String): SafeHtml {
        html"<a href='${a}${b}'>clickable</a>"
      }
      // Can't construct a dangerous URL by controlling two parts.
      assert(linkFromTwoParts("java", "script:alert(1)").toString() ==
             "<a href='javaabout:zz_Temper_zz#'>clickable</a>");
      // But hostports and ipv6 addresses are ok.
      assert(linkFromTwoParts("https://", "example.com:443/").toString() ==
             "<a href='https://example.com:443/'>clickable</a>");
      assert(linkFromTwoParts("https://", "[2001:db8:85a3:8d3:1319:8a2e:370:7348]").toString() ==
             "<a href='https://[2001:db8:85a3:8d3:1319:8a2e:370:7348]'>clickable</a>");
    }

    test("In JS, difference between RegExp and Div contexts matter") {
      let n = 2.5;
      assert(
        html"""
          "<script>
          "  1 / ${n} /i
          //     ^^^^  Dividing by a number
          "  ; / ${n} /i
          //     ^^^^  Into a RegExp literal
          "</script>
          .toString() ==
        """
          "<script>
          "  1 / 2.5 /i
          "  ; / 2\\.5 /i
          //        ^ Dot has a special meaning in RegExps so it is escaped.
          "</script>
      );
    }

TODO: encoded commas in srcset interpolations: interpolated value like `http://example.com/,javascript:evil()`
