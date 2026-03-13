using S = System;
using T = System.Text;
using C = TemperLang.Core;
using R = TemperLang.Std.Regex;
namespace TemperLang.Std.Regex
{
    class RegexFormatter
    {
        readonly T::StringBuilder out__303;
        public static string RegexFormat(IRegexNode data__309)
        {
            return new RegexFormatter().Format(data__309);
        }
        public string Format(IRegexNode regex__312)
        {
            this.PushRegex(regex__312);
            return this.out__303.ToString();
        }
        void PushRegex(IRegexNode regex__315)
        {
            Capture t___894;
            CodePoints t___895;
            CodeRange t___896;
            CodeSet t___897;
            Or t___898;
            Repeat t___899;
            Sequence t___900;
            if (regex__315 is Capture)
            {
                t___894 = (Capture) regex__315;
                this.PushCapture(t___894);
            }
            else if (regex__315 is CodePoints)
            {
                t___895 = (CodePoints) regex__315;
                this.PushCodePoints(t___895, false);
            }
            else if (regex__315 is CodeRange)
            {
                t___896 = (CodeRange) regex__315;
                this.PushCodeRange(t___896);
            }
            else if (regex__315 is CodeSet)
            {
                t___897 = (CodeSet) regex__315;
                this.PushCodeSet(t___897);
            }
            else if (regex__315 is Or)
            {
                t___898 = (Or) regex__315;
                this.PushOr(t___898);
            }
            else if (regex__315 is Repeat)
            {
                t___899 = (Repeat) regex__315;
                this.PushRepeat(t___899);
            }
            else if (regex__315 is Sequence)
            {
                t___900 = (Sequence) regex__315;
                this.PushSequence(t___900);
            }
            else if (regex__315 == R::RegexGlobal.Begin) this.out__303.Append("^");
            else if (regex__315 == R::RegexGlobal.Dot) this.out__303.Append(".");
            else if (regex__315 == R::RegexGlobal.End) this.out__303.Append("\u0024");
            else if (regex__315 == R::RegexGlobal.WordBoundary) this.out__303.Append("\\b");
            else if (regex__315 == R::RegexGlobal.Digit) this.out__303.Append("\\d");
            else if (regex__315 == R::RegexGlobal.Space) this.out__303.Append("\\s");
            else if (regex__315 == R::RegexGlobal.Word) this.out__303.Append("\\w");
        }
        void PushCapture(Capture capture__318)
        {
            this.out__303.Append("(");
            T::StringBuilder t___868 = this.out__303;
            string t___1262 = capture__318.Name;
            this.PushCaptureName(t___868, t___1262);
            IRegexNode t___1264 = capture__318.Item;
            this.PushRegex(t___1264);
            this.out__303.Append(")");
        }
        void PushCaptureName(T::StringBuilder out__321, string name__322)
        {
            out__321.Append("?<" + name__322 + ">");
        }
        void PushCode(int code__325, bool insideCodeSet__326)
        {
            bool t___856;
            bool t___857;
            string t___858;
            string t___860;
            bool t___861;
            bool t___862;
            bool t___863;
            bool t___864;
            string t___865;
            {
                {
                    string specialEscape__328;
                    if (code__325 == Codes.CarriageReturn)
                    {
                        specialEscape__328 = "r";
                    }
                    else if (code__325 == Codes.Newline)
                    {
                        specialEscape__328 = "n";
                    }
                    else if (code__325 == Codes.Tab)
                    {
                        specialEscape__328 = "t";
                    }
                    else
                    {
                        specialEscape__328 = "";
                    }
                    if (specialEscape__328 != "")
                    {
                        this.out__303.Append("\\");
                        this.out__303.Append(specialEscape__328);
                        goto fn__327;
                    }
                    if (code__325 <= 127)
                    {
                        int escapeNeed__329 = R::RegexGlobal.escapeNeeds__165[code__325];
                        if (escapeNeed__329 == 2)
                        {
                            t___857 = true;
                        }
                        else
                        {
                            if (insideCodeSet__326)
                            {
                                t___856 = code__325 == Codes.Dash;
                            }
                            else
                            {
                                t___856 = false;
                            }
                            t___857 = t___856;
                        }
                        if (t___857)
                        {
                            this.out__303.Append("\\");
                            t___858 = C::Core.StringFromCodePoint(code__325);
                            this.out__303.Append(t___858);
                            goto fn__327;
                        }
                        else if (escapeNeed__329 == 0)
                        {
                            t___860 = C::Core.StringFromCodePoint(code__325);
                            this.out__303.Append(t___860);
                            goto fn__327;
                        }
                    }
                    if (code__325 >= Codes.SupplementalMin)
                    {
                        t___864 = true;
                    }
                    else
                    {
                        if (code__325 > Codes.HighControlMax)
                        {
                            if (Codes.SurrogateMin <= code__325)
                            {
                                t___861 = code__325 <= Codes.SurrogateMax;
                            }
                            else
                            {
                                t___861 = false;
                            }
                            if (t___861)
                            {
                                t___862 = true;
                            }
                            else
                            {
                                t___862 = code__325 == Codes.Uint16_Max;
                            }
                            t___863 = !t___862;
                        }
                        else
                        {
                            t___863 = false;
                        }
                        t___864 = t___863;
                    }
                    if (t___864)
                    {
                        t___865 = C::Core.StringFromCodePoint(code__325);
                        this.out__303.Append(t___865);
                    }
                    else R::RegexSupport.PushCodeTo(this, this.out__303, code__325, insideCodeSet__326);
                }
                fn__327:
                {
                }
            }
        }
        void PushCodePoints(CodePoints codePoints__336, bool insideCodeSet__337)
        {
            int t___1249;
            int t___1251;
            string value__339 = codePoints__336.Value;
            int index__340 = 0;
            while (true)
            {
                if (!C::StringUtil.HasIndex(value__339, index__340)) break;
                t___1249 = C::StringUtil.Get(value__339, index__340);
                this.PushCode(t___1249, insideCodeSet__337);
                t___1251 = C::StringUtil.Next(value__339, index__340);
                index__340 = t___1251;
            }
        }
        void PushCodeRange(CodeRange codeRange__342)
        {
            this.out__303.Append("[");
            this.PushCodeRangeUnwrapped(codeRange__342);
            this.out__303.Append("]");
        }
        void PushCodeRangeUnwrapped(CodeRange codeRange__345)
        {
            int t___1239 = codeRange__345.Min;
            this.PushCode(t___1239, true);
            this.out__303.Append("-");
            int t___1242 = codeRange__345.Max;
            this.PushCode(t___1242, true);
        }
        void PushCodeSet(CodeSet codeSet__348)
        {
            int t___1233;
            ICodePart t___1235;
            CodeSet t___841;
            IRegexNode adjusted__350 = R::RegexSupport.AdjustCodeSet(this, codeSet__348, R::RegexGlobal.regexRefs__164);
            if (adjusted__350 is CodeSet)
            {
                t___841 = (CodeSet) adjusted__350;
                if (t___841.Items.Count == 0) if (t___841.Negated) this.out__303.Append("[\\s\\S]");
                else this.out__303.Append("(?:\u0024.)");
                else
                {
                    this.out__303.Append("[");
                    if (t___841.Negated) this.out__303.Append("^");
                    int i__351 = 0;
                    while (true)
                    {
                        t___1233 = t___841.Items.Count;
                        if (!(i__351 < t___1233)) break;
                        t___1235 = t___841.Items[i__351];
                        this.PushCodeSetItem(t___1235);
                        i__351 = i__351 + 1;
                    }
                    this.out__303.Append("]");
                }
            }
            else this.PushRegex(adjusted__350);
        }
        void PushCodeSetItem(ICodePart codePart__357)
        {
            CodePoints t___826;
            CodeRange t___827;
            ISpecialSet t___828;
            if (codePart__357 is CodePoints)
            {
                t___826 = (CodePoints) codePart__357;
                this.PushCodePoints(t___826, true);
            }
            else if (codePart__357 is CodeRange)
            {
                t___827 = (CodeRange) codePart__357;
                this.PushCodeRangeUnwrapped(t___827);
            }
            else if (codePart__357 is ISpecialSet)
            {
                t___828 = (ISpecialSet) codePart__357;
                this.PushRegex(t___828);
            }
        }
        void PushOr(Or or__360)
        {
            IRegexNode t___1207;
            int t___1210;
            IRegexNode t___1213;
            if (!(or__360.Items.Count == 0))
            {
                this.out__303.Append("(?:");
                t___1207 = or__360.Items[0];
                this.PushRegex(t___1207);
                int i__362 = 1;
                while (true)
                {
                    t___1210 = or__360.Items.Count;
                    if (!(i__362 < t___1210)) break;
                    this.out__303.Append("|");
                    t___1213 = or__360.Items[i__362];
                    this.PushRegex(t___1213);
                    i__362 = i__362 + 1;
                }
                this.out__303.Append(")");
            }
        }
        void PushRepeat(Repeat repeat__364)
        {
            string t___1195;
            string t___1198;
            bool t___803;
            bool t___804;
            bool t___805;
            this.out__303.Append("(?:");
            IRegexNode t___1187 = repeat__364.Item;
            this.PushRegex(t___1187);
            this.out__303.Append(")");
            int min__366 = repeat__364.Min;
            int ? max__367 = repeat__364.Max;
            if (min__366 == 0)
            {
                t___803 = max__367 == 1;
            }
            else
            {
                t___803 = false;
            }
            if (t___803) this.out__303.Append("?");
            else
            {
                if (min__366 == 0)
                {
                    t___804 = max__367 == null;
                }
                else
                {
                    t___804 = false;
                }
                if (t___804) this.out__303.Append("*");
                else
                {
                    if (min__366 == 1)
                    {
                        t___805 = max__367 == null;
                    }
                    else
                    {
                        t___805 = false;
                    }
                    if (t___805) this.out__303.Append("+");
                    else
                    {
                        t___1195 = S::Convert.ToString(min__366);
                        this.out__303.Append("{" + t___1195);
                        if (min__366 != max__367)
                        {
                            this.out__303.Append(",");
                            if (!(max__367 == null))
                            {
                                t___1198 = S::Convert.ToString(max__367.Value);
                                this.out__303.Append(t___1198);
                            }
                        }
                        this.out__303.Append("}");
                    }
                }
            }
            if (repeat__364.Reluctant) this.out__303.Append("?");
        }
        void PushSequence(Sequence sequence__369)
        {
            int t___1182;
            IRegexNode t___1184;
            int i__371 = 0;
            while (true)
            {
                t___1182 = sequence__369.Items.Count;
                if (!(i__371 < t___1182)) break;
                t___1184 = sequence__369.Items[i__371];
                this.PushRegex(t___1184);
                i__371 = i__371 + 1;
            }
        }
        public int ? MaxCode(ICodePart codePart__373)
        {
            int ? return__159;
            int t___1178;
            CodePoints t___791;
            if (codePart__373 is CodePoints)
            {
                t___791 = (CodePoints) codePart__373;
                string value__375 = t___791.Value;
                if (string.IsNullOrEmpty(value__375))
                {
                    return__159 = null;
                }
                else
                {
                    int max__376 = 0;
                    int index__377 = 0;
                    while (true)
                    {
                        if (!C::StringUtil.HasIndex(value__375, index__377)) break;
                        int next__378 = C::StringUtil.Get(value__375, index__377);
                        if (next__378 > max__376)
                        {
                            max__376 = next__378;
                        }
                        t___1178 = C::StringUtil.Next(value__375, index__377);
                        index__377 = t___1178;
                    }
                    return__159 = max__376;
                }
            }
            else if (codePart__373 is CodeRange)
            {
                return__159 = ((CodeRange) codePart__373).Max;
            }
            else if (codePart__373 == R::RegexGlobal.Digit)
            {
                return__159 = Codes.Digit9;
            }
            else if (codePart__373 == R::RegexGlobal.Space)
            {
                return__159 = Codes.Space;
            }
            else if (codePart__373 == R::RegexGlobal.Word)
            {
                return__159 = Codes.LowerZ;
            }
            else
            {
                return__159 = null;
            }
            return return__159;
        }
        public RegexFormatter()
        {
            T::StringBuilder t___1172 = new T::StringBuilder();
            this.out__303 = t___1172;
        }
    }
}
