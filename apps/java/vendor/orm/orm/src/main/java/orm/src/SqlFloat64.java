package orm.src;
import temper.core.Core;
public final class SqlFloat64 implements SqlPart {
    public final double value;
    public void formatTo(StringBuilder builder__773) {
        String t_4795 = Core.float64ToString(this.value);
        builder__773.append(t_4795);
    }
    public SqlFloat64(double value__776) {
        this.value = value__776;
    }
    public double getValue() {
        return this.value;
    }
}
