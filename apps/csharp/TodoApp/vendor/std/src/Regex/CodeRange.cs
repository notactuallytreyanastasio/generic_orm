using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodeRange: ICodePart
    {
        readonly int min__206;
        readonly int max__207;
        public CodeRange(int min__209, int max__210)
        {
            this.min__206 = min__209;
            this.max__207 = max__210;
        }
        public int Min
        {
            get
            {
                return this.min__206;
            }
        }
        public int Max
        {
            get
            {
                return this.max__207;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1339)
        {
            return IRegexNode.FoundDefault(this, text___1339);
        }
        public Match Find(string text___1341)
        {
            return IRegexNode.FindDefault(this, text___1341);
        }
        public string Replace(string text___1343, S::Func<Match, string> format___1344)
        {
            return IRegexNode.ReplaceDefault(this, text___1343, (S::Func<Match, string>) format___1344);
        }
        public G::IReadOnlyList<string> Split(string text___1346)
        {
            return IRegexNode.SplitDefault(this, text___1346);
        }
    }
}
