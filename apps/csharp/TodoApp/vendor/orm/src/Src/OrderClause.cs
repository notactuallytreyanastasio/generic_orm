namespace Orm.Src
{
    public class OrderClause
    {
        readonly ISafeIdentifier field__529;
        readonly bool ascending__530;
        public OrderClause(ISafeIdentifier field__532, bool ascending__533)
        {
            this.field__529 = field__532;
            this.ascending__530 = ascending__533;
        }
        public ISafeIdentifier Field
        {
            get
            {
                return this.field__529;
            }
        }
        public bool Ascending
        {
            get
            {
                return this.ascending__530;
            }
        }
    }
}
