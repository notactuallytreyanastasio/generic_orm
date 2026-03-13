package temper.std.temporal;
import java.time.LocalDate;
import temper.std.json.JsonAdapter;
import temper.std.json.JsonProducer;
import temper.std.json.JsonSyntaxTree;
import temper.std.json.InterchangeContext;
final class DateJsonAdapter implements JsonAdapter<LocalDate> {
    public void encodeToJson(LocalDate x__116, JsonProducer p__117) {
        TemporalGlobal.encodeToJson__90(x__116, p__117);
    }
    public LocalDate decodeFromJson(JsonSyntaxTree t__118, InterchangeContext ic__119) {
        return TemporalGlobal.decodeFromJson__93(t__118, ic__119);
    }
    public DateJsonAdapter() {
    }
}
