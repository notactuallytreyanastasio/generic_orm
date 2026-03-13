package orm.src;
public final class FieldDef {
    public final SafeIdentifier name;
    public final FieldType fieldType;
    public final boolean nullable;
    public static final class Builder {
        SafeIdentifier name;
        public Builder name(SafeIdentifier name) {
            this.name = name;
            return this;
        }
        FieldType fieldType;
        public Builder fieldType(FieldType fieldType) {
            this.fieldType = fieldType;
            return this;
        }
        boolean nullable;
        boolean nullable__set;
        public Builder nullable(boolean nullable) {
            nullable__set = true;
            this.nullable = nullable;
            return this;
        }
        public FieldDef build() {
            if (!nullable__set || name == null || fieldType == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!nullable__set) {
                    _message.append(" nullable");
                }
                if (name == null) {
                    _message.append(" name");
                }
                if (fieldType == null) {
                    _message.append(" fieldType");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new FieldDef(name, fieldType, nullable);
        }
    }
    public FieldDef(SafeIdentifier name__643, FieldType fieldType__644, boolean nullable__645) {
        this.name = name__643;
        this.fieldType = fieldType__644;
        this.nullable = nullable__645;
    }
    public SafeIdentifier getName() {
        return this.name;
    }
    public FieldType getFieldType() {
        return this.fieldType;
    }
    public boolean isNullable() {
        return this.nullable;
    }
}
