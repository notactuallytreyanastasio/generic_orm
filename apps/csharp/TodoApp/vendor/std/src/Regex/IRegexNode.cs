using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public interface IRegexNode
    {
        protected static Regex CompiledDefault(IRegexNode this__44)
        {
            return new Regex(this__44);
        }
        Regex Compiled()
        {
            return CompiledDefault(this);
        }
        protected static bool FoundDefault(IRegexNode this__45, string text__172)
        {
            return this__45.Compiled().Found(text__172);
        }
        bool Found(string text__172)
        {
            return FoundDefault(this, text__172);
        }
        protected static Match FindDefault(IRegexNode this__46, string text__175)
        {
            return this__46.Compiled().Find(text__175);
        }
        Match Find(string text__175)
        {
            return FindDefault(this, text__175);
        }
        protected static string ReplaceDefault(IRegexNode this__47, string text__178, S::Func<Match, string> format__179)
        {
            return this__47.Compiled().Replace(text__178, (S::Func<Match, string>) format__179);
        }
        string Replace(string text__178, S::Func<Match, string> format__179)
        {
            return ReplaceDefault(this, text__178, format__179);
        }
        protected static G::IReadOnlyList<string> SplitDefault(IRegexNode this__48, string text__182)
        {
            return this__48.Compiled().Split(text__182);
        }
        G::IReadOnlyList<string> Split(string text__182)
        {
            return SplitDefault(this, text__182);
        }
    }
}
