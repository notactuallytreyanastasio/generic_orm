using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Space: ISpecialSet
    {
        public Space()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1439)
        {
            return IRegexNode.FoundDefault(this, text___1439);
        }
        public Match Find(string text___1441)
        {
            return IRegexNode.FindDefault(this, text___1441);
        }
        public string Replace(string text___1443, S::Func<Match, string> format___1444)
        {
            return IRegexNode.ReplaceDefault(this, text___1443, (S::Func<Match, string>) format___1444);
        }
        public G::IReadOnlyList<string> Split(string text___1446)
        {
            return IRegexNode.SplitDefault(this, text___1446);
        }
    }
}
