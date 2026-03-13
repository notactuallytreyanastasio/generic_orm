using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Digit: ISpecialSet
    {
        public Digit()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1429)
        {
            return IRegexNode.FoundDefault(this, text___1429);
        }
        public Match Find(string text___1431)
        {
            return IRegexNode.FindDefault(this, text___1431);
        }
        public string Replace(string text___1433, S::Func<Match, string> format___1434)
        {
            return IRegexNode.ReplaceDefault(this, text___1433, (S::Func<Match, string>) format___1434);
        }
        public G::IReadOnlyList<string> Split(string text___1436)
        {
            return IRegexNode.SplitDefault(this, text___1436);
        }
    }
}
