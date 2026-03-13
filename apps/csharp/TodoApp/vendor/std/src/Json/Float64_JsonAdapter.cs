namespace TemperLang.Std.Json
{
    class Float64_JsonAdapter: IJsonAdapter<double>
    {
        public void EncodeToJson(double x__783, IJsonProducer p__784)
        {
            p__784.Float64_Value(x__783);
        }
        public double DecodeFromJson(IJsonSyntaxTree t__787, IInterchangeContext ic__788)
        {
            IJsonNumeric t___1387;
            t___1387 = (IJsonNumeric) t__787;
            return t___1387.AsFloat64();
        }
        public Float64_JsonAdapter()
        {
        }
    }
}
