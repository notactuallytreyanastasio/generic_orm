namespace TemperLang.Std.Json
{
    class Int64_JsonAdapter: IJsonAdapter<long>
    {
        public void EncodeToJson(long x__803, IJsonProducer p__804)
        {
            p__804.Int64_Value(x__803);
        }
        public long DecodeFromJson(IJsonSyntaxTree t__807, IInterchangeContext ic__808)
        {
            IJsonNumeric t___1379;
            t___1379 = (IJsonNumeric) t__807;
            return t___1379.AsInt64();
        }
        public Int64_JsonAdapter()
        {
        }
    }
}
