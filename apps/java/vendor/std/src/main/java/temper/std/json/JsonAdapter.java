package temper.std.json;
public interface JsonAdapter<T__161> {
    void encodeToJson(T__161 x__765, JsonProducer p__766);
    T__161 decodeFromJson(JsonSyntaxTree t__769, InterchangeContext ic__770);
}
