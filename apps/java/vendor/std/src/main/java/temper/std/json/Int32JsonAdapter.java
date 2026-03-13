package temper.std.json;
import temper.core.Core;
final class Int32JsonAdapter implements JsonAdapter<Integer> {
    private void encodeToJson__792(int x__793, JsonProducer p__794) {
        p__794.int32Value(x__793);
    }
    public void encodeToJson(Integer x__793, JsonProducer p__794) {
        encodeToJson__792(x__793.intValue(), p__794);
    }
    private int decodeFromJson__796(JsonSyntaxTree t__797, InterchangeContext ic__798) {
        JsonNumeric t_1383;
        t_1383 = Core.cast(JsonNumeric.class, t__797);
        return t_1383.asInt32();
    }
    public Integer decodeFromJson(JsonSyntaxTree t__797, InterchangeContext ic__798) {
        return decodeFromJson__796(t__797, ic__798);
    }
    public Int32JsonAdapter() {
    }
}
