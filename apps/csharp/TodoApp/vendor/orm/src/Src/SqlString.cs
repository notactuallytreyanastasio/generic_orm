using S = System;
using T = System.Text;
using C = TemperLang.Core;
namespace Orm.Src
{
    public class SqlString: ISqlPart
    {
        readonly string value__789;
        public void FormatTo(T::StringBuilder builder__791)
        {
            builder__791.Append("'");
            void fn__4802(int c__793)
            {
                if (c__793 == 39) builder__791.Append("''");
                else C::StringUtil.AppendCodePoint(builder__791, c__793);
            }
            C::StringUtil.ForEach(this.value__789, (S::Action<int>) fn__4802);
            builder__791.Append("'");
        }
        public SqlString(string value__795)
        {
            this.value__789 = value__795;
        }
        public string Value
        {
            get
            {
                return this.value__789;
            }
        }
    }
}
