using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Regex
{
    class RegexRefs
    {
        readonly CodePoints codePoints__253;
        readonly Group group__254;
        readonly Match match__255;
        readonly Or orObject__256;
        public RegexRefs(CodePoints ? codePoints = null, Group ? @group = null, Match ? match = null, Or ? orObject = null)
        {
            CodePoints t___1292;
            Group t___1293;
            G::IReadOnlyDictionary<string, Group> t___1295;
            Match t___1296;
            Or t___1297;
            CodePoints codePoints__258;
            if (codePoints == null)
            {
                t___1292 = new CodePoints("");
                codePoints__258 = t___1292;
            }
            else
            {
                codePoints__258 = codePoints!;
            }
            Group group__259;
            if (@group == null)
            {
                t___1293 = new Group("", "", 0, 0);
                group__259 = t___1293;
            }
            else
            {
                group__259 = @group!;
            }
            Match match__260;
            if (match == null)
            {
                t___1295 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, Group>>(new G::KeyValuePair<string, Group>("", group__259)));
                t___1296 = new Match(group__259, t___1295);
                match__260 = t___1296;
            }
            else
            {
                match__260 = match!;
            }
            Or orObject__261;
            if (orObject == null)
            {
                t___1297 = new Or(C::Listed.CreateReadOnlyList<IRegexNode>());
                orObject__261 = t___1297;
            }
            else
            {
                orObject__261 = orObject!;
            }
            this.codePoints__253 = codePoints__258;
            this.group__254 = group__259;
            this.match__255 = match__260;
            this.orObject__256 = orObject__261;
        }
        public CodePoints CodePoints
        {
            get
            {
                return this.codePoints__253;
            }
        }
        public Group Group
        {
            get
            {
                return this.group__254;
            }
        }
        public Match Match
        {
            get
            {
                return this.match__255;
            }
        }
        public Or OrObject
        {
            get
            {
                return this.orObject__256;
            }
        }
    }
}
