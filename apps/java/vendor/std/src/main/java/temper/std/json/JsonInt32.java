package temper.std.json;
public final class JsonInt32 implements JsonNumeric {
    public final int content;
    public void produce(JsonProducer p__470) {
        p__470.int32Value(this.content);
    }
    public String asJsonNumericToken() {
        return Integer.toString(this.content);
    }
    public int asInt32() {
        return this.content;
    }
    public long asInt64() {
        return (long) this.content;
    }
    public double asFloat64() {
        return (double) this.content;
    }
    public JsonInt32(int content__481) {
        this.content = content__481;
    }
    public int getContent() {
        return this.content;
    }
}
