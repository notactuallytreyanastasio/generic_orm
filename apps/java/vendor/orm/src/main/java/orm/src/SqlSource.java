package orm.src;
/**
 * `SqlSource` represents known-safe SQL source code that doesn't need escaped.
 */
public final class SqlSource implements SqlPart {
    public final String source;
    public void formatTo(StringBuilder builder__755) {
        builder__755.append(this.source);
    }
    public SqlSource(String source__758) {
        this.source = source__758;
    }
    public String getSource() {
        return this.source;
    }
}
