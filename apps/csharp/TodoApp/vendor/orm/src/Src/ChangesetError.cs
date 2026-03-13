namespace Orm.Src
{
    public class ChangesetError
    {
        readonly string field__300;
        readonly string message__301;
        public ChangesetError(string field__303, string message__304)
        {
            this.field__300 = field__303;
            this.message__301 = message__304;
        }
        public string Field
        {
            get
            {
                return this.field__300;
            }
        }
        public string Message
        {
            get
            {
                return this.message__301;
            }
        }
    }
}
