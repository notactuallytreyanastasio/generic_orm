package temper.std.json;
import temper.core.Nullable;
public final class OrNullJsonAdapter<T__184> implements JsonAdapter<@Nullable T__184> {
    final JsonAdapter<T__184> adapterForT;
    public void encodeToJson(@Nullable T__184 x__843, JsonProducer p__844) {
        if (x__843 == null) {
            p__844.nullValue();
        } else {
            T__184 x_962 = x__843;
            this.adapterForT.encodeToJson(x_962, p__844);
        }
    }
    public @Nullable T__184 decodeFromJson(JsonSyntaxTree t__847, InterchangeContext ic__848) {
        @Nullable T__184 return__350;
        if (t__847 instanceof JsonNull) {
            return__350 = null;
        } else {
            return__350 = this.adapterForT.decodeFromJson(t__847, ic__848);
        }
        return return__350;
    }
    public OrNullJsonAdapter(JsonAdapter<T__184> adapterForT__851) {
        this.adapterForT = adapterForT__851;
    }
}
