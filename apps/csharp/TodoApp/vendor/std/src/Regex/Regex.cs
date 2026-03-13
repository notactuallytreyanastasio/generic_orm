using S = System;
using G = System.Collections.Generic;
using R = TemperLang.Std.Regex;
namespace TemperLang.Std.Regex
{
    public class Regex
    {
        readonly IRegexNode data__262;
        public Regex(IRegexNode data__264)
        {
            IRegexNode t___421 = data__264;
            this.data__262 = t___421;
            string formatted__266 = RegexFormatter.RegexFormat(data__264);
            object t___1171 = R::RegexSupport.CompileFormatted(data__264, formatted__266);
            this.compiled__281 = t___1171;
        }
        public bool Found(string text__268)
        {
            return R::RegexSupport.CompiledFound(this, this.compiled__281, text__268);
        }
        public Match Find(string text__271, int ? begin = null)
        {
            int begin__272;
            if (begin == null)
            {
                begin__272 = 0;
            }
            else
            {
                begin__272 = begin.Value;
            }
            return R::RegexSupport.CompiledFind(this, this.compiled__281, text__271, begin__272, R::RegexGlobal.regexRefs__164);
        }
        public string Replace(string text__275, S::Func<Match, string> format__276)
        {
            return R::RegexSupport.CompiledReplace(this, this.compiled__281, text__275, (S::Func<Match, string>) format__276, R::RegexGlobal.regexRefs__164);
        }
        public G::IReadOnlyList<string> Split(string text__279)
        {
            return R::RegexSupport.CompiledSplit(this, this.compiled__281, text__279, R::RegexGlobal.regexRefs__164);
        }
        readonly object compiled__281;
        public IRegexNode Data
        {
            get
            {
                return this.data__262;
            }
        }
    }
}
