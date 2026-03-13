package orm.src;
import java.util.List;
import temper.core.Core;
public final class TableDef {
    public final SafeIdentifier tableName;
    public final List<FieldDef> fields;
    public FieldDef field(String name__649) {
        FieldDef return__241;
        fn__650: {
            List<FieldDef> this__2984 = this.fields;
            int n__2985 = this__2984.size();
            int i__2986 = 0;
            while (i__2986 < n__2985) {
                FieldDef el__2987 = Core.listGet(this__2984, i__2986);
                i__2986 = i__2986 + 1;
                FieldDef f__651 = el__2987;
                if (f__651.getName().getSqlValue().equals(name__649)) {
                    return__241 = f__651;
                    break fn__650;
                }
            }
            throw Core.bubble();
        }
        return return__241;
    }
    public static final class Builder {
        SafeIdentifier tableName;
        public Builder tableName(SafeIdentifier tableName) {
            this.tableName = tableName;
            return this;
        }
        List<FieldDef> fields;
        public Builder fields(List<FieldDef> fields) {
            this.fields = fields;
            return this;
        }
        public TableDef build() {
            return new TableDef(tableName, fields);
        }
    }
    public TableDef(SafeIdentifier tableName__653, List<FieldDef> fields__654) {
        this.tableName = tableName__653;
        this.fields = fields__654;
    }
    public SafeIdentifier getTableName() {
        return this.tableName;
    }
    public List<FieldDef> getFields() {
        return this.fields;
    }
}
