using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodePoints: ICodePart
    {
        readonly string value__189;
        public CodePoints(string value__191)
        {
            this.value__189 = value__191;
        }
        public string Value
        {
            get
            {
                return this.value__189;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1329)
        {
            return IRegexNode.FoundDefault(this, text___1329);
        }
        public Match Find(string text___1331)
        {
            return IRegexNode.FindDefault(this, text___1331);
        }
        public string Replace(string text___1333, S::Func<Match, string> format___1334)
        {
            return IRegexNode.ReplaceDefault(this, text___1333, (S::Func<Match, string>) format___1334);
        }
        public G::IReadOnlyList<string> Split(string text___1336)
        {
            return IRegexNode.SplitDefault(this, text___1336);
        }
    }
}
