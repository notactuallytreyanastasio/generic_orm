package temper.std.json;
import temper.core.Core;
public final class JsonInt64 implements JsonNumeric {
    public final long content;
    public void produce(JsonProducer p__484) {
        p__484.int64Value(this.content);
    }
    public String asJsonNumericToken() {
        return Long.toString(this.content);
    }
    public int asInt32() {
        return Core.int64ToInt(this.content);
    }
    public long asInt64() {
        return this.content;
    }
    public double asFloat64() {
        return Core.int64ToFloat64(this.content);
    }
    public JsonInt64(long content__495) {
        this.content = content__495;
    }
    public long getContent() {
        return this.content;
    }
}
