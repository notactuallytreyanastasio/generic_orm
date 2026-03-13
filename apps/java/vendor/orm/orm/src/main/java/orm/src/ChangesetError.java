package orm.src;
public final class ChangesetError {
    public final String field;
    public final String message;
    public static final class Builder {
        String field;
        public Builder field(String field) {
            this.field = field;
            return this;
        }
        String message;
        public Builder message(String message) {
            this.message = message;
            return this;
        }
        public ChangesetError build() {
            return new ChangesetError(field, message);
        }
    }
    public ChangesetError(String field__303, String message__304) {
        this.field = field__303;
        this.message = message__304;
    }
    public String getField() {
        return this.field;
    }
    public String getMessage() {
        return this.message;
    }
}
