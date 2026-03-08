# Usage examples

This is a complex composition with a branching instruction.

    test("Using html tag to make a link") {
      let makeLink(url: String, text: SafeHtml?): SafeHtml {
        html"""
          "<a href=${ url }>
          "{: if (text != null) { :}
            "${text}
          "{: } else { :}
            "click me
          "{: } :}
          "</a>
      }
      assert(
        makeLink("https://hello.example.com/", html"Hello").text ==
        '<a href="https://hello.example.com/">\nHello\n</a>'
      );
    }

    let { ... } = import("../html");
