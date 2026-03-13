package temper.std.json;
public final class JsonString implements JsonSyntaxTree {
    public final String content;
    public void produce(JsonProducer p__456) {
        p__456.stringValue(this.content);
    }
    public JsonString(String content__459) {
        this.content = content__459;
    }
    public String getContent() {
        return this.content;
    }
}
