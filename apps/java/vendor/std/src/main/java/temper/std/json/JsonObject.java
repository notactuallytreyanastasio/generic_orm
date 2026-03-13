package temper.std.json;
import java.util.List;
import temper.core.Core;
import java.util.Map;
import temper.core.Nullable;
import java.util.function.Consumer;
import java.util.function.BiConsumer;
public final class JsonObject implements JsonSyntaxTree {
    public final Map<String, List<JsonSyntaxTree>> properties;
    /**
     * The JSON value tree associated with the given property key or null
     * if there is no such value.
     *
     * The properties map contains a list of sub-trees because JSON
     * allows duplicate properties.  ECMA-404 §6 notes (emphasis added):
     *
     * &gt; The JSON syntax does not impose any restrictions on the strings
     * &gt; used as names, &#42;&#42;does not require that name strings be unique&#42;&#42;,
     * &gt; and does not assign any significance to the ordering of
     * &gt; name/value pairs.
     *
     * When widely used JSON parsers need to relate a property key
     * to a single value, they tend to prefer the last key/value pair
     * from a JSON object.  For example:
     *
     * JS:
     *
     *     JSON.parse('&#123;"x":"first","x":"last"&#125;').x === 'last'
     *
     * Python:
     *
     *     import json
     *     json.loads('&#123;"x":"first","x":"last"&#125;')['x'] == 'last'
     *
     * C#:
     *
     *    using System.Text.Json;
     * 		JsonDocument d = JsonDocument.Parse(
     * 			"""
     * 			&#123;"x":"first","x":"last"&#125;
     * 			"""
     * 		);
     * 		d.RootElement.GetProperty("x").GetString() == "last"
     */
    public @Nullable JsonSyntaxTree propertyValueOrNull(String propertyKey__422) {
        @Nullable JsonSyntaxTree return__209;
        List<JsonSyntaxTree> treeList__424 = this.properties.getOrDefault(propertyKey__422, List.of());
        int lastIndex__425 = treeList__424.size() - 1;
        if (lastIndex__425 >= 0) {
            return__209 = Core.listGet(treeList__424, lastIndex__425);
        } else {
            return__209 = null;
        }
        return return__209;
    }
    public JsonSyntaxTree propertyValueOrBubble(String propertyKey__427) {
        JsonSyntaxTree return__210;
        @Nullable JsonSyntaxTree t_2518 = this.propertyValueOrNull(propertyKey__427);
        if (t_2518 == null) {
            throw Core.bubble();
        } else {
            return__210 = t_2518;
        }
        return return__210;
    }
    public void produce(JsonProducer p__430) {
        p__430.startObject();
        BiConsumer<String, List<JsonSyntaxTree>> fn__2513 = (k__432, vs__433) -> {
            Consumer<JsonSyntaxTree> fn__2510 = v__434 -> {
                p__430.objectKey(k__432);
                v__434.produce(p__430);
            };
            vs__433.forEach(fn__2510);
        };
        Core.mappedForEach(this.properties, fn__2513);
        p__430.endObject();
    }
    public JsonObject(Map<String, List<JsonSyntaxTree>> properties__436) {
        this.properties = properties__436;
    }
    public Map<String, List<JsonSyntaxTree>> getProperties() {
        return this.properties;
    }
}
