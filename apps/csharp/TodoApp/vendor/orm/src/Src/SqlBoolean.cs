using T = System.Text;
namespace Orm.Src
{
    public class SqlBoolean: ISqlPart
    {
        readonly bool value__759;
        public void FormatTo(T::StringBuilder builder__761)
        {
            string t___2868;
            if (this.value__759)
            {
                t___2868 = "TRUE";
            }
            else
            {
                t___2868 = "FALSE";
            }
            builder__761.Append(t___2868);
        }
        public SqlBoolean(bool value__764)
        {
            this.value__759 = value__764;
        }
        public bool Value
        {
            get
            {
                return this.value__759;
            }
        }
    }
}
