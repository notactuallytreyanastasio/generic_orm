package orm.src;
import java.util.List;
import temper.core.Core;
import java.util.function.Consumer;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.function.IntConsumer;
import java.util.function.DoubleConsumer;
public final class SqlBuilder {
    final List<SqlPart> buffer;
    public void appendSafe(String sqlSource__676) {
        SqlSource t_4789 = new SqlSource(sqlSource__676);
        Core.listAdd(this.buffer, t_4789);
    }
    public void appendFragment(SqlFragment fragment__679) {
        List<SqlPart> t_4787 = fragment__679.getParts();
        Core.listAddAll(this.buffer, t_4787);
    }
    public void appendPart(SqlPart part__682) {
        Core.listAdd(this.buffer, part__682);
    }
    public void appendPartList(List<SqlPart> values__685) {
        Consumer<SqlPart> fn__4783 = x__687 -> {
            this.appendPart(x__687);
        };
        this.appendList(values__685, fn__4783);
    }
    public void appendBoolean(boolean value__689) {
        SqlBoolean t_4780 = new SqlBoolean(value__689);
        Core.listAdd(this.buffer, t_4780);
    }
    public void appendBooleanList(List<Boolean> values__692) {
        Consumer<Boolean> fn__4777 = x__694 -> {
            this.appendBoolean(x__694);
        };
        this.appendList(values__692, fn__4777);
    }
    public void appendDate(LocalDate value__696) {
        SqlDate t_4774 = new SqlDate(value__696);
        Core.listAdd(this.buffer, t_4774);
    }
    public void appendDateList(List<LocalDate> values__699) {
        Consumer<LocalDate> fn__4771 = x__701 -> {
            this.appendDate(x__701);
        };
        this.appendList(values__699, fn__4771);
    }
    public void appendFloat64(double value__703) {
        SqlFloat64 t_4768 = new SqlFloat64(value__703);
        Core.listAdd(this.buffer, t_4768);
    }
    public void appendFloat64List(List<Double> values__706) {
        DoubleConsumer fn__4765 = x__708 -> {
            this.appendFloat64(x__708);
        };
        this.appendList(values__706, fn__4765 :: accept);
    }
    public void appendInt32(int value__710) {
        SqlInt32 t_4762 = new SqlInt32(value__710);
        Core.listAdd(this.buffer, t_4762);
    }
    public void appendInt32List(List<Integer> values__713) {
        IntConsumer fn__4759 = x__715 -> {
            this.appendInt32(x__715);
        };
        this.appendList(values__713, fn__4759 :: accept);
    }
    public void appendInt64(long value__717) {
        SqlInt64 t_4756 = new SqlInt64(value__717);
        Core.listAdd(this.buffer, t_4756);
    }
    public void appendInt64List(List<Long> values__720) {
        Consumer<Long> fn__4753 = x__722 -> {
            this.appendInt64(x__722);
        };
        this.appendList(values__720, fn__4753);
    }
    public void appendString(String value__724) {
        SqlString t_4750 = new SqlString(value__724);
        Core.listAdd(this.buffer, t_4750);
    }
    public void appendStringList(List<String> values__727) {
        Consumer<String> fn__4747 = x__729 -> {
            this.appendString(x__729);
        };
        this.appendList(values__727, fn__4747);
    }
    <T__136> void appendList(List<T__136> values__731, Consumer<T__136> appendValue__732) {
        int t_4742;
        T__136 t_4744;
        int i__734 = 0;
        while (true) {
            t_4742 = values__731.size();
            if (i__734 >= t_4742) {
                break;
            }
            if (i__734 > 0) {
                this.appendSafe(", ");
            }
            t_4744 = Core.listGet(values__731, i__734);
            appendValue__732.accept(t_4744);
            i__734 = i__734 + 1;
        }
    }
    public SqlFragment getAccumulated() {
        return new SqlFragment(List.copyOf(this.buffer));
    }
    public SqlBuilder() {
        List<SqlPart> t_4739 = new ArrayList<>();
        this.buffer = t_4739;
    }
}
