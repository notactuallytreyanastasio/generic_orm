package temper.std.regex;
import temper.core.Core;
import java.util.Objects;
import static temper.std.regex.RegexGlobal.Word;
import static temper.std.regex.RegexGlobal.Digit;
import static temper.std.regex.RegexGlobal.Space;
import temper.core.Nullable;
import static temper.std.regex.RegexGlobal.Dot;
import static temper.std.regex.RegexGlobal.End;
import static temper.std.regex.Codes.tab;
import static temper.std.regex.Codes.dash;
import static temper.std.regex.RegexGlobal.Begin;
import static temper.std.regex.Codes.space;
import static temper.std.regex.Codes.digit9;
import static temper.std.regex.Codes.lowerZ;
import static temper.std.regex.Codes.newline;
import static temper.std.regex.Codes.uint16Max;
import static temper.std.regex.RegexGlobal.WordBoundary;
import static temper.std.regex.Codes.surrogateMin;
import static temper.std.regex.Codes.surrogateMax;
import static temper.std.regex.Codes.carriageReturn;
import static temper.std.regex.Codes.highControlMax;
import static temper.std.regex.RegexGlobal.regexRefs__164;
import static temper.std.regex.Codes.supplementalMin;
import static temper.std.regex.RegexGlobal.escapeNeeds__165;
final class RegexFormatter {
    final StringBuilder out;
    public String format(RegexNode regex__312) {
        this.pushRegex(regex__312);
        return this.out.toString();
    }
    void pushRegex(RegexNode regex__315) {
        Capture t_894;
        CodePoints t_895;
        CodeRange t_896;
        CodeSet t_897;
        Or t_898;
        Repeat t_899;
        Sequence t_900;
        if (regex__315 instanceof Capture) {
            t_894 = Core.cast(Capture.class, regex__315);
            this.pushCapture(t_894);
        } else if (regex__315 instanceof CodePoints) {
            t_895 = Core.cast(CodePoints.class, regex__315);
            this.pushCodePoints(t_895, false);
        } else if (regex__315 instanceof CodeRange) {
            t_896 = Core.cast(CodeRange.class, regex__315);
            this.pushCodeRange(t_896);
        } else if (regex__315 instanceof CodeSet) {
            t_897 = Core.cast(CodeSet.class, regex__315);
            this.pushCodeSet(t_897);
        } else if (regex__315 instanceof Or) {
            t_898 = Core.cast(Or.class, regex__315);
            this.pushOr(t_898);
        } else if (regex__315 instanceof Repeat) {
            t_899 = Core.cast(Repeat.class, regex__315);
            this.pushRepeat(t_899);
        } else if (regex__315 instanceof Sequence) {
            t_900 = Core.cast(Sequence.class, regex__315);
            this.pushSequence(t_900);
        } else if (Objects.equals(regex__315, Begin)) {
            this.out.append("^");
        } else if (Objects.equals(regex__315, Dot)) {
            this.out.append(".");
        } else if (Objects.equals(regex__315, End)) {
            this.out.append("$");
        } else if (Objects.equals(regex__315, WordBoundary)) {
            this.out.append("\\b");
        } else if (Objects.equals(regex__315, Digit)) {
            this.out.append("\\d");
        } else if (Objects.equals(regex__315, Space)) {
            this.out.append("\\s");
        } else if (Objects.equals(regex__315, Word)) {
            this.out.append("\\w");
        }
    }
    void pushCapture(Capture capture__318) {
        this.out.append("(");
        StringBuilder t_868 = this.out;
        String t_1262 = capture__318.getName();
        this.pushCaptureName(t_868, t_1262);
        RegexNode t_1264 = capture__318.getItem();
        this.pushRegex(t_1264);
        this.out.append(")");
    }
    void pushCaptureName(StringBuilder out__321, String name__322) {
        out__321.append("?<" + name__322 + ">");
    }
    void pushCode(int code__325, boolean insideCodeSet__326) {
        boolean t_856;
        boolean t_857;
        String t_858;
        String t_860;
        boolean t_861;
        boolean t_862;
        boolean t_863;
        boolean t_864;
        String t_865;
        fn__327: {
            String specialEscape__328;
            if (code__325 == carriageReturn) {
                specialEscape__328 = "r";
            } else if (code__325 == newline) {
                specialEscape__328 = "n";
            } else if (code__325 == tab) {
                specialEscape__328 = "t";
            } else {
                specialEscape__328 = "";
            }
            if (!specialEscape__328.equals("")) {
                this.out.append("\\");
                this.out.append(specialEscape__328);
                break fn__327;
            }
            if (code__325 <= 127) {
                int escapeNeed__329 = Core.listGet(escapeNeeds__165, code__325);
                if (Objects.equals(escapeNeed__329, 2)) {
                    t_857 = true;
                } else {
                    if (insideCodeSet__326) {
                        t_856 = code__325 == dash;
                    } else {
                        t_856 = false;
                    }
                    t_857 = t_856;
                }
                if (t_857) {
                    this.out.append("\\");
                    t_858 = Core.stringFromCodePoint(code__325);
                    this.out.append(t_858);
                    break fn__327;
                } else if (Objects.equals(escapeNeed__329, 0)) {
                    t_860 = Core.stringFromCodePoint(code__325);
                    this.out.append(t_860);
                    break fn__327;
                }
            }
            if (code__325 >= supplementalMin) {
                t_864 = true;
            } else {
                if (code__325 > highControlMax) {
                    if (surrogateMin <= code__325) {
                        t_861 = code__325 <= surrogateMax;
                    } else {
                        t_861 = false;
                    }
                    if (t_861) {
                        t_862 = true;
                    } else {
                        t_862 = code__325 == uint16Max;
                    }
                    t_863 = !t_862;
                } else {
                    t_863 = false;
                }
                t_864 = t_863;
            }
            if (t_864) {
                t_865 = Core.stringFromCodePoint(code__325);
                this.out.append(t_865);
            } else {
                temper.std.regex.Core.regexFormatterPushCodeTo(this, this.out, code__325, insideCodeSet__326);
            }
        }
    }
    void pushCodePoints(CodePoints codePoints__336, boolean insideCodeSet__337) {
        int t_1249;
        int t_1251;
        String value__339 = codePoints__336.getValue();
        int index__340 = 0;
        while (true) {
            if (!Core.stringHasIndex(value__339, index__340)) {
                break;
            }
            t_1249 = value__339.codePointAt(index__340);
            this.pushCode(t_1249, insideCodeSet__337);
            t_1251 = Core.stringNext(value__339, index__340);
            index__340 = t_1251;
        }
    }
    void pushCodeRange(CodeRange codeRange__342) {
        this.out.append("[");
        this.pushCodeRangeUnwrapped(codeRange__342);
        this.out.append("]");
    }
    void pushCodeRangeUnwrapped(CodeRange codeRange__345) {
        int t_1239 = codeRange__345.getMin();
        this.pushCode(t_1239, true);
        this.out.append("-");
        int t_1242 = codeRange__345.getMax();
        this.pushCode(t_1242, true);
    }
    void pushCodeSet(CodeSet codeSet__348) {
        int t_1233;
        CodePart t_1235;
        CodeSet t_841;
        RegexNode adjusted__350 = this.adjustCodeSet(codeSet__348, regexRefs__164);
        if (adjusted__350 instanceof CodeSet) {
            t_841 = Core.cast(CodeSet.class, adjusted__350);
            if (t_841.getItems().isEmpty()) {
                if (t_841.isNegated()) {
                    this.out.append("[\\s\\S]");
                } else {
                    this.out.append("(?:$.)");
                }
            } else {
                this.out.append("[");
                if (t_841.isNegated()) {
                    this.out.append("^");
                }
                int i__351 = 0;
                while (true) {
                    t_1233 = t_841.getItems().size();
                    if (i__351 >= t_1233) {
                        break;
                    }
                    t_1235 = Core.listGet(t_841.getItems(), i__351);
                    this.pushCodeSetItem(t_1235);
                    i__351 = i__351 + 1;
                }
                this.out.append("]");
            }
        } else {
            this.pushRegex(adjusted__350);
        }
    }
    RegexNode adjustCodeSet(CodeSet codeSet__353, RegexRefs regexRefs__354) {
        return codeSet__353;
    }
    void pushCodeSetItem(CodePart codePart__357) {
        CodePoints t_826;
        CodeRange t_827;
        SpecialSet t_828;
        if (codePart__357 instanceof CodePoints) {
            t_826 = Core.cast(CodePoints.class, codePart__357);
            this.pushCodePoints(t_826, true);
        } else if (codePart__357 instanceof CodeRange) {
            t_827 = Core.cast(CodeRange.class, codePart__357);
            this.pushCodeRangeUnwrapped(t_827);
        } else if (codePart__357 instanceof SpecialSet) {
            t_828 = Core.cast(SpecialSet.class, codePart__357);
            this.pushRegex(t_828);
        }
    }
    void pushOr(Or or__360) {
        RegexNode t_1207;
        int t_1210;
        RegexNode t_1213;
        if (!or__360.getItems().isEmpty()) {
            this.out.append("(?:");
            t_1207 = Core.listGet(or__360.getItems(), 0);
            this.pushRegex(t_1207);
            int i__362 = 1;
            while (true) {
                t_1210 = or__360.getItems().size();
                if (i__362 >= t_1210) {
                    break;
                }
                this.out.append("|");
                t_1213 = Core.listGet(or__360.getItems(), i__362);
                this.pushRegex(t_1213);
                i__362 = i__362 + 1;
            }
            this.out.append(")");
        }
    }
    void pushRepeat(Repeat repeat__364) {
        String t_1195;
        String t_1198;
        boolean t_803;
        boolean t_804;
        boolean t_805;
        this.out.append("(?:");
        RegexNode t_1187 = repeat__364.getItem();
        this.pushRegex(t_1187);
        this.out.append(")");
        int min__366 = repeat__364.getMin();
        @Nullable Integer max__367 = repeat__364.getMax();
        if (min__366 == 0) {
            t_803 = Core.boxedEq(max__367, 1);
        } else {
            t_803 = false;
        }
        if (t_803) {
            this.out.append("?");
        } else {
            if (min__366 == 0) {
                t_804 = max__367 == null;
            } else {
                t_804 = false;
            }
            if (t_804) {
                this.out.append("*");
            } else {
                if (min__366 == 1) {
                    t_805 = max__367 == null;
                } else {
                    t_805 = false;
                }
                if (t_805) {
                    this.out.append("+");
                } else {
                    t_1195 = Integer.toString(min__366);
                    this.out.append("{" + t_1195);
                    if (!Core.boxedEqRev(min__366, max__367)) {
                        this.out.append(",");
                        if (max__367 != null) {
                            t_1198 = Integer.toString(max__367);
                            this.out.append(t_1198);
                        }
                    }
                    this.out.append("}");
                }
            }
        }
        if (repeat__364.isReluctant()) {
            this.out.append("?");
        }
    }
    void pushSequence(Sequence sequence__369) {
        int t_1182;
        RegexNode t_1184;
        int i__371 = 0;
        while (true) {
            t_1182 = sequence__369.getItems().size();
            if (i__371 >= t_1182) {
                break;
            }
            t_1184 = Core.listGet(sequence__369.getItems(), i__371);
            this.pushRegex(t_1184);
            i__371 = i__371 + 1;
        }
    }
    public @Nullable Integer maxCode(CodePart codePart__373) {
        @Nullable Integer return__159;
        int t_1178;
        CodePoints t_791;
        if (codePart__373 instanceof CodePoints) {
            t_791 = Core.cast(CodePoints.class, codePart__373);
            String value__375 = t_791.getValue();
            if (value__375.isEmpty()) {
                return__159 = null;
            } else {
                int max__376 = 0;
                int index__377 = 0;
                while (true) {
                    if (!Core.stringHasIndex(value__375, index__377)) {
                        break;
                    }
                    int next__378 = value__375.codePointAt(index__377);
                    if (next__378 > max__376) {
                        max__376 = next__378;
                    }
                    t_1178 = Core.stringNext(value__375, index__377);
                    index__377 = t_1178;
                }
                return__159 = max__376;
            }
        } else if (codePart__373 instanceof CodeRange) {
            return__159 = Core.cast(CodeRange.class, codePart__373).getMax();
        } else if (Objects.equals(codePart__373, Digit)) {
            return__159 = digit9;
        } else if (Objects.equals(codePart__373, Space)) {
            return__159 = space;
        } else if (Objects.equals(codePart__373, Word)) {
            return__159 = lowerZ;
        } else {
            return__159 = null;
        }
        return return__159;
    }
    public RegexFormatter() {
        StringBuilder t_1172 = new StringBuilder();
        this.out = t_1172;
    }
}
