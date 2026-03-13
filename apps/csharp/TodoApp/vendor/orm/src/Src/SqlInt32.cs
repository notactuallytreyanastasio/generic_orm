using S = System;
using T = System.Text;
namespace Orm.Src
{
    public class SqlInt32: ISqlPart
    {
        readonly int value__777;
        public void FormatTo(T::StringBuilder builder__779)
        {
            string t___4799 = S::Convert.ToString(this.value__777);
            builder__779.Append(t___4799);
        }
        public SqlInt32(int value__782)
        {
            this.value__777 = value__782;
        }
        public int Value
        {
            get
            {
                return this.value__777;
            }
        }
    }
}
