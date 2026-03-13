package temper.std.json;
import temper.core.Core;
import java.util.List;
import temper.core.Nullable;
import java.util.ArrayList;
import static temper.std.json.NullInterchangeContext.instance;
public final class JsonTextProducer implements JsonProducer {
    public final InterchangeContext interchangeContext;
    final StringBuilder buffer;
    final List<Integer> stack;
    boolean wellFormed;
    public JsonTextProducer(@Nullable InterchangeContext interchangeContext__938) {
        InterchangeContext interchangeContext__529;
        if (interchangeContext__938 == null) {
            interchangeContext__529 = instance;
        } else {
            interchangeContext__529 = interchangeContext__938;
        }
        this.interchangeContext = interchangeContext__529;
        StringBuilder t_2470 = new StringBuilder();
        this.buffer = t_2470;
        List<Integer> t_2471 = new ArrayList<>();
        this.stack = t_2471;
        Core.listAdd(this.stack, 5);
        this.wellFormed = true;
    }
    public JsonTextProducer() {
        this(null);
    }
    int state() {
        int t_2468 = this.stack.size();
        return Core.listGetOr(this.stack, t_2468 - 1, -1);
    }
    void beforeValue() {
        int t_2461;
        int t_2464;
        int t_2466;
        boolean t_1728;
        int currentState__535 = this.state();
        if (currentState__535 == 3) {
            t_2461 = this.stack.size();
            this.stack.set(t_2461 - 1, 4);
        } else if (currentState__535 == 4) {
            this.buffer.append(",");
        } else if (currentState__535 == 1) {
            t_2464 = this.stack.size();
            this.stack.set(t_2464 - 1, 2);
        } else if (currentState__535 == 5) {
            t_2466 = this.stack.size();
            this.stack.set(t_2466 - 1, 6);
        } else {
            if (currentState__535 == 6) {
                t_1728 = true;
            } else {
                t_1728 = currentState__535 == 2;
            }
            if (t_1728) {
                this.wellFormed = false;
            }
        }
    }
    public void startObject() {
        this.beforeValue();
        this.buffer.append("{");
        Core.listAdd(this.stack, 0);
    }
    public void endObject() {
        boolean t_1716;
        this.buffer.append("}");
        int currentState__540 = this.state();
        if (0 == currentState__540) {
            t_1716 = true;
        } else {
            t_1716 = 2 == currentState__540;
        }
        if (t_1716) {
            Core.listRemoveLast(this.stack);
        } else {
            this.wellFormed = false;
        }
    }
    public void objectKey(String key__542) {
        int t_2452;
        int currentState__544 = this.state();
        if (currentState__544 != 0) {
            if (currentState__544 == 2) {
                this.buffer.append(",");
            } else {
                this.wellFormed = false;
            }
        }
        JsonGlobal.encodeJsonString__351(key__542, this.buffer);
        this.buffer.append(":");
        if (currentState__544 >= 0) {
            t_2452 = this.stack.size();
            this.stack.set(t_2452 - 1, 1);
        }
    }
    public void startArray() {
        this.beforeValue();
        this.buffer.append("[");
        Core.listAdd(this.stack, 3);
    }
    public void endArray() {
        boolean t_1704;
        this.buffer.append("]");
        int currentState__549 = this.state();
        if (3 == currentState__549) {
            t_1704 = true;
        } else {
            t_1704 = 4 == currentState__549;
        }
        if (t_1704) {
            Core.listRemoveLast(this.stack);
        } else {
            this.wellFormed = false;
        }
    }
    public void nullValue() {
        this.beforeValue();
        this.buffer.append("null");
    }
    public void booleanValue(boolean x__553) {
        String t_1700;
        this.beforeValue();
        if (x__553) {
            t_1700 = "true";
        } else {
            t_1700 = "false";
        }
        this.buffer.append(t_1700);
    }
    public void int32Value(int x__556) {
        this.beforeValue();
        String t_2436 = Integer.toString(x__556);
        this.buffer.append(t_2436);
    }
    public void int64Value(long x__559) {
        this.beforeValue();
        String t_2433 = Long.toString(x__559);
        this.buffer.append(t_2433);
    }
    public void float64Value(double x__562) {
        this.beforeValue();
        String t_2430 = Core.float64ToString(x__562);
        this.buffer.append(t_2430);
    }
    public void numericTokenValue(String x__565) {
        this.beforeValue();
        this.buffer.append(x__565);
    }
    public void stringValue(String x__568) {
        this.beforeValue();
        JsonGlobal.encodeJsonString__351(x__568, this.buffer);
    }
    public String toJsonString() {
        String return__272;
        int t_2423;
        boolean t_1689;
        boolean t_1690;
        if (this.wellFormed) {
            if (this.stack.size() == 1) {
                t_2423 = this.state();
                t_1689 = t_2423 == 6;
            } else {
                t_1689 = false;
            }
            t_1690 = t_1689;
        } else {
            t_1690 = false;
        }
        if (t_1690) {
            return__272 = this.buffer.toString();
        } else {
            throw Core.bubble();
        }
        return return__272;
    }
    public InterchangeContext getInterchangeContext() {
        return this.interchangeContext;
    }
}
