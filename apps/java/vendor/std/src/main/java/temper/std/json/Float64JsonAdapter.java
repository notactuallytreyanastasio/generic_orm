package temper.std.json;
import temper.core.Core;
final class Float64JsonAdapter implements JsonAdapter<Double> {
    private void encodeToJson__782(double x__783, JsonProducer p__784) {
        p__784.float64Value(x__783);
    }
    public void encodeToJson(Double x__783, JsonProducer p__784) {
        encodeToJson__782(x__783.doubleValue(), p__784);
    }
    private double decodeFromJson__786(JsonSyntaxTree t__787, InterchangeContext ic__788) {
        JsonNumeric t_1387;
        t_1387 = Core.cast(JsonNumeric.class, t__787);
        return t_1387.asFloat64();
    }
    public Double decodeFromJson(JsonSyntaxTree t__787, InterchangeContext ic__788) {
        return decodeFromJson__786(t__787, ic__788);
    }
    public Float64JsonAdapter() {
    }
}
