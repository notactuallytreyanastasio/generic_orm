using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class End: ISpecial
    {
        public End()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1409)
        {
            return IRegexNode.FoundDefault(this, text___1409);
        }
        public Match Find(string text___1411)
        {
            return IRegexNode.FindDefault(this, text___1411);
        }
        public string Replace(string text___1413, S::Func<Match, string> format___1414)
        {
            return IRegexNode.ReplaceDefault(this, text___1413, (S::Func<Match, string>) format___1414);
        }
        public G::IReadOnlyList<string> Split(string text___1416)
        {
            return IRegexNode.SplitDefault(this, text___1416);
        }
    }
}
