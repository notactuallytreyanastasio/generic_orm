package orm.src;
import java.time.LocalDate;
public final class SqlDate implements SqlPart {
    public final LocalDate value;
    public void formatTo(StringBuilder builder__767) {
        builder__767.append("'");
        String t_4792 = this.value.toString();
        builder__767.append(t_4792);
        builder__767.append("'");
    }
    public SqlDate(LocalDate value__770) {
        this.value = value__770;
    }
    public LocalDate getValue() {
        return this.value;
    }
}
