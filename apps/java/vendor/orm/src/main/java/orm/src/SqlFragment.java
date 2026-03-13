package orm.src;
import java.util.List;
import temper.core.Core;
public final class SqlFragment {
    public final List<SqlPart> parts;
    public SqlSource toSource() {
        return new SqlSource(this.toString());
    }
    public String toString() {
        int t_4809;
        StringBuilder builder__746 = new StringBuilder();
        int i__747 = 0;
        while (true) {
            t_4809 = this.parts.size();
            if (i__747 >= t_4809) {
                break;
            }
            Core.listGet(this.parts, i__747).formatTo(builder__746);
            i__747 = i__747 + 1;
        }
        return builder__746.toString();
    }
    public SqlFragment(List<SqlPart> parts__749) {
        this.parts = parts__749;
    }
    public List<SqlPart> getParts() {
        return this.parts;
    }
}
