namespace TemperLang.Std.Json
{
    class Int32_JsonAdapter: IJsonAdapter<int>
    {
        public void EncodeToJson(int x__793, IJsonProducer p__794)
        {
            p__794.Int32_Value(x__793);
        }
        public int DecodeFromJson(IJsonSyntaxTree t__797, IInterchangeContext ic__798)
        {
            IJsonNumeric t___1383;
            t___1383 = (IJsonNumeric) t__797;
            return t___1383.AsInt32();
        }
        public Int32_JsonAdapter()
        {
        }
    }
}
