using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Repeat: IRegexNode
    {
        readonly IRegexNode item__219;
        readonly int min__220;
        readonly int ? max__221;
        readonly bool reluctant__222;
        public Repeat(IRegexNode item__224, int min__225, int ? max__226, bool ? reluctant = null)
        {
            bool reluctant__227;
            if (reluctant == null)
            {
                reluctant__227 = false;
            }
            else
            {
                reluctant__227 = reluctant.Value;
            }
            this.item__219 = item__224;
            this.min__220 = min__225;
            this.max__221 = max__226;
            this.reluctant__222 = reluctant__227;
        }
        public IRegexNode Item
        {
            get
            {
                return this.item__219;
            }
        }
        public int Min
        {
            get
            {
                return this.min__220;
            }
        }
        public int ? Max
        {
            get
            {
                return this.max__221;
            }
        }
        public bool Reluctant
        {
            get
            {
                return this.reluctant__222;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1369)
        {
            return IRegexNode.FoundDefault(this, text___1369);
        }
        public Match Find(string text___1371)
        {
            return IRegexNode.FindDefault(this, text___1371);
        }
        public string Replace(string text___1373, S::Func<Match, string> format___1374)
        {
            return IRegexNode.ReplaceDefault(this, text___1373, (S::Func<Match, string>) format___1374);
        }
        public G::IReadOnlyList<string> Split(string text___1376)
        {
            return IRegexNode.SplitDefault(this, text___1376);
        }
    }
}
