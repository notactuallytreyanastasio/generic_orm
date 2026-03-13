using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Match
    {
        readonly Group full__239;
        readonly G::IReadOnlyDictionary<string, Group> groups__240;
        public Match(Group full__242, G::IReadOnlyDictionary<string, Group> groups__243)
        {
            this.full__239 = full__242;
            this.groups__240 = groups__243;
        }
        public Group Full
        {
            get
            {
                return this.full__239;
            }
        }
        public G::IReadOnlyDictionary<string, Group> Groups
        {
            get
            {
                return this.groups__240;
            }
        }
    }
}
