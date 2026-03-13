package temper.std.json;
import temper.core.Core;
final class BooleanJsonAdapter implements JsonAdapter<Boolean> {
    private void encodeToJson__772(boolean x__773, JsonProducer p__774) {
        p__774.booleanValue(x__773);
    }
    public void encodeToJson(Boolean x__773, JsonProducer p__774) {
        encodeToJson__772(x__773.booleanValue(), p__774);
    }
    private boolean decodeFromJson__776(JsonSyntaxTree t__777, InterchangeContext ic__778) {
        JsonBoolean t_1391;
        t_1391 = Core.cast(JsonBoolean.class, t__777);
        return t_1391.isContent();
    }
    public Boolean decodeFromJson(JsonSyntaxTree t__777, InterchangeContext ic__778) {
        return decodeFromJson__776(t__777, ic__778);
    }
    public BooleanJsonAdapter() {
    }
}
