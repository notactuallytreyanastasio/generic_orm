package orm.src;
public final class SqlBoolean implements SqlPart {
    public final boolean value;
    public void formatTo(StringBuilder builder__761) {
        String t_2868;
        if (this.value) {
            t_2868 = "TRUE";
        } else {
            t_2868 = "FALSE";
        }
        builder__761.append(t_2868);
    }
    public SqlBoolean(boolean value__764) {
        this.value = value__764;
    }
    public boolean isValue() {
        return this.value;
    }
}
