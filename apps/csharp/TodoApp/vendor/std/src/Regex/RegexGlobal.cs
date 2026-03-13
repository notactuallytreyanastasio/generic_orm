using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Regex
{
    public static class RegexGlobal
    {
        internal static ISpecial return__192;
        public static ISpecial Begin;
        internal static ISpecial return__194;
        public static ISpecial Dot;
        internal static ISpecial return__196;
        public static ISpecial End;
        internal static ISpecial return__198;
        public static ISpecial WordBoundary;
        internal static ISpecialSet return__200;
        public static ISpecialSet Digit;
        internal static ISpecialSet return__202;
        public static ISpecialSet Space;
        internal static ISpecialSet return__204;
        public static ISpecialSet Word;
        internal static G::IReadOnlyList<int> buildEscapeNeeds__163()
        {
            bool t___935;
            bool t___936;
            bool t___937;
            bool t___938;
            bool t___939;
            bool t___940;
            bool t___941;
            bool t___942;
            bool t___943;
            bool t___944;
            bool t___945;
            bool t___946;
            bool t___947;
            bool t___948;
            bool t___949;
            bool t___950;
            bool t___951;
            bool t___952;
            bool t___953;
            bool t___954;
            bool t___955;
            bool t___956;
            bool t___957;
            bool t___958;
            int t___959;
            G::IList<int> escapeNeeds__381 = new G::List<int>();
            int code__382 = 0;
            while (code__382 <= 127)
            {
                if (code__382 == Codes.Dash)
                {
                    t___942 = true;
                }
                else
                {
                    if (code__382 == Codes.Space)
                    {
                        t___941 = true;
                    }
                    else
                    {
                        if (code__382 == Codes.Underscore)
                        {
                            t___940 = true;
                        }
                        else
                        {
                            if (Codes.Digit0 <= code__382)
                            {
                                t___935 = code__382 <= Codes.Digit9;
                            }
                            else
                            {
                                t___935 = false;
                            }
                            if (t___935)
                            {
                                t___939 = true;
                            }
                            else
                            {
                                if (Codes.UpperA <= code__382)
                                {
                                    t___936 = code__382 <= Codes.UpperZ;
                                }
                                else
                                {
                                    t___936 = false;
                                }
                                if (t___936)
                                {
                                    t___938 = true;
                                }
                                else
                                {
                                    if (Codes.LowerA <= code__382)
                                    {
                                        t___937 = code__382 <= Codes.LowerZ;
                                    }
                                    else
                                    {
                                        t___937 = false;
                                    }
                                    t___938 = t___937;
                                }
                                t___939 = t___938;
                            }
                            t___940 = t___939;
                        }
                        t___941 = t___940;
                    }
                    t___942 = t___941;
                }
                if (t___942)
                {
                    t___959 = 0;
                }
                else
                {
                    if (code__382 == Codes.Ampersand)
                    {
                        t___958 = true;
                    }
                    else
                    {
                        if (code__382 == Codes.Backslash)
                        {
                            t___957 = true;
                        }
                        else
                        {
                            if (code__382 == Codes.Caret)
                            {
                                t___956 = true;
                            }
                            else
                            {
                                if (code__382 == Codes.CurlyLeft)
                                {
                                    t___955 = true;
                                }
                                else
                                {
                                    if (code__382 == Codes.CurlyRight)
                                    {
                                        t___954 = true;
                                    }
                                    else
                                    {
                                        if (code__382 == Codes.Dot)
                                        {
                                            t___953 = true;
                                        }
                                        else
                                        {
                                            if (code__382 == Codes.Peso)
                                            {
                                                t___952 = true;
                                            }
                                            else
                                            {
                                                if (code__382 == Codes.Pipe)
                                                {
                                                    t___951 = true;
                                                }
                                                else
                                                {
                                                    if (code__382 == Codes.Plus)
                                                    {
                                                        t___950 = true;
                                                    }
                                                    else
                                                    {
                                                        if (code__382 == Codes.Question)
                                                        {
                                                            t___949 = true;
                                                        }
                                                        else
                                                        {
                                                            if (code__382 == Codes.RoundLeft)
                                                            {
                                                                t___948 = true;
                                                            }
                                                            else
                                                            {
                                                                if (code__382 == Codes.RoundRight)
                                                                {
                                                                    t___947 = true;
                                                                }
                                                                else
                                                                {
                                                                    if (code__382 == Codes.Slash)
                                                                    {
                                                                        t___946 = true;
                                                                    }
                                                                    else
                                                                    {
                                                                        if (code__382 == Codes.SquareLeft)
                                                                        {
                                                                            t___945 = true;
                                                                        }
                                                                        else
                                                                        {
                                                                            if (code__382 == Codes.SquareRight)
                                                                            {
                                                                                t___944 = true;
                                                                            }
                                                                            else
                                                                            {
                                                                                if (code__382 == Codes.Star)
                                                                                {
                                                                                    t___943 = true;
                                                                                }
                                                                                else
                                                                                {
                                                                                    t___943 = code__382 == Codes.Tilde;
                                                                                }
                                                                                t___944 = t___943;
                                                                            }
                                                                            t___945 = t___944;
                                                                        }
                                                                        t___946 = t___945;
                                                                    }
                                                                    t___947 = t___946;
                                                                }
                                                                t___948 = t___947;
                                                            }
                                                            t___949 = t___948;
                                                        }
                                                        t___950 = t___949;
                                                    }
                                                    t___951 = t___950;
                                                }
                                                t___952 = t___951;
                                            }
                                            t___953 = t___952;
                                        }
                                        t___954 = t___953;
                                    }
                                    t___955 = t___954;
                                }
                                t___956 = t___955;
                            }
                            t___957 = t___956;
                        }
                        t___958 = t___957;
                    }
                    if (t___958)
                    {
                        t___959 = 2;
                    }
                    else
                    {
                        t___959 = 1;
                    }
                }
                C::Listed.Add(escapeNeeds__381, t___959);
                code__382 = code__382 + 1;
            }
            return C::Listed.ToReadOnlyList(escapeNeeds__381);
        }
        internal static G::IReadOnlyList<int> escapeNeeds__165;
        internal static RegexRefs regexRefs__164;
        public static IRegexNode Entire(IRegexNode item__228)
        {
            return new Sequence(C::Listed.CreateReadOnlyList<IRegexNode>(Begin, item__228, End));
        }
        public static Repeat OneOrMore(IRegexNode item__230, bool ? reluctant = null)
        {
            bool reluctant__231;
            if (reluctant == null)
            {
                reluctant__231 = false;
            }
            else
            {
                reluctant__231 = reluctant.Value;
            }
            return new Repeat(item__230, 1, null, reluctant__231);
        }
        public static Repeat Optional(IRegexNode item__233, bool ? reluctant = null)
        {
            bool reluctant__234;
            if (reluctant == null)
            {
                reluctant__234 = false;
            }
            else
            {
                reluctant__234 = reluctant.Value;
            }
            return new Repeat(item__233, 0, 1, reluctant__234);
        }
        static RegexGlobal()
        {
            return__192 = new Begin();
            Begin = return__192;
            return__194 = new Dot();
            Dot = return__194;
            return__196 = new End();
            End = return__196;
            return__198 = new WordBoundary();
            WordBoundary = return__198;
            return__200 = new Digit();
            Digit = return__200;
            return__202 = new Space();
            Space = return__202;
            return__204 = new Word();
            Word = return__204;
            escapeNeeds__165 = buildEscapeNeeds__163();
            regexRefs__164 = new RegexRefs();
        }
    }
}
