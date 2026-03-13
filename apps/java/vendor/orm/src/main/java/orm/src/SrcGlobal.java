package orm.src;
import temper.core.Core;
import java.util.List;
import java.util.Map;
public final class SrcGlobal {
    private SrcGlobal() {
    }
    public static Changeset changeset(TableDef tableDef__435, Map<String, String> params__436) {
        Map<String, String> t_4591 = Core.mapConstructor(List.of());
        return new ChangesetImpl(tableDef__435, params__436, t_4591, List.of(), true);
    }
    static boolean isIdentStart__296(int c__626) {
        boolean return__221;
        boolean t_2636;
        boolean t_2637;
        if (c__626 >= 97) {
            t_2636 = c__626 <= 122;
        } else {
            t_2636 = false;
        }
        if (t_2636) {
            return__221 = true;
        } else {
            if (c__626 >= 65) {
                t_2637 = c__626 <= 90;
            } else {
                t_2637 = false;
            }
            if (t_2637) {
                return__221 = true;
            } else {
                return__221 = c__626 == 95;
            }
        }
        return return__221;
    }
    static boolean isIdentPart__297(int c__628) {
        boolean return__222;
        if (SrcGlobal.isIdentStart__296(c__628)) {
            return__222 = true;
        } else if (c__628 >= 48) {
            return__222 = c__628 <= 57;
        } else {
            return__222 = false;
        }
        return return__222;
    }
    public static SafeIdentifier safeIdentifier(String name__630) {
        int t_4589;
        if (name__630.isEmpty()) {
            throw Core.bubble();
        }
        int idx__632 = 0;
        if (!SrcGlobal.isIdentStart__296(name__630.codePointAt(idx__632))) {
            throw Core.bubble();
        }
        int t_4586 = Core.stringNext(name__630, idx__632);
        idx__632 = t_4586;
        while (true) {
            if (!Core.stringHasIndex(name__630, idx__632)) {
                break;
            }
            if (!SrcGlobal.isIdentPart__297(name__630.codePointAt(idx__632))) {
                throw Core.bubble();
            }
            t_4589 = Core.stringNext(name__630, idx__632);
            idx__632 = t_4589;
        }
        return new ValidatedIdentifier(name__630);
    }
    public static SqlFragment deleteSql(TableDef tableDef__525, int id__526) {
        SqlBuilder b__528 = new SqlBuilder();
        b__528.appendSafe("DELETE FROM ");
        b__528.appendSafe(tableDef__525.getTableName().getSqlValue());
        b__528.appendSafe(" WHERE id = ");
        b__528.appendInt32(id__526);
        return b__528.getAccumulated();
    }
    public static Query from(SafeIdentifier tableName__577) {
        return new Query(tableName__577, List.of(), List.of(), List.of(), null, null);
    }
}
