namespace TemperLang.Std.Json
{
    public interface IJsonProducer
    {
        IInterchangeContext InterchangeContext
        {
            get;
        }
        void StartObject();
        void EndObject();
        void ObjectKey(string key__389);
        void StartArray();
        void EndArray();
        void NullValue();
        void BooleanValue(bool x__398);
        void Int32_Value(int x__401);
        void Int64_Value(long x__404);
        void Float64_Value(double x__407);
        void NumericTokenValue(string x__410);
        void StringValue(string x__413);
        IJsonParseErrorReceiver ? ParseErrorReceiver
        {
            get
            {
                return GetParseErrorReceiverDefault(this);
            }
        }
        protected static IJsonParseErrorReceiver ? GetParseErrorReceiverDefault(IJsonProducer this__92)
        {
            return null;
        }
    }
}
