package orm.src;
public final class SqlInt32 implements SqlPart {
    public final int value;
    public void formatTo(StringBuilder builder__779) {
        String t_4799 = Integer.toString(this.value);
        builder__779.append(t_4799);
    }
    public SqlInt32(int value__782) {
        this.value = value__782;
    }
    public int getValue() {
        return this.value;
    }
}
