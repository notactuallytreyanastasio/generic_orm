package temper.std.json;
import java.util.List;
import temper.core.Core;
import java.util.Map;
import temper.core.Nullable;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.function.BiConsumer;
import static temper.std.json.NullInterchangeContext.instance;
public final class JsonSyntaxTreeProducer implements JsonProducer, JsonParseErrorReceiver {
    final List<List<JsonSyntaxTree>> stack;
    @Nullable String error;
    public InterchangeContext getInterchangeContext() {
        return instance;
    }
    public JsonSyntaxTreeProducer() {
        List<List<JsonSyntaxTree>> t_2419 = new ArrayList<>();
        this.stack = t_2419;
        List<JsonSyntaxTree> t_2420 = new ArrayList<>();
        Core.listAdd(this.stack, t_2420);
        this.error = null;
    }
    void storeValue(JsonSyntaxTree v__597) {
        int t_2416;
        if (!this.stack.isEmpty()) {
            t_2416 = this.stack.size();
            Core.listAdd(Core.listGet(this.stack, t_2416 - 1), v__597);
        }
    }
    public void startObject() {
        List<JsonSyntaxTree> t_2413 = new ArrayList<>();
        Core.listAdd(this.stack, t_2413);
    }
    public void endObject() {
        @Nullable Map<String, List<JsonSyntaxTree>> t_2402;
        JsonObject t_2411;
        JsonString t_1656;
        JsonString t_1658;
        Map<String, List<JsonSyntaxTree>> t_1664;
        Map<String, List<JsonSyntaxTree>> t_1666;
        List<JsonSyntaxTree> t_1668;
        List<JsonSyntaxTree> t_1669;
        List<JsonSyntaxTree> t_1671;
        List<JsonSyntaxTree> t_1672;
        fn__602: {
            if (this.stack.isEmpty()) {
                break fn__602;
            }
            List<JsonSyntaxTree> ls__603 = Core.listRemoveLast(this.stack);
            Map<String, List<JsonSyntaxTree>> m__604 = new LinkedHashMap<>();
            @Nullable Map<String, List<JsonSyntaxTree>> multis__605 = null;
            int i__606 = 0;
            int n__607 = ls__603.size() & -2;
            while (i__606 < n__607) {
                int postfixReturn_40 = i__606;
                i__606 = i__606 + 1;
                JsonSyntaxTree keyTree__608 = Core.listGet(ls__603, postfixReturn_40);
                if (!(keyTree__608 instanceof JsonString)) {
                    break;
                }
                t_1656 = Core.cast(JsonString.class, keyTree__608);
                t_1658 = t_1656;
                String key__609 = t_1658.getContent();
                int postfixReturn_41 = i__606;
                i__606 = i__606 + 1;
                JsonSyntaxTree value__610 = Core.listGet(ls__603, postfixReturn_41);
                if (m__604.containsKey(key__609)) {
                    if (multis__605 == null) {
                        t_2402 = new LinkedHashMap<>();
                        multis__605 = t_2402;
                    }
                    if (multis__605 == null) {
                        throw Core.bubble();
                    } else {
                        t_1664 = multis__605;
                    }
                    t_1666 = t_1664;
                    Map<String, List<JsonSyntaxTree>> mb__611 = t_1666;
                    if (!mb__611.containsKey(key__609)) {
                        t_1668 = Core.mappedGet(m__604, key__609);
                        t_1669 = t_1668;
                        mb__611.put(key__609, new ArrayList<>(t_1669));
                    }
                    t_1671 = Core.mappedGet(mb__611, key__609);
                    t_1672 = t_1671;
                    Core.listAdd(t_1672, value__610);
                } else {
                    m__604.put(key__609, List.of(value__610));
                }
            }
            @Nullable Map<String, List<JsonSyntaxTree>> multis__612 = multis__605;
            if (multis__612 != null) {
                BiConsumer<String, List<JsonSyntaxTree>> fn__2392 = (k__613, vs__614) -> {
                    List<JsonSyntaxTree> t_2390 = List.copyOf(vs__614);
                    m__604.put(k__613, t_2390);
                };
                Core.mappedForEach(multis__612, fn__2392);
            }
            t_2411 = new JsonObject(Core.mappedToMap(m__604));
            this.storeValue(t_2411);
        }
    }
    public void objectKey(String key__616) {
        JsonString t_2388 = new JsonString(key__616);
        this.storeValue(t_2388);
    }
    public void startArray() {
        List<JsonSyntaxTree> t_2386 = new ArrayList<>();
        Core.listAdd(this.stack, t_2386);
    }
    public void endArray() {
        JsonArray t_2384;
        fn__621: {
            if (this.stack.isEmpty()) {
                break fn__621;
            }
            List<JsonSyntaxTree> ls__622 = Core.listRemoveLast(this.stack);
            t_2384 = new JsonArray(List.copyOf(ls__622));
            this.storeValue(t_2384);
        }
    }
    public void nullValue() {
        JsonNull t_2379 = new JsonNull();
        this.storeValue(t_2379);
    }
    public void booleanValue(boolean x__626) {
        JsonBoolean t_2377 = new JsonBoolean(x__626);
        this.storeValue(t_2377);
    }
    public void int32Value(int x__629) {
        JsonInt32 t_2375 = new JsonInt32(x__629);
        this.storeValue(t_2375);
    }
    public void int64Value(long x__632) {
        JsonInt64 t_2373 = new JsonInt64(x__632);
        this.storeValue(t_2373);
    }
    public void float64Value(double x__635) {
        JsonFloat64 t_2371 = new JsonFloat64(x__635);
        this.storeValue(t_2371);
    }
    public void numericTokenValue(String x__638) {
        JsonNumericToken t_2369 = new JsonNumericToken(x__638);
        this.storeValue(t_2369);
    }
    public void stringValue(String x__641) {
        JsonString t_2367 = new JsonString(x__641);
        this.storeValue(t_2367);
    }
    public JsonSyntaxTree toJsonSyntaxTree() {
        boolean t_1629;
        if (this.stack.size() != 1) {
            t_1629 = true;
        } else {
            t_1629 = this.error != null;
        }
        if (t_1629) {
            throw Core.bubble();
        }
        List<JsonSyntaxTree> ls__645 = Core.listGet(this.stack, 0);
        if (ls__645.size() != 1) {
            throw Core.bubble();
        }
        return Core.listGet(ls__645, 0);
    }
    public @Nullable String getJsonError() {
        return this.error;
    }
    public JsonParseErrorReceiver getParseErrorReceiver() {
        return this;
    }
    public void explainJsonError(String error__651) {
        this.error = error__651;
    }
}
