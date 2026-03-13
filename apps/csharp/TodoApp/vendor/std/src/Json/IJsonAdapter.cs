namespace TemperLang.Std.Json
{
    public interface IJsonAdapter<T__161>
    {
        void EncodeToJson(T__161 x__765, IJsonProducer p__766);
        T__161 DecodeFromJson(IJsonSyntaxTree t__769, IInterchangeContext ic__770);
    }
}
