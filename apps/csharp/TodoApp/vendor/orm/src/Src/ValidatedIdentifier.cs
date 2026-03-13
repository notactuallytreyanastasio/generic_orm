namespace Orm.Src
{
    class ValidatedIdentifier: ISafeIdentifier
    {
        readonly string _value__621;
        public string SqlValue
        {
            get
            {
                return this._value__621;
            }
        }
        public ValidatedIdentifier(string _value__625)
        {
            this._value__621 = _value__625;
        }
    }
}
