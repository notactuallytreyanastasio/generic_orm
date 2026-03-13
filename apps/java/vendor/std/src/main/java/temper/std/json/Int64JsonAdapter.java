package temper.std.json;
import temper.core.Core;
final class Int64JsonAdapter implements JsonAdapter<Long> {
    private void encodeToJson__802(long x__803, JsonProducer p__804) {
        p__804.int64Value(x__803);
    }
    public void encodeToJson(Long x__803, JsonProducer p__804) {
        encodeToJson__802(x__803.longValue(), p__804);
    }
    private long decodeFromJson__806(JsonSyntaxTree t__807, InterchangeContext ic__808) {
        JsonNumeric t_1379;
        t_1379 = Core.cast(JsonNumeric.class, t__807);
        return t_1379.asInt64();
    }
    public Long decodeFromJson(JsonSyntaxTree t__807, InterchangeContext ic__808) {
        return decodeFromJson__806(t__807, ic__808);
    }
    public Int64JsonAdapter() {
    }
}
