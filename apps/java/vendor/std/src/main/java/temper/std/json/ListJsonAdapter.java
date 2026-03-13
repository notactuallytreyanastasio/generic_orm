package temper.std.json;
import java.util.List;
import temper.core.Core;
import java.util.function.Consumer;
import java.util.ArrayList;
final class ListJsonAdapter<T__180> implements JsonAdapter<List<T__180>> {
    final JsonAdapter<T__180> adapterForT;
    public void encodeToJson(List<T__180> x__824, JsonProducer p__825) {
        p__825.startArray();
        Consumer<T__180> fn__2174 = el__827 -> {
            this.adapterForT.encodeToJson(el__827, p__825);
        };
        x__824.forEach(fn__2174);
        p__825.endArray();
    }
    public List<T__180> decodeFromJson(JsonSyntaxTree t__829, InterchangeContext ic__830) {
        T__180 t_1369;
        List<T__180> b__832 = new ArrayList<>();
        JsonArray t_1364;
        t_1364 = Core.cast(JsonArray.class, t__829);
        List<JsonSyntaxTree> elements__833 = t_1364.getElements();
        int n__834 = elements__833.size();
        int i__835 = 0;
        while (i__835 < n__834) {
            JsonSyntaxTree el__836 = Core.listGet(elements__833, i__835);
            i__835 = i__835 + 1;
            t_1369 = this.adapterForT.decodeFromJson(el__836, ic__830);
            Core.listAdd(b__832, t_1369);
        }
        return List.copyOf(b__832);
    }
    public ListJsonAdapter(JsonAdapter<T__180> adapterForT__838) {
        this.adapterForT = adapterForT__838;
    }
}
