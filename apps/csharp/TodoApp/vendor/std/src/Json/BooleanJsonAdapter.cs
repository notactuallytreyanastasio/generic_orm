namespace TemperLang.Std.Json
{
    class BooleanJsonAdapter: IJsonAdapter<bool>
    {
        public void EncodeToJson(bool x__773, IJsonProducer p__774)
        {
            p__774.BooleanValue(x__773);
        }
        public bool DecodeFromJson(IJsonSyntaxTree t__777, IInterchangeContext ic__778)
        {
            JsonBoolean t___1391;
            t___1391 = (JsonBoolean) t__777;
            return t___1391.Content;
        }
        public BooleanJsonAdapter()
        {
        }
    }
}
