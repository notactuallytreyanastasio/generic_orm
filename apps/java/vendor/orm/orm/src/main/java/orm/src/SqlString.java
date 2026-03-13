package orm.src;
import temper.core.Core;
import java.util.function.IntConsumer;
/**
 * `SqlString` represents text data that needs escaped.
 */
public final class SqlString implements SqlPart {
    public final String value;
    public void formatTo(StringBuilder builder__791) {
        builder__791.append("'");
        IntConsumer fn__4802 = c__793 -> {
            if (c__793 == 39) {
                builder__791.append("''");
            } else {
                Core.stringBuilderAppendCodePoint(builder__791, c__793);
            }
        };
        Core.stringForEach(this.value, fn__4802);
        builder__791.append("'");
    }
    public SqlString(String value__795) {
        this.value = value__795;
    }
    public String getValue() {
        return this.value;
    }
}
