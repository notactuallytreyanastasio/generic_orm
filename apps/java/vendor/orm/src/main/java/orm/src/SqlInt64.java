package orm.src;
public final class SqlInt64 implements SqlPart {
    public final long value;
    public void formatTo(StringBuilder builder__785) {
        String t_4797 = Long.toString(this.value);
        builder__785.append(t_4797);
    }
    public SqlInt64(long value__788) {
        this.value = value__788;
    }
    public long getValue() {
        return this.value;
    }
}
