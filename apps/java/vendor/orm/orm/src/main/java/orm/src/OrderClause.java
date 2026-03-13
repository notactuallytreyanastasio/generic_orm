package orm.src;
public final class OrderClause {
    public final SafeIdentifier field;
    public final boolean ascending;
    public static final class Builder {
        SafeIdentifier field;
        public Builder field(SafeIdentifier field) {
            this.field = field;
            return this;
        }
        boolean ascending;
        boolean ascending__set;
        public Builder ascending(boolean ascending) {
            ascending__set = true;
            this.ascending = ascending;
            return this;
        }
        public OrderClause build() {
            if (!ascending__set || field == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!ascending__set) {
                    _message.append(" ascending");
                }
                if (field == null) {
                    _message.append(" field");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new OrderClause(field, ascending);
        }
    }
    public OrderClause(SafeIdentifier field__532, boolean ascending__533) {
        this.field = field__532;
        this.ascending = ascending__533;
    }
    public SafeIdentifier getField() {
        return this.field;
    }
    public boolean isAscending() {
        return this.ascending;
    }
}
