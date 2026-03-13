package temper.std.json;
import temper.core.Core;
public final class JsonFloat64 implements JsonNumeric {
    public final double content;
    public void produce(JsonProducer p__498) {
        p__498.float64Value(this.content);
    }
    public String asJsonNumericToken() {
        return Core.float64ToString(this.content);
    }
    public int asInt32() {
        return Core.float64ToInt(this.content);
    }
    public long asInt64() {
        return Core.float64ToInt64(this.content);
    }
    public double asFloat64() {
        return this.content;
    }
    public JsonFloat64(double content__509) {
        this.content = content__509;
    }
    public double getContent() {
        return this.content;
    }
}
