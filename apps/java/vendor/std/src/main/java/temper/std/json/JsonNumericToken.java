package temper.std.json;
import temper.core.Core;
public final class JsonNumericToken implements JsonNumeric {
    public final String content;
    public void produce(JsonProducer p__512) {
        p__512.numericTokenValue(this.content);
    }
    public String asJsonNumericToken() {
        return this.content;
    }
    public int asInt32() {
        int t_1774;
        double t_1775;
        int return_2542;
        try {
            t_1774 = Core.stringToInt(this.content);
            return_2542 = t_1774;
        } catch (RuntimeException ignored$1) {
            t_1775 = Core.stringToFloat64(this.content);
            return_2542 = Core.float64ToInt(t_1775);
        }
        return return_2542;
    }
    public long asInt64() {
        long t_1770;
        double t_1771;
        long return_2544;
        try {
            t_1770 = Core.stringToInt64(this.content);
            return_2544 = t_1770;
        } catch (RuntimeException ignored$2) {
            t_1771 = Core.stringToFloat64(this.content);
            return_2544 = Core.float64ToInt64(t_1771);
        }
        return return_2544;
    }
    public double asFloat64() {
        return Core.stringToFloat64(this.content);
    }
    public JsonNumericToken(String content__523) {
        this.content = content__523;
    }
    public String getContent() {
        return this.content;
    }
}
