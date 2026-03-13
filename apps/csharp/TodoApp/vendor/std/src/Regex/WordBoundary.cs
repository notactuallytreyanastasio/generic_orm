using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class WordBoundary: ISpecial
    {
        public WordBoundary()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1419)
        {
            return IRegexNode.FoundDefault(this, text___1419);
        }
        public Match Find(string text___1421)
        {
            return IRegexNode.FindDefault(this, text___1421);
        }
        public string Replace(string text___1423, S::Func<Match, string> format___1424)
        {
            return IRegexNode.ReplaceDefault(this, text___1423, (S::Func<Match, string>) format___1424);
        }
        public G::IReadOnlyList<string> Split(string text___1426)
        {
            return IRegexNode.SplitDefault(this, text___1426);
        }
    }
}
