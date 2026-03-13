namespace TemperLang.Std.Json
{
    class StringJsonAdapter: IJsonAdapter<string>
    {
        public void EncodeToJson(string x__813, IJsonProducer p__814)
        {
            p__814.StringValue(x__813);
        }
        public string DecodeFromJson(IJsonSyntaxTree t__817, IInterchangeContext ic__818)
        {
            JsonString t___1375;
            t___1375 = (JsonString) t__817;
            return t___1375.Content;
        }
        public StringJsonAdapter()
        {
        }
    }
}
