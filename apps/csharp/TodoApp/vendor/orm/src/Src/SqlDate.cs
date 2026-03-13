using S = System;
using T = System.Text;
namespace Orm.Src
{
    public class SqlDate: ISqlPart
    {
        readonly S::DateTime value__765;
        public void FormatTo(T::StringBuilder builder__767)
        {
            builder__767.Append("'");
            string t___4792 = this.value__765.ToString("yyyy-MM-dd");
            builder__767.Append(t___4792);
            builder__767.Append("'");
        }
        public SqlDate(S::DateTime value__770)
        {
            this.value__765 = value__770;
        }
        public S::DateTime Value
        {
            get
            {
                return this.value__765;
            }
        }
    }
}
