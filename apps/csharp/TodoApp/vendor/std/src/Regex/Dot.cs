using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Dot: ISpecial
    {
        public Dot()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1399)
        {
            return IRegexNode.FoundDefault(this, text___1399);
        }
        public Match Find(string text___1401)
        {
            return IRegexNode.FindDefault(this, text___1401);
        }
        public string Replace(string text___1403, S::Func<Match, string> format___1404)
        {
            return IRegexNode.ReplaceDefault(this, text___1403, (S::Func<Match, string>) format___1404);
        }
        public G::IReadOnlyList<string> Split(string text___1406)
        {
            return IRegexNode.SplitDefault(this, text___1406);
        }
    }
}
