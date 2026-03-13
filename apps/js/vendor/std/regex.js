import {
  type as type__797, requireInstanceOf as requireInstanceOf__921, pairConstructor as pairConstructor_875, mapConstructor as mapConstructor_874, regexCompileFormatted as regexCompileFormatted_887, regexCompiledFound as regexCompiledFound_891, regexCompiledFind as regexCompiledFind_896, regexCompiledReplace as regexCompiledReplace_901, regexCompiledSplit as regexCompiledSplit_904, listedGet as listedGet_956, stringFromCodePoint as stringFromCodePoint_958, regexFormatterPushCodeTo as regexFormatterPushCodeTo_959, stringGet as stringGet_967, stringNext as stringNext_968, regexFormatterAdjustCodeSet as regexFormatterAdjustCodeSet_982, listBuilderAdd as listBuilderAdd_1098, listBuilderToList as listBuilderToList_1099
} from "@temperlang/core";
export class RegexNode extends type__797() {
  /** @returns {Regex} */
  compiled() {
    return new Regex(this);
  }
  /**
   * @param {string} text_789
   * @returns {boolean}
   */
  found(text_789) {
    return this.compiled().found(text_789);
  }
  /**
   * @param {string} text_791
   * @returns {Match}
   */
  find(text_791) {
    return this.compiled().find(text_791);
  }
  /**
   * @param {string} text_793
   * @param {(arg0: Match) => string} format_794
   * @returns {string}
   */
  replace(text_793, format_794) {
    return this.compiled().replace(text_793, format_794);
  }
  /**
   * @param {string} text_796
   * @returns {Array<string>}
   */
  split(text_796) {
    return this.compiled().split(text_796);
  }
};
export class Capture extends type__797(RegexNode) {
  /** @type {string} */
  #name_798;
  /** @type {RegexNode} */
  #item_799;
  /**
   * @param {{
   *   name: string, item: RegexNode
   * }}
   * props
   * @returns {Capture}
   */
  static["new"](props) {
    return new Capture(props.name, props.item);
  }
  /**
   * @param {string} name_800
   * @param {RegexNode} item_801
   */
  constructor(name_800, item_801) {
    super ();
    this.#name_798 = name_800;
    this.#item_799 = item_801;
    return;
  }
  /** @returns {string} */
  get name() {
    return this.#name_798;
  }
  /** @returns {RegexNode} */
  get item() {
    return this.#item_799;
  }
};
export class CodePart extends type__797(RegexNode) {
};
export class CodePoints extends type__797(CodePart) {
  /** @type {string} */
  #value_804;
  /** @param {string} value_805 */
  constructor(value_805) {
    super ();
    this.#value_804 = value_805;
    return;
  }
  /** @returns {string} */
  get value() {
    return this.#value_804;
  }
};
export class Special extends type__797(RegexNode) {
};
export class SpecialSet extends type__797(CodePart, Special) {
};
export class CodeRange extends type__797(CodePart) {
  /** @type {number} */
  #min_807;
  /** @type {number} */
  #max_808;
  /**
   * @param {{
   *   min: number, max: number
   * }}
   * props
   * @returns {CodeRange}
   */
  static["new"](props) {
    return new CodeRange(props.min, props.max);
  }
  /**
   * @param {number} min_809
   * @param {number} max_810
   */
  constructor(min_809, max_810) {
    super ();
    this.#min_807 = min_809;
    this.#max_808 = max_810;
    return;
  }
  /** @returns {number} */
  get min() {
    return this.#min_807;
  }
  /** @returns {number} */
  get max() {
    return this.#max_808;
  }
};
export class CodeSet extends type__797(RegexNode) {
  /** @type {Array<CodePart>} */
  #items_813;
  /** @type {boolean} */
  #negated_814;
  /**
   * @param {{
   *   items: Array<CodePart>, negated ?: boolean | null
   * }}
   * props
   * @returns {CodeSet}
   */
  static["new"](props) {
    return new CodeSet(props.items, props.negated);
  }
  /**
   * @param {Array<CodePart>} items_815
   * @param {boolean | null} [negated_816]
   */
  constructor(items_815, negated_816) {
    super ();
    let negated_817;
    if (negated_816 == null) {
      negated_817 = false;
    } else {
      negated_817 = negated_816;
    }
    this.#items_813 = items_815;
    this.#negated_814 = negated_817;
    return;
  }
  /** @returns {Array<CodePart>} */
  get items() {
    return this.#items_813;
  }
  /** @returns {boolean} */
  get negated() {
    return this.#negated_814;
  }
};
export class Or extends type__797(RegexNode) {
  /** @type {Array<RegexNode>} */
  #items_820;
  /** @param {Array<RegexNode>} items_821 */
  constructor(items_821) {
    super ();
    this.#items_820 = items_821;
    return;
  }
  /** @returns {Array<RegexNode>} */
  get items() {
    return this.#items_820;
  }
};
export class Repeat extends type__797(RegexNode) {
  /** @type {RegexNode} */
  #item_823;
  /** @type {number} */
  #min_824;
  /** @type {number | null} */
  #max_825;
  /** @type {boolean} */
  #reluctant_826;
  /**
   * @param {{
   *   item: RegexNode, min: number, max: number | null, reluctant ?: boolean | null
   * }}
   * props
   * @returns {Repeat}
   */
  static["new"](props) {
    return new Repeat(props.item, props.min, props.max, props.reluctant);
  }
  /**
   * @param {RegexNode} item_827
   * @param {number} min_828
   * @param {number | null} max_829
   * @param {boolean | null} [reluctant_830]
   */
  constructor(item_827, min_828, max_829, reluctant_830) {
    super ();
    let reluctant_831;
    if (reluctant_830 == null) {
      reluctant_831 = false;
    } else {
      reluctant_831 = reluctant_830;
    }
    this.#item_823 = item_827;
    this.#min_824 = min_828;
    this.#max_825 = max_829;
    this.#reluctant_826 = reluctant_831;
    return;
  }
  /** @returns {RegexNode} */
  get item() {
    return this.#item_823;
  }
  /** @returns {number} */
  get min() {
    return this.#min_824;
  }
  /** @returns {number | null} */
  get max() {
    return this.#max_825;
  }
  /** @returns {boolean} */
  get reluctant() {
    return this.#reluctant_826;
  }
};
export class Sequence extends type__797(RegexNode) {
  /** @type {Array<RegexNode>} */
  #items_836;
  /** @param {Array<RegexNode>} items_837 */
  constructor(items_837) {
    super ();
    this.#items_836 = items_837;
    return;
  }
  /** @returns {Array<RegexNode>} */
  get items() {
    return this.#items_836;
  }
};
export class Match extends type__797() {
  /** @type {Group} */
  #full_839;
  /** @type {Map<string, Group>} */
  #groups_840;
  /**
   * @param {{
   *   full: Group, groups: Map<string, Group>
   * }}
   * props
   * @returns {Match}
   */
  static["new"](props) {
    return new Match(props.full, props.groups);
  }
  /**
   * @param {Group} full_841
   * @param {Map<string, Group>} groups_842
   */
  constructor(full_841, groups_842) {
    super ();
    this.#full_839 = full_841;
    this.#groups_840 = groups_842;
    return;
  }
  /** @returns {Group} */
  get full() {
    return this.#full_839;
  }
  /** @returns {Map<string, Group>} */
  get groups() {
    return this.#groups_840;
  }
};
export class Group extends type__797() {
  /** @type {string} */
  #name_845;
  /** @type {string} */
  #value_846;
  /** @type {globalThis.number} */
  #begin_847;
  /** @type {globalThis.number} */
  #end_848;
  /**
   * @param {{
   *   name: string, value: string, begin: globalThis.number, end: globalThis.number
   * }}
   * props
   * @returns {Group}
   */
  static["new"](props) {
    return new Group(props.name, props.value, props.begin, props.end);
  }
  /**
   * @param {string} name_849
   * @param {string} value_850
   * @param {globalThis.number} begin_851
   * @param {globalThis.number} end_852
   */
  constructor(name_849, value_850, begin_851, end_852) {
    super ();
    this.#name_845 = name_849;
    this.#value_846 = value_850;
    this.#begin_847 = begin_851;
    this.#end_848 = end_852;
    return;
  }
  /** @returns {string} */
  get name() {
    return this.#name_845;
  }
  /** @returns {string} */
  get value() {
    return this.#value_846;
  }
  /** @returns {globalThis.number} */
  get begin() {
    return this.#begin_847;
  }
  /** @returns {globalThis.number} */
  get end() {
    return this.#end_848;
  }
};
class RegexRefs_857 extends type__797() {
  /** @type {CodePoints} */
  #codePoints_858;
  /** @type {Group} */
  #group_859;
  /** @type {Match} */
  #match_860;
  /** @type {Or} */
  #orObject_861;
  /**
   * @param {{
   *   codePoints ?: CodePoints | null, group ?: Group | null, match ?: Match | null, orObject ?: Or | null
   * }}
   * props
   * @returns {RegexRefs_857}
   */
  static["new"](props) {
    return new RegexRefs_857(props.codePoints, props.group, props.match, props.orObject);
  }
  /**
   * @param {CodePoints | null} [codePoints_862]
   * @param {Group | null} [group_863]
   * @param {Match | null} [match_864]
   * @param {Or | null} [orObject_865]
   */
  constructor(codePoints_862, group_863, match_864, orObject_865) {
    super ();
    let t_866;
    let t_867;
    let t_868;
    let t_869;
    let t_870;
    let codePoints_871;
    if (codePoints_862 == null) {
      t_866 = new CodePoints("");
      codePoints_871 = t_866;
    } else {
      codePoints_871 = codePoints_862;
    }
    let group_872;
    if (group_863 == null) {
      t_867 = new Group("", "", 0, 0);
      group_872 = t_867;
    } else {
      group_872 = group_863;
    }
    let match_873;
    if (match_864 == null) {
      t_868 = mapConstructor_874(Object.freeze([pairConstructor_875("", group_872)]));
      t_869 = new Match(group_872, t_868);
      match_873 = t_869;
    } else {
      match_873 = match_864;
    }
    let orObject_876;
    if (orObject_865 == null) {
      t_870 = new Or(Object.freeze([]));
      orObject_876 = t_870;
    } else {
      orObject_876 = orObject_865;
    }
    this.#codePoints_858 = codePoints_871;
    this.#group_859 = group_872;
    this.#match_860 = match_873;
    this.#orObject_861 = orObject_876;
    return;
  }
  /** @returns {CodePoints} */
  get codePoints() {
    return this.#codePoints_858;
  }
  /** @returns {Group} */
  get group() {
    return this.#group_859;
  }
  /** @returns {Match} */
  get match() {
    return this.#match_860;
  }
  /** @returns {Or} */
  get orObject() {
    return this.#orObject_861;
  }
}
export class Regex extends type__797() {
  /** @type {RegexNode} */
  #data_881;
  /** @param {RegexNode} data_882 */
  constructor(data_882) {
    super ();
    const t_883 = data_882;
    this.#data_881 = t_883;
    const formatted_884 = RegexFormatter_885.regexFormat(data_882);
    let t_886 = regexCompileFormatted_887(data_882, formatted_884);
    this.#compiled_888 = t_886;
    return;
  }
  /**
   * @param {string} text_890
   * @returns {boolean}
   */
  found(text_890) {
    return regexCompiledFound_891(this, this.#compiled_888, text_890);
  }
  /**
   * @param {string} text_893
   * @param {globalThis.number | null} [begin_894]
   * @returns {Match}
   */
  find(text_893, begin_894) {
    let begin_895;
    if (begin_894 == null) {
      begin_895 = 0;
    } else {
      begin_895 = begin_894;
    }
    return regexCompiledFind_896(this, this.#compiled_888, text_893, begin_895, regexRefs_897);
  }
  /**
   * @param {string} text_899
   * @param {(arg0: Match) => string} format_900
   * @returns {string}
   */
  replace(text_899, format_900) {
    return regexCompiledReplace_901(this, this.#compiled_888, text_899, format_900, regexRefs_897);
  }
  /**
   * @param {string} text_903
   * @returns {Array<string>}
   */
  split(text_903) {
    return regexCompiledSplit_904(this, this.#compiled_888, text_903, regexRefs_897);
  }
  /** @type {unknown} */
  #compiled_888;
  /** @returns {RegexNode} */
  get data() {
    return this.#data_881;
  }
};
class RegexFormatter_885 extends type__797() {
  /** @type {globalThis.Array<string>} */
  #out_906;
  /**
   * @param {RegexNode} data_908
   * @returns {string}
   */
  static regexFormat(data_908) {
    return new RegexFormatter_885().format(data_908);
  }
  /**
   * @param {RegexNode} regex_910
   * @returns {string}
   */
  format(regex_910) {
    this.#pushRegex_911(regex_910);
    return this.#out_906[0];
  }
  /** @param {RegexNode} regex_913 */
  #pushRegex_911(regex_913) {
    let t_914;
    let t_915;
    let t_916;
    let t_917;
    let t_918;
    let t_919;
    let t_920;
    if (regex_913 instanceof Capture) {
      t_914 = requireInstanceOf__921(regex_913, Capture);
      this.#pushCapture_922(t_914);
    } else if (regex_913 instanceof CodePoints) {
      t_915 = requireInstanceOf__921(regex_913, CodePoints);
      this.#pushCodePoints_923(t_915, false);
    } else if (regex_913 instanceof CodeRange) {
      t_916 = requireInstanceOf__921(regex_913, CodeRange);
      this.#pushCodeRange_924(t_916);
    } else if (regex_913 instanceof CodeSet) {
      t_917 = requireInstanceOf__921(regex_913, CodeSet);
      this.#pushCodeSet_925(t_917);
    } else if (regex_913 instanceof Or) {
      t_918 = requireInstanceOf__921(regex_913, Or);
      this.#pushOr_926(t_918);
    } else if (regex_913 instanceof Repeat) {
      t_919 = requireInstanceOf__921(regex_913, Repeat);
      this.#pushRepeat_927(t_919);
    } else if (regex_913 instanceof Sequence) {
      t_920 = requireInstanceOf__921(regex_913, Sequence);
      this.#pushSequence_928(t_920);
    } else if (Object.is(regex_913, Begin)) {
      this.#out_906[0] += "^";
    } else if (Object.is(regex_913, Dot)) {
      this.#out_906[0] += ".";
    } else if (Object.is(regex_913, End)) {
      this.#out_906[0] += "$";
    } else if (Object.is(regex_913, WordBoundary)) {
      this.#out_906[0] += "\\b";
    } else if (Object.is(regex_913, Digit)) {
      this.#out_906[0] += "\\d";
    } else if (Object.is(regex_913, Space)) {
      this.#out_906[0] += "\\s";
    } else if (Object.is(regex_913, Word)) {
      this.#out_906[0] += "\\w";
    }
    return;
  }
  /** @param {Capture} capture_930 */
  #pushCapture_922(capture_930) {
    this.#out_906[0] += "(";
    let t_931 = this.#out_906;
    let t_932 = capture_930.name;
    this.#pushCaptureName_933(t_931, t_932);
    let t_934 = capture_930.item;
    this.#pushRegex_911(t_934);
    this.#out_906[0] += ")";
    return;
  }
  /**
   * @param {globalThis.Array<string>} out_936
   * @param {string} name_937
   */
  #pushCaptureName_933(out_936, name_937) {
    out_936[0] += "?<" + name_937 + ">";
    return;
  }
  /**
   * @param {number} code_940
   * @param {boolean} insideCodeSet_941
   */
  #pushCode_939(code_940, insideCodeSet_941) {
    let return_942;
    let t_943;
    let t_944;
    let t_945;
    let t_946;
    let t_947;
    let t_948;
    let t_949;
    let t_950;
    let t_951;
    fn_952: {
      try {
        let specialEscape_953;
        if (code_940 === Codes_954.carriageReturn) {
          specialEscape_953 = "r";
        } else if (code_940 === Codes_954.newline) {
          specialEscape_953 = "n";
        } else if (code_940 === Codes_954.tab) {
          specialEscape_953 = "t";
        } else {
          specialEscape_953 = "";
        }
        if (specialEscape_953 !== "") {
          this.#out_906[0] += "\\";
          this.#out_906[0] += specialEscape_953;
          return_942 = void 0;
          break fn_952;
        }
        if (code_940 <= 127) {
          const escapeNeed_955 = listedGet_956(escapeNeeds_957, code_940);
          if (Object.is(escapeNeed_955, 2)) {
            t_944 = true;
          } else {
            if (insideCodeSet_941) {
              t_943 = code_940 === Codes_954.dash;
            } else {
              t_943 = false;
            }
            t_944 = t_943;
          }
          if (t_944) {
            this.#out_906[0] += "\\";
            t_945 = stringFromCodePoint_958(code_940);
            this.#out_906[0] += t_945;
            return_942 = void 0;
            break fn_952;
          } else if (Object.is(escapeNeed_955, 0)) {
            t_946 = stringFromCodePoint_958(code_940);
            this.#out_906[0] += t_946;
            return_942 = void 0;
            break fn_952;
          }
        }
        if (code_940 >= Codes_954.supplementalMin) {
          t_950 = true;
        } else {
          if (code_940 > Codes_954.highControlMax) {
            if (Codes_954.surrogateMin <= code_940) {
              t_947 = code_940 <= Codes_954.surrogateMax;
            } else {
              t_947 = false;
            }
            if (t_947) {
              t_948 = true;
            } else {
              t_948 = code_940 === Codes_954.uint16Max;
            }
            t_949 = ! t_948;
          } else {
            t_949 = false;
          }
          t_950 = t_949;
        }
        if (t_950) {
          t_951 = stringFromCodePoint_958(code_940);
          this.#out_906[0] += t_951;
        } else {
          regexFormatterPushCodeTo_959(this, this.#out_906, code_940, insideCodeSet_941);
        }
      } catch {
        throw Error();
      }
      return_942 = void 0;
    }
    return return_942;
  }
  /**
   * @param {CodePoints} codePoints_961
   * @param {boolean} insideCodeSet_962
   */
  #pushCodePoints_923(codePoints_961, insideCodeSet_962) {
    let t_963;
    let t_964;
    const value_965 = codePoints_961.value;
    let index_966 = 0;
    while (true) {
      if (!(value_965.length > index_966)) {
        break;
      }
      t_963 = stringGet_967(value_965, index_966);
      this.#pushCode_939(t_963, insideCodeSet_962);
      t_964 = stringNext_968(value_965, index_966);
      index_966 = t_964;
    }
    return;
  }
  /** @param {CodeRange} codeRange_970 */
  #pushCodeRange_924(codeRange_970) {
    this.#out_906[0] += "[";
    this.#pushCodeRangeUnwrapped_971(codeRange_970);
    this.#out_906[0] += "]";
    return;
  }
  /** @param {CodeRange} codeRange_973 */
  #pushCodeRangeUnwrapped_971(codeRange_973) {
    let t_974 = codeRange_973.min;
    this.#pushCode_939(t_974, true);
    this.#out_906[0] += "-";
    let t_975 = codeRange_973.max;
    this.#pushCode_939(t_975, true);
    return;
  }
  /** @param {CodeSet} codeSet_977 */
  #pushCodeSet_925(codeSet_977) {
    let t_978;
    let t_979;
    let t_980;
    const adjusted_981 = regexFormatterAdjustCodeSet_982(this, codeSet_977, regexRefs_897);
    if (adjusted_981 instanceof CodeSet) {
      t_980 = requireInstanceOf__921(adjusted_981, CodeSet);
      if (! t_980.items.length) {
        if (t_980.negated) {
          this.#out_906[0] += "[\\s\\S]";
        } else {
          this.#out_906[0] += "(?:$.)";
        }
      } else {
        this.#out_906[0] += "[";
        if (t_980.negated) {
          this.#out_906[0] += "^";
        }
        let i_983 = 0;
        while (true) {
          t_978 = t_980.items.length;
          if (!(i_983 < t_978)) {
            break;
          }
          t_979 = listedGet_956(t_980.items, i_983);
          this.#pushCodeSetItem_984(t_979);
          i_983 = i_983 + 1 | 0;
        }
        this.#out_906[0] += "]";
      }
    } else {
      this.#pushRegex_911(adjusted_981);
    }
    return;
  }
  /** @param {CodePart} codePart_986 */
  #pushCodeSetItem_984(codePart_986) {
    let t_987;
    let t_988;
    let t_989;
    if (codePart_986 instanceof CodePoints) {
      t_987 = requireInstanceOf__921(codePart_986, CodePoints);
      this.#pushCodePoints_923(t_987, true);
    } else if (codePart_986 instanceof CodeRange) {
      t_988 = requireInstanceOf__921(codePart_986, CodeRange);
      this.#pushCodeRangeUnwrapped_971(t_988);
    } else if (codePart_986 instanceof SpecialSet) {
      t_989 = requireInstanceOf__921(codePart_986, SpecialSet);
      this.#pushRegex_911(t_989);
    }
    return;
  }
  /** @param {Or} or_991 */
  #pushOr_926(or_991) {
    let t_992;
    let t_993;
    let t_994;
    if (! ! or_991.items.length) {
      this.#out_906[0] += "(?:";
      t_992 = listedGet_956(or_991.items, 0);
      this.#pushRegex_911(t_992);
      let i_995 = 1;
      while (true) {
        t_993 = or_991.items.length;
        if (!(i_995 < t_993)) {
          break;
        }
        this.#out_906[0] += "|";
        t_994 = listedGet_956(or_991.items, i_995);
        this.#pushRegex_911(t_994);
        i_995 = i_995 + 1 | 0;
      }
      this.#out_906[0] += ")";
    }
    return;
  }
  /** @param {Repeat} repeat_997 */
  #pushRepeat_927(repeat_997) {
    let t_998;
    let t_999;
    let t_1000;
    let t_1001;
    let t_1002;
    this.#out_906[0] += "(?:";
    let t_1003 = repeat_997.item;
    this.#pushRegex_911(t_1003);
    this.#out_906[0] += ")";
    const min_1004 = repeat_997.min;
    const max_1005 = repeat_997.max;
    if (min_1004 === 0) {
      t_1000 = max_1005 === 1;
    } else {
      t_1000 = false;
    }
    if (t_1000) {
      this.#out_906[0] += "?";
    } else {
      if (min_1004 === 0) {
        t_1001 = max_1005 == null;
      } else {
        t_1001 = false;
      }
      if (t_1001) {
        this.#out_906[0] += "*";
      } else {
        if (min_1004 === 1) {
          t_1002 = max_1005 == null;
        } else {
          t_1002 = false;
        }
        if (t_1002) {
          this.#out_906[0] += "+";
        } else {
          t_998 = min_1004.toString();
          this.#out_906[0] += "{" + t_998;
          if (min_1004 !== max_1005) {
            this.#out_906[0] += ",";
            if (!(max_1005 == null)) {
              t_999 = max_1005.toString();
              this.#out_906[0] += t_999;
            }
          }
          this.#out_906[0] += "}";
        }
      }
    }
    if (repeat_997.reluctant) {
      this.#out_906[0] += "?";
    }
    return;
  }
  /** @param {Sequence} sequence_1007 */
  #pushSequence_928(sequence_1007) {
    let t_1008;
    let t_1009;
    let i_1010 = 0;
    while (true) {
      t_1008 = sequence_1007.items.length;
      if (!(i_1010 < t_1008)) {
        break;
      }
      t_1009 = listedGet_956(sequence_1007.items, i_1010);
      this.#pushRegex_911(t_1009);
      i_1010 = i_1010 + 1 | 0;
    }
    return;
  }
  /**
   * @param {CodePart} codePart_1012
   * @returns {number | null}
   */
  maxCode(codePart_1012) {
    let return_1013;
    let t_1014;
    let t_1015;
    if (codePart_1012 instanceof CodePoints) {
      t_1015 = requireInstanceOf__921(codePart_1012, CodePoints);
      const value_1016 = t_1015.value;
      if (! value_1016) {
        return_1013 = null;
      } else {
        let max_1017 = 0;
        let index_1018 = 0;
        while (true) {
          if (!(value_1016.length > index_1018)) {
            break;
          }
          const next_1019 = stringGet_967(value_1016, index_1018);
          if (next_1019 > max_1017) {
            max_1017 = next_1019;
          }
          t_1014 = stringNext_968(value_1016, index_1018);
          index_1018 = t_1014;
        }
        return_1013 = max_1017;
      }
    } else if (codePart_1012 instanceof CodeRange) {
      return_1013 = requireInstanceOf__921(codePart_1012, CodeRange).max;
    } else if (Object.is(codePart_1012, Digit)) {
      return_1013 = Codes_954.digit9;
    } else if (Object.is(codePart_1012, Space)) {
      return_1013 = Codes_954.space;
    } else if (Object.is(codePart_1012, Word)) {
      return_1013 = Codes_954.lowerZ;
    } else {
      return_1013 = null;
    }
    return return_1013;
  }
  constructor() {
    super ();
    let t_1020 = [""];
    this.#out_906 = t_1020;
    return;
  }
}
class Codes_954 extends type__797() {
  /** @type {number} */
  static #ampersand_1021 = 38;
  /** @returns {number} */
  static get ampersand() {
    return this.#ampersand_1021;
  }
  /** @type {number} */
  static #backslash_1022 = 92;
  /** @returns {number} */
  static get backslash() {
    return this.#backslash_1022;
  }
  /** @type {number} */
  static #caret_1023 = 94;
  /** @returns {number} */
  static get caret() {
    return this.#caret_1023;
  }
  /** @type {number} */
  static #carriageReturn_1024 = 13;
  /** @returns {number} */
  static get carriageReturn() {
    return this.#carriageReturn_1024;
  }
  /** @type {number} */
  static #curlyLeft_1025 = 123;
  /** @returns {number} */
  static get curlyLeft() {
    return this.#curlyLeft_1025;
  }
  /** @type {number} */
  static #curlyRight_1026 = 125;
  /** @returns {number} */
  static get curlyRight() {
    return this.#curlyRight_1026;
  }
  /** @type {number} */
  static #dash_1027 = 45;
  /** @returns {number} */
  static get dash() {
    return this.#dash_1027;
  }
  /** @type {number} */
  static #dot_1028 = 46;
  /** @returns {number} */
  static get dot() {
    return this.#dot_1028;
  }
  /** @type {number} */
  static #highControlMin_1029 = 127;
  /** @returns {number} */
  static get highControlMin() {
    return this.#highControlMin_1029;
  }
  /** @type {number} */
  static #highControlMax_1030 = 159;
  /** @returns {number} */
  static get highControlMax() {
    return this.#highControlMax_1030;
  }
  /** @type {number} */
  static #digit0_1031 = 48;
  /** @returns {number} */
  static get digit0() {
    return this.#digit0_1031;
  }
  /** @type {number} */
  static #digit9_1032 = 57;
  /** @returns {number} */
  static get digit9() {
    return this.#digit9_1032;
  }
  /** @type {number} */
  static #lowerA_1033 = 97;
  /** @returns {number} */
  static get lowerA() {
    return this.#lowerA_1033;
  }
  /** @type {number} */
  static #lowerZ_1034 = 122;
  /** @returns {number} */
  static get lowerZ() {
    return this.#lowerZ_1034;
  }
  /** @type {number} */
  static #newline_1035 = 10;
  /** @returns {number} */
  static get newline() {
    return this.#newline_1035;
  }
  /** @type {number} */
  static #peso_1036 = 36;
  /** @returns {number} */
  static get peso() {
    return this.#peso_1036;
  }
  /** @type {number} */
  static #pipe_1037 = 124;
  /** @returns {number} */
  static get pipe() {
    return this.#pipe_1037;
  }
  /** @type {number} */
  static #plus_1038 = 43;
  /** @returns {number} */
  static get plus() {
    return this.#plus_1038;
  }
  /** @type {number} */
  static #question_1039 = 63;
  /** @returns {number} */
  static get question() {
    return this.#question_1039;
  }
  /** @type {number} */
  static #roundLeft_1040 = 40;
  /** @returns {number} */
  static get roundLeft() {
    return this.#roundLeft_1040;
  }
  /** @type {number} */
  static #roundRight_1041 = 41;
  /** @returns {number} */
  static get roundRight() {
    return this.#roundRight_1041;
  }
  /** @type {number} */
  static #slash_1042 = 47;
  /** @returns {number} */
  static get slash() {
    return this.#slash_1042;
  }
  /** @type {number} */
  static #squareLeft_1043 = 91;
  /** @returns {number} */
  static get squareLeft() {
    return this.#squareLeft_1043;
  }
  /** @type {number} */
  static #squareRight_1044 = 93;
  /** @returns {number} */
  static get squareRight() {
    return this.#squareRight_1044;
  }
  /** @type {number} */
  static #star_1045 = 42;
  /** @returns {number} */
  static get star() {
    return this.#star_1045;
  }
  /** @type {number} */
  static #tab_1046 = 9;
  /** @returns {number} */
  static get tab() {
    return this.#tab_1046;
  }
  /** @type {number} */
  static #tilde_1047 = 42;
  /** @returns {number} */
  static get tilde() {
    return this.#tilde_1047;
  }
  /** @type {number} */
  static #upperA_1048 = 65;
  /** @returns {number} */
  static get upperA() {
    return this.#upperA_1048;
  }
  /** @type {number} */
  static #upperZ_1049 = 90;
  /** @returns {number} */
  static get upperZ() {
    return this.#upperZ_1049;
  }
  /** @type {number} */
  static #space_1050 = 32;
  /** @returns {number} */
  static get space() {
    return this.#space_1050;
  }
  /** @type {number} */
  static #surrogateMin_1051 = 55296;
  /** @returns {number} */
  static get surrogateMin() {
    return this.#surrogateMin_1051;
  }
  /** @type {number} */
  static #surrogateMax_1052 = 57343;
  /** @returns {number} */
  static get surrogateMax() {
    return this.#surrogateMax_1052;
  }
  /** @type {number} */
  static #supplementalMin_1053 = 65536;
  /** @returns {number} */
  static get supplementalMin() {
    return this.#supplementalMin_1053;
  }
  /** @type {number} */
  static #uint16Max_1054 = 65535;
  /** @returns {number} */
  static get uint16Max() {
    return this.#uint16Max_1054;
  }
  /** @type {number} */
  static #underscore_1055 = 95;
  /** @returns {number} */
  static get underscore() {
    return this.#underscore_1055;
  }
  constructor() {
    super ();
    return;
  }
}
class Begin_1056 extends type__797(Special) {
  constructor() {
    super ();
    return;
  }
}
/** @type {Special} */
const return_1057 = new Begin_1056();
/** @type {Special} */
export const Begin = return_1057;
class Dot_1058 extends type__797(Special) {
  constructor() {
    super ();
    return;
  }
}
/** @type {Special} */
const return_1059 = new Dot_1058();
/** @type {Special} */
export const Dot = return_1059;
class End_1060 extends type__797(Special) {
  constructor() {
    super ();
    return;
  }
}
/** @type {Special} */
const return_1061 = new End_1060();
/** @type {Special} */
export const End = return_1061;
class WordBoundary_1062 extends type__797(Special) {
  constructor() {
    super ();
    return;
  }
}
/** @type {Special} */
const return_1063 = new WordBoundary_1062();
/** @type {Special} */
export const WordBoundary = return_1063;
class Digit_1064 extends type__797(SpecialSet) {
  constructor() {
    super ();
    return;
  }
}
/** @type {SpecialSet} */
const return_1065 = new Digit_1064();
/** @type {SpecialSet} */
export const Digit = return_1065;
class Space_1066 extends type__797(SpecialSet) {
  constructor() {
    super ();
    return;
  }
}
/** @type {SpecialSet} */
const return_1067 = new Space_1066();
/** @type {SpecialSet} */
export const Space = return_1067;
class Word_1068 extends type__797(SpecialSet) {
  constructor() {
    super ();
    return;
  }
}
/** @type {SpecialSet} */
const return_1069 = new Word_1068();
/** @type {SpecialSet} */
export const Word = return_1069;
/** @returns {Array<number>} */
function buildEscapeNeeds_1070() {
  let t_1071;
  let t_1072;
  let t_1073;
  let t_1074;
  let t_1075;
  let t_1076;
  let t_1077;
  let t_1078;
  let t_1079;
  let t_1080;
  let t_1081;
  let t_1082;
  let t_1083;
  let t_1084;
  let t_1085;
  let t_1086;
  let t_1087;
  let t_1088;
  let t_1089;
  let t_1090;
  let t_1091;
  let t_1092;
  let t_1093;
  let t_1094;
  let t_1095;
  const escapeNeeds_1096 = [];
  let code_1097 = 0;
  while (code_1097 <= 127) {
    if (code_1097 === Codes_954.dash) {
      t_1078 = true;
    } else {
      if (code_1097 === Codes_954.space) {
        t_1077 = true;
      } else {
        if (code_1097 === Codes_954.underscore) {
          t_1076 = true;
        } else {
          if (Codes_954.digit0 <= code_1097) {
            t_1071 = code_1097 <= Codes_954.digit9;
          } else {
            t_1071 = false;
          }
          if (t_1071) {
            t_1075 = true;
          } else {
            if (Codes_954.upperA <= code_1097) {
              t_1072 = code_1097 <= Codes_954.upperZ;
            } else {
              t_1072 = false;
            }
            if (t_1072) {
              t_1074 = true;
            } else {
              if (Codes_954.lowerA <= code_1097) {
                t_1073 = code_1097 <= Codes_954.lowerZ;
              } else {
                t_1073 = false;
              }
              t_1074 = t_1073;
            }
            t_1075 = t_1074;
          }
          t_1076 = t_1075;
        }
        t_1077 = t_1076;
      }
      t_1078 = t_1077;
    }
    if (t_1078) {
      t_1095 = 0;
    } else {
      if (code_1097 === Codes_954.ampersand) {
        t_1094 = true;
      } else {
        if (code_1097 === Codes_954.backslash) {
          t_1093 = true;
        } else {
          if (code_1097 === Codes_954.caret) {
            t_1092 = true;
          } else {
            if (code_1097 === Codes_954.curlyLeft) {
              t_1091 = true;
            } else {
              if (code_1097 === Codes_954.curlyRight) {
                t_1090 = true;
              } else {
                if (code_1097 === Codes_954.dot) {
                  t_1089 = true;
                } else {
                  if (code_1097 === Codes_954.peso) {
                    t_1088 = true;
                  } else {
                    if (code_1097 === Codes_954.pipe) {
                      t_1087 = true;
                    } else {
                      if (code_1097 === Codes_954.plus) {
                        t_1086 = true;
                      } else {
                        if (code_1097 === Codes_954.question) {
                          t_1085 = true;
                        } else {
                          if (code_1097 === Codes_954.roundLeft) {
                            t_1084 = true;
                          } else {
                            if (code_1097 === Codes_954.roundRight) {
                              t_1083 = true;
                            } else {
                              if (code_1097 === Codes_954.slash) {
                                t_1082 = true;
                              } else {
                                if (code_1097 === Codes_954.squareLeft) {
                                  t_1081 = true;
                                } else {
                                  if (code_1097 === Codes_954.squareRight) {
                                    t_1080 = true;
                                  } else {
                                    if (code_1097 === Codes_954.star) {
                                      t_1079 = true;
                                    } else {
                                      t_1079 = code_1097 === Codes_954.tilde;
                                    }
                                    t_1080 = t_1079;
                                  }
                                  t_1081 = t_1080;
                                }
                                t_1082 = t_1081;
                              }
                              t_1083 = t_1082;
                            }
                            t_1084 = t_1083;
                          }
                          t_1085 = t_1084;
                        }
                        t_1086 = t_1085;
                      }
                      t_1087 = t_1086;
                    }
                    t_1088 = t_1087;
                  }
                  t_1089 = t_1088;
                }
                t_1090 = t_1089;
              }
              t_1091 = t_1090;
            }
            t_1092 = t_1091;
          }
          t_1093 = t_1092;
        }
        t_1094 = t_1093;
      }
      if (t_1094) {
        t_1095 = 2;
      } else {
        t_1095 = 1;
      }
    }
    listBuilderAdd_1098(escapeNeeds_1096, t_1095);
    code_1097 = code_1097 + 1 | 0;
  }
  return listBuilderToList_1099(escapeNeeds_1096);
}
/** @type {Array<number>} */
const escapeNeeds_957 = buildEscapeNeeds_1070();
/** @type {RegexRefs_857} */
const regexRefs_897 = new RegexRefs_857();
/**
 * @param {RegexNode} item_1100
 * @returns {RegexNode}
 */
export function entire(item_1100) {
  return new Sequence(Object.freeze([Begin, item_1100, End]));
};
/**
 * @param {RegexNode} item_1101
 * @param {boolean | null} [reluctant_1102]
 * @returns {Repeat}
 */
export function oneOrMore(item_1101, reluctant_1102) {
  let reluctant_1103;
  if (reluctant_1102 == null) {
    reluctant_1103 = false;
  } else {
    reluctant_1103 = reluctant_1102;
  }
  return new Repeat(item_1101, 1, null, reluctant_1103);
};
/**
 * @param {RegexNode} item_1104
 * @param {boolean | null} [reluctant_1105]
 * @returns {Repeat}
 */
export function optional(item_1104, reluctant_1105) {
  let reluctant_1106;
  if (reluctant_1105 == null) {
    reluctant_1106 = false;
  } else {
    reluctant_1106 = reluctant_1105;
  }
  return new Repeat(item_1104, 0, 1, reluctant_1106);
};
