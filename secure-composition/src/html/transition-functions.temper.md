# Transition tables for Safe HTML

The transition tables are defined in `./safe-html.temper.md` and
similar files but the code generated from them lives here.

<details><summary>Generated propagation functions</summary>

<!-- GENERATED_PROPAGATION_FN: Html -->

    let htmlStateStr(x: Int32): String {
      when (x) {
        htmlStatePcdata -> "Pcdata";
        htmlStateOName -> "OName";
        htmlStateCName -> "CName";
        htmlStateBeforeAttr -> "BeforeAttr";
        htmlStateBeforeEq -> "BeforeEq";
        htmlStateBeforeValue -> "BeforeValue";
        htmlStateEnterAttr -> "EnterAttr";
        htmlStateAttr -> "Attr";
        htmlStateAfterAttr -> "AfterAttr";
        htmlStateSpecialBody -> "SpecialBody";
        htmlStateComment -> "Comment";
        htmlStateBogusComment -> "BogusComment";
        htmlStateError -> "Error";
        else -> x.toString();
      }
    }

    let tagStateStr(x: Int32): String {
      when (x) {
        tagStateGeneric -> "Generic";
        tagStateScript -> "Script";
        tagStateStyle -> "Style";
        else -> x.toString();
      }
    }

    let attribStateStr(x: Int32): String {
      when (x) {
        attribStateGeneric -> "Generic";
        attribStateCss -> "Css";
        attribStateJs -> "Js";
        attribStateUrl -> "Url";
        attribStateUrls -> "Urls";
        attribStateAmbig -> "Ambig";
        else -> x.toString();
      }
    }

    let delimStateStr(x: Int32): String {
      when (x) {
        delimStateUq -> "Uq";
        delimStateSq -> "Sq";
        delimStateDq -> "Dq";
        else -> x.toString();
      }
    }

    let htmlStatesEqual(
      a: AutoescState<HtmlEscaperContext, HtmlEscaper>,
      b: AutoescState<HtmlEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: HtmlEscaperContext = a.context;
      let d: HtmlEscaperContext = b.context;
      if (c.htmlState != d.htmlState) { return false; }
      if (c.tagState != d.tagState) { return false; }
      if (c.attribState != d.attribState) { return false; }
      if (c.delimState != d.delimState) { return false; }
      true
    }

    let htmlPropagateContext(
      before: AutoescState<HtmlEscaperContext, HtmlEscaper>,
      literalPart: String?,
      callout: DiagnosticCallout?,
    ): AfterPropagate<HtmlEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx'^(?:"[^"]*"?)';
      let pattern2: Regex = rgx"^(?:')";
      let pattern3: Regex = rgx"^(?:'[^']*'?)";
      let pattern4: Regex = rgx'^(?:(?:"))';
      let pattern5: Regex = rgx"^(?:(?:'))";
      let pattern6: Regex = rgx"^(?:(?:(?:[^ !-(>\n\r\t]|[!#$%&(])))";
      let pattern7: Regex = rgx"^(?:(?:--))";
      let pattern8: Regex = rgx"^(?:(?:</[Ss][Cc][Rr][Ii][Pp][Tt](?:$|[^-.0-9:A-Z_a-z])))";
      let pattern9: Regex = rgx"^(?:(?:</[Ss][Tt][Yy][Ll][Ee](?:$|[^-.0-9:A-Z_a-z])))";
      let pattern10: Regex = rgx"^(?:(?:>))";
      let pattern11: Regex = rgx"^(?:(?:[ >\n\r\t]))";
      let pattern12: Regex = rgx"^(?:(?:[ \n\r\t]*/?>))";
      let pattern13: Regex = rgx"^(?:(?:[-.0-9:A-Z_a-z]))";
      let pattern14: Regex = rgx"^(?:(?:[->]))";
      let pattern15: Regex = rgx"^(?:(?:[A-Za-z]))";
      let pattern16: Regex = rgx"^(?:(?:[Ss][Rr][Cc][Ss][Ee][Tt]))";
      let pattern17: Regex = rgx"^(?:(?:[Ss][Rr][Cc]|[Hh][Rr][Ee][Ff]))";
      let pattern18: Regex = rgx"^(?:(?:[^ !-(=>\n\r\t]|[!#$%&(])+)";
      let pattern19: Regex = rgx"^(?:(?:[^!-(>]|[!#$%&(])+)";
      let pattern20: Regex = rgx"^(?:(?:[^->]|-?>)+)";
      let pattern21: Regex = rgx"^(?:(?:[^<]|<+(?:[^/<]|$))+)";
      let pattern22: Regex = rgx"^(?:(?:[^\n]|\n)+)";
      let pattern23: Regex = rgx"^(?:,[ \n\r\t]*)";
      let pattern24: Regex = rgx"^(?:--+)";
      let pattern25: Regex = rgx"^(?:--+>)";
      let pattern26: Regex = rgx"^(?:<!)";
      let pattern27: Regex = rgx"^(?:<)";
      let pattern28: Regex = rgx"^(?:</)";
      let pattern29: Regex = rgx"^(?:<[!?])";
      let pattern30: Regex = rgx"^(?:<[Ss][Cc][Rr][Ii][Pp][Tt])";
      let pattern31: Regex = rgx"^(?:<[Ss][Tt][Yy][Ll][Ee])";
      let pattern32: Regex = rgx"^(?:=)";
      let pattern33: Regex = rgx"^(?:>)";
      let pattern34: Regex = rgx"^(?:[ \n\r\t]+)";
      let pattern35: Regex = rgx"^(?:[-0-9A-Za-z]+:)";
      let pattern36: Regex = rgx"^(?:[A-Za-z][-.0-9:A-Z_a-z]*)";
      let pattern37: Regex = rgx"^(?:[Dd][Aa][Tt][Aa]-(?:[^ !-(=>\n\r\t]|[!#$%&(])*[Uu][Rr][ILil](?:[^ !-(=>\n\r\t]|[!#$%&(])*)";
      let pattern38: Regex = rgx"^(?:[Oo][Nn](?:[^ !-(=>\n\r\t]|[!#$%&(])*)";
      let pattern39: Regex = rgx"^(?:[Ss][Tt][Yy][Ll][Ee])";
      let pattern40: Regex = rgx'^(?:[^ ">\n\r\t]+)';
      let pattern41: Regex = rgx'^(?:[^"]+)';
      let pattern42: Regex = rgx"^(?:[^']+)";
      let pattern43: Regex = rgx"^(?:[^<>]+)";
      let pattern44: Regex = rgx"^(?:[^>]+)";
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern28.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern15.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateCName,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern30.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern13.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  tagStateScript,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern13.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  tagStateStyle,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern15.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateOName,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern26.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern7.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateComment,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBogusComment,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&lt;",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&gt;",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern43.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            if (callout != null) {
              callout("HTML attribute content in close tag", false);
            }
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            if (callout != null) {
              callout("HTML attribute content in close tag", false);
            }
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStatePcdata,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          let match: Match? = pattern36.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          do {
            if ((pattern10.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          let match: Match? = pattern34.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStatePcdata,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern14.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                contextBefore,
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBogusComment) {
          let match: Match? = pattern44.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBogusComment) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStatePcdata,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateStyle) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateSpecialBody,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ).start(new HtmlCssDelegate(), noTagEndCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateScript) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateSpecialBody,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ).start(new HtmlJsDelegate(),  noTagEndCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateGeneric) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStatePcdata,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern34.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern35.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern13.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateUrls,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern13.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateUrl,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern37.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeEq,
                contextBefore.tagState,
                attribStateUrl,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern39.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern13.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateCss,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern38.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeEq,
                contextBefore.tagState,
                attribStateJs,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeEq,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          let match: Match? = pattern34.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          let match: Match? = pattern32.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeValue,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          do {
            if ((pattern12.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateEnterAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                delimStateDq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateEnterAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                delimStateSq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          let match: Match? = pattern34.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateEnterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  delimStateUq,
                ),
                before.subsidiary,
              );
            }
          }
        }
      }
      if (literalPart == null && contextBefore.htmlState == htmlStateBeforeValue) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "\"",
            String.begin,
            new HtmlEscaperContext(
              htmlStateEnterAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              delimStateUq,
            ),
            before.subsidiary,
          );
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateUrls) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              contextBefore.delimState,
            ),
            before.subsidiary,
          ).start(new HtmlUrlDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateUrl) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              contextBefore.delimState,
            ),
            before.subsidiary,
          ).start(new HtmlUrlDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateCss) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              contextBefore.delimState,
            ),
            before.subsidiary,
          ).start(new HtmlCssDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateJs) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              contextBefore.delimState,
            ),
            before.subsidiary,
          ).start(new HtmlJsDelegate(),  htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateAttr,
              contextBefore.tagState,
              contextBefore.attribState,
              contextBefore.delimState,
            ),
            before.subsidiary,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          do {
            if ((pattern4.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          do {
            if ((pattern5.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.attribState == attribStateUrls) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateEnterAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ).end();
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern40.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&#34;",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern41.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern42.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.htmlState == htmlStateAttr && contextBefore.attribState == attribStateAmbig) {
        do {
          if (callout != null) {
            callout("Cannot interpolate into ambiguous attribute.", true);
          }
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new HtmlEscaperContext(
              htmlStateError,
              tagStateGeneric,
              contextBefore.attribState,
              delimStateUq,
            ),
            before.subsidiary,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                attribStateGeneric,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                attribStateGeneric,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateUq) {
          do {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "\"",
              String.begin,
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                attribStateGeneric,
                contextBefore.delimState,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody && contextBefore.tagState == tagStateScript) {
          do {
            if ((pattern8.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStatePcdata,
                  tagStateGeneric,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody && contextBefore.tagState == tagStateStyle) {
          do {
            if ((pattern9.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new HtmlEscaperContext(
                  htmlStatePcdata,
                  tagStateGeneric,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateError) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class HtmlContextPropagator extends ContextPropagator<HtmlEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<HtmlEscaperContext, HtmlEscaper>,
        literalPart: String?,
        callout: DiagnosticCallout?,
      ): AfterPropagate<HtmlEscaperContext, HtmlEscaper> {
        htmlPropagateContext(before, literalPart, callout)
      }
    }

<!-- GENERATED_PROPAGATION_FN: Url esc=HtmlEscaper -->

    let urlStateStr(x: Int32): String {
      when (x) {
        urlStateStart -> "Start";
        urlStateBeforeQuery -> "BeforeQuery";
        urlStateQuery -> "Query";
        urlStateFragment -> "Fragment";
        else -> x.toString();
      }
    }

    let urlStatesEqual(
      a: AutoescState<UrlEscaperContext, HtmlEscaper>,
      b: AutoescState<UrlEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: UrlEscaperContext = a.context;
      let d: UrlEscaperContext = b.context;
      if (c.urlState != d.urlState) { return false; }
      true
    }

    let urlPropagateContext(
      before: AutoescState<UrlEscaperContext, HtmlEscaper>,
      literalPart: String?,
      callout: DiagnosticCallout?,
    ): AfterPropagate<UrlEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx"^(?:(?:[^#]|#)+)";
      let pattern1: Regex = rgx"^(?:[#])";
      let pattern2: Regex = rgx"^(?:[?])";
      let pattern3: Regex = rgx"^(?:[^#/:?]*[/:])";
      let pattern4: Regex = rgx"^(?:[^#/:?]+)";
      let pattern5: Regex = rgx"^(?:[^#?]+)";
      let pattern6: Regex = rgx"^(?:[^#]+)";
      if (literalPart != null) {
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateBeforeQuery,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateQuery,
              ),
              before.subsidiary,
            );
          }
        }
        do {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateFragment,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateQuery,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateFragment,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern6.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new UrlEscaperContext(
                urlStateFragment,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.urlState == urlStateFragment) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class UrlContextPropagator extends ContextPropagator<UrlEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<UrlEscaperContext, HtmlEscaper>,
        literalPart: String?,
        callout: DiagnosticCallout?,
      ): AfterPropagate<UrlEscaperContext, HtmlEscaper> {
        urlPropagateContext(before, literalPart, callout)
      }
    }

    let urlContextPropagator = doPure { new UrlContextPropagator() };

<!-- GENERATED_PROPAGATION_FN: Css esc=HtmlEscaper -->

    let cssStateStr(x: Int32): String {
      when (x) {
        cssStateTop -> "Top";
        cssStateQuoted -> "Quoted";
        cssStateComment -> "Comment";
        cssStateHash -> "Hash";
        cssStateBeforeUrl -> "BeforeUrl";
        cssStateUrl -> "Url";
        cssStateAfterUrl -> "AfterUrl";
        else -> x.toString();
      }
    }

    let cssDelimStr(x: Int32): String {
      when (x) {
        cssDelimUq -> "Uq";
        cssDelimDq -> "Dq";
        cssDelimSq -> "Sq";
        else -> x.toString();
      }
    }

    let cssStatesEqual(
      a: AutoescState<CssEscaperContext, HtmlEscaper>,
      b: AutoescState<CssEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: CssEscaperContext = a.context;
      let d: CssEscaperContext = b.context;
      if (c.cssState != d.cssState) { return false; }
      if (c.cssDelim != d.cssDelim) { return false; }
      true
    }

    let cssPropagateContext(
      before: AutoescState<CssEscaperContext, HtmlEscaper>,
      literalPart: String?,
      callout: DiagnosticCallout?,
    ): AfterPropagate<CssEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx"^(?:#)";
      let pattern2: Regex = rgx"^(?:')";
      let pattern3: Regex = rgx"^(?:((?:[^!-(-0-9<A-Z\\_a-z-􏿿]|[!#$%&(])|\\[^<]?)+)";
      let pattern4: Regex = rgx"^(?:(?:))";
      let pattern5: Regex = rgx"^(?:(?:[!#$%&*-~]|[-􏿿]|\\[0-9A-Fa-f]{1,6}[ \n\r\t]?|\\.?)+)";
      let pattern6: Regex = rgx'^(?:(?:["\n\r]))';
      let pattern7: Regex = rgx"^(?:(?:['\n\r]))";
      let pattern8: Regex = rgx"^(?:(?:[*/]))";
      let pattern9: Regex = rgx"^(?:(?:[-0-9A-Z_a-z]|\\[0-9A-Fa-f]{1,6}[ \n\r\t]?|\\.?|[-􏿿])+)";
      let pattern10: Regex = rgx"^(?:(?:[-0-9A-Z_a-z-􏿿]|\\[^<]?)+)";
      let pattern11: Regex = rgx"^(?:(?:[\n\r]))";
      let pattern12: Regex = rgx'^(?:(?:[^"\\\n\r]|\\[^<]?)+)';
      let pattern13: Regex = rgx"^(?:(?:[^'\\\n\r]|\\[^<]?)+)";
      let pattern14: Regex = rgx"^(?:(?:[^|]))";
      let pattern15: Regex = rgx"^(?:(?:\\.|[^\\\n\r])+)";
      let pattern16: Regex = rgx"^(?:/)";
      let pattern17: Regex = rgx"^(?:/\*)";
      let pattern18: Regex = rgx"^(?:<)";
      let pattern19: Regex = rgx"^(?:[ \n\r\t]+)";
      let pattern20: Regex = rgx"^(?:[*]+)";
      let pattern21: Regex = rgx"^(?:[*]+/)";
      let pattern22: Regex = rgx"^(?:[Uu][Rr][Ll][ \n\r\t]*\()";
      let pattern23: Regex = rgx"^(?:[^*/<]+)";
      let pattern24: Regex = rgx"^(?:\\<)";
      let pattern25: Regex = rgx"^(?:\\?<)";
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateBeforeUrl,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateQuoted,
                cssDelimDq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateQuoted,
                cssDelimSq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "< ",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\\3c ",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateComment,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateHash,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern10.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                cssDelimUq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                cssDelimUq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Missing close quote", true);
              }
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Missing close quote", true);
              }
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateQuoted) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\\3c ",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern12.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern14.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                contextBefore,
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "< ",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              " /",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern8.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                contextBefore,
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "* ",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateHash) {
          let match: Match? = pattern9.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateHash) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateHash) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateUrl,
                cssDelimDq,
              ),
              before.subsidiary,
            ).start(new CssUrlDelegate(), cssCodec);
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateUrl,
                cssDelimSq,
              ),
              before.subsidiary,
            ).start(new CssUrlDelegate(), cssCodec);
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateBeforeUrl) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "\"",
            String.begin,
            new CssEscaperContext(
              cssStateUrl,
              cssDelimUq,
            ),
            before.subsidiary,
          ).start(new CssUrlDelegate(), cssCodec);
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\"",
              match.full.end,
              new CssEscaperContext(
                cssStateUrl,
                cssDelimUq,
              ),
              before.subsidiary,
            ).start(new CssUrlDelegate(), cssCodec);
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new CssEscaperContext(
                  cssStateAfterUrl,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern7.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new CssEscaperContext(
                  cssStateAfterUrl,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ).end();
            }
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%27",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%22",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateUrl) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%3c",
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimUq) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateAfterUrl,
                contextBefore.cssDelim,
              ),
              before.subsidiary,
            ).end();
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                cssDelimUq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new CssEscaperContext(
                cssStateTop,
                cssDelimUq,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Mising close quote for CSS url", true);
              }
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Mising close quote for CSS url", true);
              }
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimUq) {
          do {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\"",
              String.begin,
              new CssEscaperContext(
                cssStateTop,
                cssDelimUq,
              ),
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class CssContextPropagator extends ContextPropagator<CssEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<CssEscaperContext, HtmlEscaper>,
        literalPart: String?,
        callout: DiagnosticCallout?,
      ): AfterPropagate<CssEscaperContext, HtmlEscaper> {
        cssPropagateContext(before, literalPart, callout)
      }
    }

    let cssContextPropagator = doPure { new CssContextPropagator() };

