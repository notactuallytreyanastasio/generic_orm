package temper.std.json;
public final class JsonBoolean implements JsonSyntaxTree {
    public final boolean content;
    public void produce(JsonProducer p__446) {
        p__446.booleanValue(this.content);
    }
    public JsonBoolean(boolean content__449) {
        this.content = content__449;
    }
    public boolean isContent() {
        return this.content;
    }
}
