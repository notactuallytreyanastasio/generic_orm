package temper.std.json;
import temper.core.Nullable;
public interface JsonProducer {
    InterchangeContext getInterchangeContext();
    void startObject();
    void endObject();
    void objectKey(String key__389);
    void startArray();
    void endArray();
    void nullValue();
    void booleanValue(boolean x__398);
    void int32Value(int x__401);
    void int64Value(long x__404);
    void float64Value(double x__407);
    /**
     * A number that fits the JSON number grammar to allow
     * interchange of numbers that are not easily representible
     * using numeric types that Temper connects to.
     */
    void numericTokenValue(String x__410);
    void stringValue(String x__413);
    default @Nullable JsonParseErrorReceiver getParseErrorReceiver() {
        return null;
    }
}