<!-- GENERATED_PROPAGATION_FN: Js esc=HtmlEscaper -->

    let jsStateStr(x: Int32): String {
      when (x) {
        jsStateTop -> "Top";
        jsStateBCmt -> "BCmt";
        jsStateLCmt -> "LCmt";
        jsStateId -> "Id";
        jsStateDStr -> "DStr";
        jsStateSStr -> "SStr";
        jsStateBStr -> "BStr";
        jsStateRegx -> "Regx";
        jsStateChSet -> "ChSet";
        else -> x.toString();
      }
    }

    let jsAllowStr(x: Int32): String {
      when (x) {
        jsAllowRe -> "Re";
        jsAllowDiv -> "Div";
        else -> x.toString();
      }
    }

    let jsStackStr(x: Int32): String {
      when (x) {
        else -> x.toString();
      }
    }

    let jsDepthStr(x: Int32): String {
      when (x) {
        jsDepthZero -> "Zero";
        else -> x.toString();
      }
    }

    let jsStatesEqual(
      a: AutoescState<JsEscaperContext, HtmlEscaper>,
      b: AutoescState<JsEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: JsEscaperContext = a.context;
      let d: JsEscaperContext = b.context;
      if (c.jsState != d.jsState) { return false; }
      if (c.jsAllow != d.jsAllow) { return false; }
      if (c.jsStack != d.jsStack) { return false; }
      if (c.jsDepth != d.jsDepth) { return false; }
      true
    }

    let jsPropagateContext(
      before: AutoescState<JsEscaperContext, HtmlEscaper>,
      literalPart: String?,
      callout: DiagnosticCallout?,
    ): AfterPropagate<JsEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx"^(?:')";
      let pattern2: Regex = rgx"^(?:(?:!={0,2}|#|%=?|&&?=?|\(|\*\*?=?|\+=?|,|-=?|=>|[.](?:[.][.])?|:|;|<<?=?|={1,3}|>{1,3}=?|\?(?:\?=?|[.]?)|@|\[|^=?|[|]{1,2}=?|~))";
      let pattern3: Regex = rgx"^(?:(?:(?:[$0-9A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})))";
      let pattern4: Regex = rgx"^(?:(?:0[Xx][0-9A-Fa-f]+|(?:(?:(?:0|[1-9][0-9]*)(?:[.][0-9]+?)?|[.][0-9]+)(?:[Ee][+-]?[0-9]+)?))n?)";
      let pattern5: Regex = rgx"^(?:(?:[$A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})(?:[$0-9A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})*)";
      let pattern6: Regex = rgx"^(?:(?:[\n\r  ]))";
      let pattern7: Regex = rgx'^(?:(?:[^"\\\n\r  ]|\\.?)+)';
      let pattern8: Regex = rgx"^(?:(?:[^$\\`]|\\.?)+)";
      let pattern9: Regex = rgx"^(?:(?:[^'\\\n\r  ]|\\.?)+)";
      let pattern10: Regex = rgx"^(?:(?:[^*]|\*+[^*/])+)";
      let pattern11: Regex = rgx"^(?:(?:abstract|await|break|case|catch|const|continue|debugger|default|delete|do|else|enum|export|extends|final|finally|for|function|goto|if|implements|import|in|instanceof|interface|is|namespace|native|new|package|return|static|switch|synchronized|throw|throws|transient|try|typeof|use|var|volatile|while|with|yield))";
      let pattern12: Regex = rgx"^(?:.)";
      let pattern13: Regex = rgx"^(?:/)";
      let pattern14: Regex = rgx"^(?://)";
      let pattern15: Regex = rgx"^(?:/=?)";
      let pattern16: Regex = rgx"^(?:/[dgimsuvy]*)";
      let pattern17: Regex = rgx"^(?:/\*)";
      let pattern18: Regex = rgx"^(?:[	 \n\r  -​  　﻿]+)";
      let pattern19: Regex = rgx"^(?:[$])";
      let pattern20: Regex = rgx"^(?:[$]\{)";
      let pattern21: Regex = rgx"^(?:[\n\r  ])";
      let pattern22: Regex = rgx"^(?:[^/\[\\\n\r  ]|\\.?])";
      let pattern23: Regex = rgx"^(?:[^\\\]\n\r  ]|\\.?])";
      let pattern24: Regex = rgx"^(?:[^\n\r  ]+)";
      let pattern25: Regex = rgx"^(?:\)|\]|\+\+|--)";
      let pattern26: Regex = rgx"^(?:\*+/)";
      let pattern27: Regex = rgx"^(?:\[)";
      let pattern28: Regex = rgx"^(?:\{)";
      let pattern29: Regex = rgx"^(?:\})";
      let pattern30: Regex = rgx"^(?:])";
      let pattern31: Regex = rgx"^(?:`)";
      if (literalPart != null) {
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateBCmt,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern14.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateLCmt,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateDStr,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateSStr,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateBStr,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && contextBefore.jsAllow == jsAllowRe) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateRegx,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowRe,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern11.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern3.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowRe,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowRe,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern28.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowRe,
                computeJsShl(contextBefore.jsStack),
                computeJsIncr(contextBefore.jsDepth),
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && computeJsIsProg(contextBefore.jsStack)) {
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowRe,
                computeJsShr(contextBefore.jsStack),
                computeJsDecr(contextBefore.jsDepth),
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern12.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.jsState == jsStateTop) {
        do {
          return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new JsEscaperContext(
              contextBefore.jsState,
              jsAllowDiv,
              contextBefore.jsStack,
              contextBefore.jsDepth,
            ),
            before.subsidiary,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.jsState == jsStateBCmt) {
          let match: Match? = pattern26.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateBCmt) {
          let match: Match? = pattern10.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateLCmt) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateLCmt) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Missing close quote for JavaScript string", true);
              }
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          let match: Match? = pattern7.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Missing close quote for JavaScript string", true);
              }
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          let match: Match? = pattern9.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                jsAllowRe,
                computeJsShlp(contextBefore.jsStack),
                computeJsIncr(contextBefore.jsDepth),
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && computeJsIsBStr(contextBefore.jsStack)) {
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateBStr,
                jsAllowRe,
                computeJsShr(contextBefore.jsStack),
                computeJsDecr(contextBefore.jsDepth),
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern8.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateTop,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              if (callout != null) {
                callout("Missing close '/' for JavaScript RegExp", true);
              }
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "/",
                String.begin,
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateChSet,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateChSet) {
          let match: Match? = pattern30.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new JsEscaperContext(
                jsStateRegx,
                contextBefore.jsAllow,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            );
          }
        }
        if (contextBefore.jsState == jsStateChSet) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              contextBefore,
              before.subsidiary,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            contextBefore,
            before.subsidiary,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class JsContextPropagator extends ContextPropagator<JsEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<JsEscaperContext, HtmlEscaper>,
        literalPart: String?,
        callout: DiagnosticCallout?,
      ): AfterPropagate<JsEscaperContext, HtmlEscaper> {
        jsPropagateContext(before, literalPart, callout)
      }
    }

    let jsContextPropagator = doPure { new JsContextPropagator() };

</details>
