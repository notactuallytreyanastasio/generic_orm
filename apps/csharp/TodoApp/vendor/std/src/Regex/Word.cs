using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Word: ISpecialSet
    {
        public Word()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1449)
        {
            return IRegexNode.FoundDefault(this, text___1449);
        }
        public Match Find(string text___1451)
        {
            return IRegexNode.FindDefault(this, text___1451);
        }
        public string Replace(string text___1453, S::Func<Match, string> format___1454)
        {
            return IRegexNode.ReplaceDefault(this, text___1453, (S::Func<Match, string>) format___1454);
        }
        public G::IReadOnlyList<string> Split(string text___1456)
        {
            return IRegexNode.SplitDefault(this, text___1456);
        }
    }
}
