using S = System;
using T = System.Text;
namespace Orm.Src
{
    public class SqlInt64: ISqlPart
    {
        readonly long value__783;
        public void FormatTo(T::StringBuilder builder__785)
        {
            string t___4797 = S::Convert.ToString(this.value__783);
            builder__785.Append(t___4797);
        }
        public SqlInt64(long value__788)
        {
            this.value__783 = value__788;
        }
        public long Value
        {
            get
            {
                return this.value__783;
            }
        }
    }
}
