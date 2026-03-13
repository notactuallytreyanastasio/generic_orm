using T = System.Text;
using C = TemperLang.Core;
namespace Orm.Src
{
    public class SqlFloat64: ISqlPart
    {
        readonly double value__771;
        public void FormatTo(T::StringBuilder builder__773)
        {
            string t___4795 = C::Float64.Format(this.value__771);
            builder__773.Append(t___4795);
        }
        public SqlFloat64(double value__776)
        {
            this.value__771 = value__776;
        }
        public double Value
        {
            get
            {
                return this.value__771;
            }
        }
    }
}
