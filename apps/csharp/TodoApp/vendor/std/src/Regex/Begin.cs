using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Begin: ISpecial
    {
        public Begin()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1389)
        {
            return IRegexNode.FoundDefault(this, text___1389);
        }
        public Match Find(string text___1391)
        {
            return IRegexNode.FindDefault(this, text___1391);
        }
        public string Replace(string text___1393, S::Func<Match, string> format___1394)
        {
            return IRegexNode.ReplaceDefault(this, text___1393, (S::Func<Match, string>) format___1394);
        }
        public G::IReadOnlyList<string> Split(string text___1396)
        {
            return IRegexNode.SplitDefault(this, text___1396);
        }
    }
}
