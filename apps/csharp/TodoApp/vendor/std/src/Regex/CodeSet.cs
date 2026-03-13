using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodeSet: IRegexNode
    {
        readonly G::IReadOnlyList<ICodePart> items__211;
        readonly bool negated__212;
        public CodeSet(G::IReadOnlyList<ICodePart> items__214, bool ? negated = null)
        {
            bool negated__215;
            if (negated == null)
            {
                negated__215 = false;
            }
            else
            {
                negated__215 = negated.Value;
            }
            this.items__211 = items__214;
            this.negated__212 = negated__215;
        }
        public G::IReadOnlyList<ICodePart> Items
        {
            get
            {
                return this.items__211;
            }
        }
        public bool Negated
        {
            get
            {
                return this.negated__212;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1349)
        {
            return IRegexNode.FoundDefault(this, text___1349);
        }
        public Match Find(string text___1351)
        {
            return IRegexNode.FindDefault(this, text___1351);
        }
        public string Replace(string text___1353, S::Func<Match, string> format___1354)
        {
            return IRegexNode.ReplaceDefault(this, text___1353, (S::Func<Match, string>) format___1354);
        }
        public G::IReadOnlyList<string> Split(string text___1356)
        {
            return IRegexNode.SplitDefault(this, text___1356);
        }
    }
}
