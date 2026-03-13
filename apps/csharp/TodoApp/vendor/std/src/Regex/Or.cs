using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Or: IRegexNode
    {
        readonly G::IReadOnlyList<IRegexNode> items__216;
        public Or(G::IReadOnlyList<IRegexNode> items__218)
        {
            this.items__216 = items__218;
        }
        public G::IReadOnlyList<IRegexNode> Items
        {
            get
            {
                return this.items__216;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1359)
        {
            return IRegexNode.FoundDefault(this, text___1359);
        }
        public Match Find(string text___1361)
        {
            return IRegexNode.FindDefault(this, text___1361);
        }
        public string Replace(string text___1363, S::Func<Match, string> format___1364)
        {
            return IRegexNode.ReplaceDefault(this, text___1363, (S::Func<Match, string>) format___1364);
        }
        public G::IReadOnlyList<string> Split(string text___1366)
        {
            return IRegexNode.SplitDefault(this, text___1366);
        }
    }
}
