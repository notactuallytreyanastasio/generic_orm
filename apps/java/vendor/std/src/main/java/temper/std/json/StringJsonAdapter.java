package temper.std.json;
import temper.core.Core;
final class StringJsonAdapter implements JsonAdapter<String> {
    public void encodeToJson(String x__813, JsonProducer p__814) {
        p__814.stringValue(x__813);
    }
    public String decodeFromJson(JsonSyntaxTree t__817, InterchangeContext ic__818) {
        JsonString t_1375;
        t_1375 = Core.cast(JsonString.class, t__817);
        return t_1375.getContent();
    }
    public StringJsonAdapter() {
    }
}
