using G = System.Collections.Generic;
using T = System.Text;
namespace Orm.Src
{
    public class SqlFragment
    {
        readonly G::IReadOnlyList<ISqlPart> parts__741;
        public SqlSource ToSource()
        {
            return new SqlSource(this.ToString());
        }
        public string ToString()
        {
            int t___4809;
            T::StringBuilder builder__746 = new T::StringBuilder();
            int i__747 = 0;
            while (true)
            {
                t___4809 = this.parts__741.Count;
                if (!(i__747 < t___4809)) break;
                this.parts__741[i__747].FormatTo(builder__746);
                i__747 = i__747 + 1;
            }
            return builder__746.ToString();
        }
        public SqlFragment(G::IReadOnlyList<ISqlPart> parts__749)
        {
            this.parts__741 = parts__749;
        }
        public G::IReadOnlyList<ISqlPart> Parts
        {
            get
            {
                return this.parts__741;
            }
        }
    }
}
