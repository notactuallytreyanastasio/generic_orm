using T = System.Text;
namespace Orm.Src
{
    public class SqlSource: ISqlPart
    {
        readonly string source__753;
        public void FormatTo(T::StringBuilder builder__755)
        {
            builder__755.Append(this.source__753);
        }
        public SqlSource(string source__758)
        {
            this.source__753 = source__758;
        }
        public string Source
        {
            get
            {
                return this.source__753;
            }
        }
    }
}
