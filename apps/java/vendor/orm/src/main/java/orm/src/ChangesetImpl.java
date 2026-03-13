package orm.src;
import temper.core.Core;
import java.util.List;
import java.util.Map;
import java.util.ArrayList;
import java.util.Map.Entry;
import java.util.function.Consumer;
import java.time.LocalDate;
import java.util.function.Function;
import java.util.LinkedHashMap;
final class ChangesetImpl implements Changeset {
    final TableDef _tableDef;
    final Map<String, String> _params;
    final Map<String, String> _changes;
    final List<ChangesetError> _errors;
    final boolean _isValid;
    public TableDef getTableDef() {
        return this._tableDef;
    }
    public Map<String, String> getChanges() {
        return this._changes;
    }
    public List<ChangesetError> getErrors() {
        return this._errors;
    }
    public boolean isValid() {
        return this._isValid;
    }
    public Changeset cast(List<SafeIdentifier> allowedFields__355) {
        Map<String, String> mb__357 = new LinkedHashMap<>();
        Consumer<SafeIdentifier> fn__4731 = f__358 -> {
            String t_4729;
            String t_4726 = f__358.getSqlValue();
            String val__359 = this._params.getOrDefault(t_4726, "");
            if (!val__359.isEmpty()) {
                t_4729 = f__358.getSqlValue();
                mb__357.put(t_4729, val__359);
            }
        };
        allowedFields__355.forEach(fn__4731);
        return new ChangesetImpl(this._tableDef, this._params, Core.mappedToMap(mb__357), this._errors, this._isValid);
    }
    public Changeset validateRequired(List<SafeIdentifier> fields__361) {
        Changeset return__183;
        List<ChangesetError> t_4724;
        TableDef t_2816;
        Map<String, String> t_2817;
        Map<String, String> t_2818;
        fn__362: {
            if (!this._isValid) {
                return__183 = this;
                break fn__362;
            }
            List<ChangesetError> eb__363 = new ArrayList<>(this._errors);
            class Local_1 {
                boolean valid__364 = true;
            }
            final Local_1 local$1 = new Local_1();
            Consumer<SafeIdentifier> fn__4720 = f__365 -> {
                ChangesetError t_4718;
                String t_4715 = f__365.getSqlValue();
                if (!this._changes.containsKey(t_4715)) {
                    t_4718 = new ChangesetError(f__365.getSqlValue(), "is required");
                    Core.listAdd(eb__363, t_4718);
                    local$1.valid__364 = false;
                }
            };
            fields__361.forEach(fn__4720);
            t_2816 = this._tableDef;
            t_2817 = this._params;
            t_2818 = this._changes;
            t_4724 = List.copyOf(eb__363);
            return__183 = new ChangesetImpl(t_2816, t_2817, t_2818, t_4724, local$1.valid__364);
        }
        return return__183;
    }
    public Changeset validateLength(SafeIdentifier field__367, int min__368, int max__369) {
        Changeset return__184;
        String t_4702;
        List<ChangesetError> t_4713;
        boolean t_2799;
        TableDef t_2805;
        Map<String, String> t_2806;
        Map<String, String> t_2807;
        fn__370: {
            if (!this._isValid) {
                return__184 = this;
                break fn__370;
            }
            t_4702 = field__367.getSqlValue();
            String val__371 = this._changes.getOrDefault(t_4702, "");
            int len__372 = Core.stringCountBetween(val__371, 0, val__371.length());
            if (len__372 < min__368) {
                t_2799 = true;
            } else {
                t_2799 = len__372 > max__369;
            }
            if (t_2799) {
                String msg__373 = "must be between " + Integer.toString(min__368) + " and " + Integer.toString(max__369) + " characters";
                List<ChangesetError> eb__374 = new ArrayList<>(this._errors);
                Core.listAdd(eb__374, new ChangesetError(field__367.getSqlValue(), msg__373));
                t_2805 = this._tableDef;
                t_2806 = this._params;
                t_2807 = this._changes;
                t_4713 = List.copyOf(eb__374);
                return__184 = new ChangesetImpl(t_2805, t_2806, t_2807, t_4713, false);
                break fn__370;
            }
            return__184 = this;
        }
        return return__184;
    }
    public Changeset validateInt(SafeIdentifier field__376) {
        Changeset return__185;
        String t_4693;
        List<ChangesetError> t_4700;
        TableDef t_2790;
        Map<String, String> t_2791;
        Map<String, String> t_2792;
        fn__377: {
            if (!this._isValid) {
                return__185 = this;
                break fn__377;
            }
            t_4693 = field__376.getSqlValue();
            String val__378 = this._changes.getOrDefault(t_4693, "");
            if (val__378.isEmpty()) {
                return__185 = this;
                break fn__377;
            }
            boolean parseOk__379;
            boolean parseOk_4834;
            try {
                Core.stringToInt(val__378);
                parseOk_4834 = true;
            } catch (RuntimeException ignored$1) {
                parseOk_4834 = false;
            }
            parseOk__379 = parseOk_4834;
            if (!parseOk__379) {
                List<ChangesetError> eb__380 = new ArrayList<>(this._errors);
                Core.listAdd(eb__380, new ChangesetError(field__376.getSqlValue(), "must be an integer"));
                t_2790 = this._tableDef;
                t_2791 = this._params;
                t_2792 = this._changes;
                t_4700 = List.copyOf(eb__380);
                return__185 = new ChangesetImpl(t_2790, t_2791, t_2792, t_4700, false);
                break fn__377;
            }
            return__185 = this;
        }
        return return__185;
    }
    public Changeset validateInt64(SafeIdentifier field__382) {
        Changeset return__186;
        String t_4684;
        List<ChangesetError> t_4691;
        TableDef t_2777;
        Map<String, String> t_2778;
        Map<String, String> t_2779;
        fn__383: {
            if (!this._isValid) {
                return__186 = this;
                break fn__383;
            }
            t_4684 = field__382.getSqlValue();
            String val__384 = this._changes.getOrDefault(t_4684, "");
            if (val__384.isEmpty()) {
                return__186 = this;
                break fn__383;
            }
            boolean parseOk__385;
            boolean parseOk_4836;
            try {
                Core.stringToInt64(val__384);
                parseOk_4836 = true;
            } catch (RuntimeException ignored$2) {
                parseOk_4836 = false;
            }
            parseOk__385 = parseOk_4836;
            if (!parseOk__385) {
                List<ChangesetError> eb__386 = new ArrayList<>(this._errors);
                Core.listAdd(eb__386, new ChangesetError(field__382.getSqlValue(), "must be a 64-bit integer"));
                t_2777 = this._tableDef;
                t_2778 = this._params;
                t_2779 = this._changes;
                t_4691 = List.copyOf(eb__386);
                return__186 = new ChangesetImpl(t_2777, t_2778, t_2779, t_4691, false);
                break fn__383;
            }
            return__186 = this;
        }
        return return__186;
    }
    public Changeset validateFloat(SafeIdentifier field__388) {
        Changeset return__187;
        String t_4675;
        List<ChangesetError> t_4682;
        TableDef t_2764;
        Map<String, String> t_2765;
        Map<String, String> t_2766;
        fn__389: {
            if (!this._isValid) {
                return__187 = this;
                break fn__389;
            }
            t_4675 = field__388.getSqlValue();
            String val__390 = this._changes.getOrDefault(t_4675, "");
            if (val__390.isEmpty()) {
                return__187 = this;
                break fn__389;
            }
            boolean parseOk__391;
            boolean parseOk_4838;
            try {
                Core.stringToFloat64(val__390);
                parseOk_4838 = true;
            } catch (RuntimeException ignored$3) {
                parseOk_4838 = false;
            }
            parseOk__391 = parseOk_4838;
            if (!parseOk__391) {
                List<ChangesetError> eb__392 = new ArrayList<>(this._errors);
                Core.listAdd(eb__392, new ChangesetError(field__388.getSqlValue(), "must be a number"));
                t_2764 = this._tableDef;
                t_2765 = this._params;
                t_2766 = this._changes;
                t_4682 = List.copyOf(eb__392);
                return__187 = new ChangesetImpl(t_2764, t_2765, t_2766, t_4682, false);
                break fn__389;
            }
            return__187 = this;
        }
        return return__187;
    }
    public Changeset validateBool(SafeIdentifier field__394) {
        Changeset return__188;
        String t_4666;
        List<ChangesetError> t_4673;
        boolean t_2739;
        boolean t_2740;
        boolean t_2742;
        boolean t_2743;
        boolean t_2745;
        TableDef t_2751;
        Map<String, String> t_2752;
        Map<String, String> t_2753;
        fn__395: {
            if (!this._isValid) {
                return__188 = this;
                break fn__395;
            }
            t_4666 = field__394.getSqlValue();
            String val__396 = this._changes.getOrDefault(t_4666, "");
            if (val__396.isEmpty()) {
                return__188 = this;
                break fn__395;
            }
            boolean isTrue__397;
            if (val__396.equals("true")) {
                isTrue__397 = true;
            } else {
                if (val__396.equals("1")) {
                    t_2740 = true;
                } else {
                    if (val__396.equals("yes")) {
                        t_2739 = true;
                    } else {
                        t_2739 = val__396.equals("on");
                    }
                    t_2740 = t_2739;
                }
                isTrue__397 = t_2740;
            }
            boolean isFalse__398;
            if (val__396.equals("false")) {
                isFalse__398 = true;
            } else {
                if (val__396.equals("0")) {
                    t_2743 = true;
                } else {
                    if (val__396.equals("no")) {
                        t_2742 = true;
                    } else {
                        t_2742 = val__396.equals("off");
                    }
                    t_2743 = t_2742;
                }
                isFalse__398 = t_2743;
            }
            if (!isTrue__397) {
                t_2745 = !isFalse__398;
            } else {
                t_2745 = false;
            }
            if (t_2745) {
                List<ChangesetError> eb__399 = new ArrayList<>(this._errors);
                Core.listAdd(eb__399, new ChangesetError(field__394.getSqlValue(), "must be a boolean (true/false/1/0/yes/no/on/off)"));
                t_2751 = this._tableDef;
                t_2752 = this._params;
                t_2753 = this._changes;
                t_4673 = List.copyOf(eb__399);
                return__188 = new ChangesetImpl(t_2751, t_2752, t_2753, t_4673, false);
                break fn__395;
            }
            return__188 = this;
        }
        return return__188;
    }
    SqlBoolean parseBoolSqlPart(String val__401) {
        SqlBoolean return__189;
        boolean t_2728;
        boolean t_2729;
        boolean t_2730;
        boolean t_2732;
        boolean t_2733;
        boolean t_2734;
        fn__402: {
            if (val__401.equals("true")) {
                t_2730 = true;
            } else {
                if (val__401.equals("1")) {
                    t_2729 = true;
                } else {
                    if (val__401.equals("yes")) {
                        t_2728 = true;
                    } else {
                        t_2728 = val__401.equals("on");
                    }
                    t_2729 = t_2728;
                }
                t_2730 = t_2729;
            }
            if (t_2730) {
                return__189 = new SqlBoolean(true);
                break fn__402;
            }
            if (val__401.equals("false")) {
                t_2734 = true;
            } else {
                if (val__401.equals("0")) {
                    t_2733 = true;
                } else {
                    if (val__401.equals("no")) {
                        t_2732 = true;
                    } else {
                        t_2732 = val__401.equals("off");
                    }
                    t_2733 = t_2732;
                }
                t_2734 = t_2733;
            }
            if (t_2734) {
                return__189 = new SqlBoolean(false);
                break fn__402;
            }
            throw Core.bubble();
        }
        return return__189;
    }
    SqlPart valueToSqlPart(FieldDef fieldDef__404, String val__405) {
        SqlPart return__190;
        int t_2715;
        long t_2718;
        double t_2721;
        LocalDate t_2726;
        fn__406: {
            FieldType ft__407 = fieldDef__404.getFieldType();
            if (ft__407 instanceof StringField) {
                return__190 = new SqlString(val__405);
                break fn__406;
            }
            if (ft__407 instanceof IntField) {
                t_2715 = Core.stringToInt(val__405);
                return__190 = new SqlInt32(t_2715);
                break fn__406;
            }
            if (ft__407 instanceof Int64Field) {
                t_2718 = Core.stringToInt64(val__405);
                return__190 = new SqlInt64(t_2718);
                break fn__406;
            }
            if (ft__407 instanceof FloatField) {
                t_2721 = Core.stringToFloat64(val__405);
                return__190 = new SqlFloat64(t_2721);
                break fn__406;
            }
            if (ft__407 instanceof BoolField) {
                return__190 = this.parseBoolSqlPart(val__405);
                break fn__406;
            }
            if (ft__407 instanceof DateField) {
                t_2726 = LocalDate.parse(val__405);
                return__190 = new SqlDate(t_2726);
                break fn__406;
            }
            throw Core.bubble();
        }
        return return__190;
    }
    public SqlFragment toInsertSql() {
        int t_4615;
        String t_4620;
        boolean t_4621;
        int t_4626;
        String t_4628;
        String t_4631;
        int t_4646;
        boolean t_2680;
        FieldDef t_2688;
        SqlPart t_2692;
        if (!this._isValid) {
            throw Core.bubble();
        }
        int i__410 = 0;
        while (true) {
            t_4615 = this._tableDef.getFields().size();
            if (i__410 >= t_4615) {
                break;
            }
            FieldDef f__411 = Core.listGet(this._tableDef.getFields(), i__410);
            if (!f__411.isNullable()) {
                t_4620 = f__411.getName().getSqlValue();
                t_4621 = this._changes.containsKey(t_4620);
                t_2680 = !t_4621;
            } else {
                t_2680 = false;
            }
            if (t_2680) {
                throw Core.bubble();
            }
            i__410 = i__410 + 1;
        }
        List<Entry<String, String>> pairs__412 = Core.mappedToList(this._changes);
        if (pairs__412.size() == 0) {
            throw Core.bubble();
        }
        List<String> colNames__413 = new ArrayList<>();
        List<SqlPart> valParts__414 = new ArrayList<>();
        int i__415 = 0;
        while (true) {
            t_4626 = pairs__412.size();
            if (i__415 >= t_4626) {
                break;
            }
            Entry<String, String> pair__416 = Core.listGet(pairs__412, i__415);
            t_4628 = pair__416.getKey();
            t_2688 = this._tableDef.field(t_4628);
            FieldDef fd__417 = t_2688;
            Core.listAdd(colNames__413, pair__416.getKey());
            t_4631 = pair__416.getValue();
            t_2692 = this.valueToSqlPart(fd__417, t_4631);
            Core.listAdd(valParts__414, t_2692);
            i__415 = i__415 + 1;
        }
        SqlBuilder b__418 = new SqlBuilder();
        b__418.appendSafe("INSERT INTO ");
        b__418.appendSafe(this._tableDef.getTableName().getSqlValue());
        b__418.appendSafe(" (");
        List<String> t_4639 = List.copyOf(colNames__413);
        Function<String, String> fn__4613 = c__419 -> c__419;
        b__418.appendSafe(Core.listJoinObj(t_4639, ", ", fn__4613));
        b__418.appendSafe(") VALUES (");
        b__418.appendPart(Core.listGet(valParts__414, 0));
        int j__420 = 1;
        while (true) {
            t_4646 = valParts__414.size();
            if (j__420 >= t_4646) {
                break;
            }
            b__418.appendSafe(", ");
            b__418.appendPart(Core.listGet(valParts__414, j__420));
            j__420 = j__420 + 1;
        }
        b__418.appendSafe(")");
        return b__418.getAccumulated();
    }
    public SqlFragment toUpdateSql(int id__422) {
        int t_4601;
        String t_4604;
        String t_4608;
        FieldDef t_2662;
        SqlPart t_2667;
        if (!this._isValid) {
            throw Core.bubble();
        }
        List<Entry<String, String>> pairs__424 = Core.mappedToList(this._changes);
        if (pairs__424.size() == 0) {
            throw Core.bubble();
        }
        SqlBuilder b__425 = new SqlBuilder();
        b__425.appendSafe("UPDATE ");
        b__425.appendSafe(this._tableDef.getTableName().getSqlValue());
        b__425.appendSafe(" SET ");
        int i__426 = 0;
        while (true) {
            t_4601 = pairs__424.size();
            if (i__426 >= t_4601) {
                break;
            }
            if (i__426 > 0) {
                b__425.appendSafe(", ");
            }
            Entry<String, String> pair__427 = Core.listGet(pairs__424, i__426);
            t_4604 = pair__427.getKey();
            t_2662 = this._tableDef.field(t_4604);
            FieldDef fd__428 = t_2662;
            b__425.appendSafe(pair__427.getKey());
            b__425.appendSafe(" = ");
            t_4608 = pair__427.getValue();
            t_2667 = this.valueToSqlPart(fd__428, t_4608);
            b__425.appendPart(t_2667);
            i__426 = i__426 + 1;
        }
        b__425.appendSafe(" WHERE id = ");
        b__425.appendInt32(id__422);
        return b__425.getAccumulated();
    }
    public static final class Builder {
        TableDef _tableDef;
        public Builder _tableDef(TableDef _tableDef) {
            this._tableDef = _tableDef;
            return this;
        }
        Map<String, String> _params;
        public Builder _params(Map<String, String> _params) {
            this._params = _params;
            return this;
        }
        Map<String, String> _changes;
        public Builder _changes(Map<String, String> _changes) {
            this._changes = _changes;
            return this;
        }
        List<ChangesetError> _errors;
        public Builder _errors(List<ChangesetError> _errors) {
            this._errors = _errors;
            return this;
        }
        boolean _isValid;
        boolean _isValid__set;
        public Builder _isValid(boolean _isValid) {
            _isValid__set = true;
            this._isValid = _isValid;
            return this;
        }
        public ChangesetImpl build() {
            if (!_isValid__set || _tableDef == null || _params == null || _changes == null || _errors == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!_isValid__set) {
                    _message.append(" _isValid");
                }
                if (_tableDef == null) {
                    _message.append(" _tableDef");
                }
                if (_params == null) {
                    _message.append(" _params");
                }
                if (_changes == null) {
                    _message.append(" _changes");
                }
                if (_errors == null) {
                    _message.append(" _errors");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new ChangesetImpl(_tableDef, _params, _changes, _errors, _isValid);
        }
    }
    public ChangesetImpl(TableDef _tableDef__430, Map<String, String> _params__431, Map<String, String> _changes__432, List<ChangesetError> _errors__433, boolean _isValid__434) {
        this._tableDef = _tableDef__430;
        this._params = _params__431;
        this._changes = _changes__432;
        this._errors = _errors__433;
        this._isValid = _isValid__434;
    }
}
