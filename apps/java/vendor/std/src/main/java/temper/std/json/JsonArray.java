package temper.std.json;
import java.util.List;
import java.util.function.Consumer;
public final class JsonArray implements JsonSyntaxTree {
    public final List<JsonSyntaxTree> elements;
    public void produce(JsonProducer p__439) {
        p__439.startArray();
        Consumer<JsonSyntaxTree> fn__2503 = v__441 -> {
            v__441.produce(p__439);
        };
        this.elements.forEach(fn__2503);
        p__439.endArray();
    }
    public JsonArray(List<JsonSyntaxTree> elements__443) {
        this.elements = elements__443;
    }
    public List<JsonSyntaxTree> getElements() {
        return this.elements;
    }
}
