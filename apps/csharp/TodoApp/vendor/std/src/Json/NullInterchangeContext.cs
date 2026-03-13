namespace TemperLang.Std.Json
{
    public class NullInterchangeContext: IInterchangeContext
    {
        public string ? GetHeader(string headerName__379)
        {
            return null;
        }
        public static readonly NullInterchangeContext Instance = new NullInterchangeContext();
        public NullInterchangeContext()
        {
        }
    }
}
