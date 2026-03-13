package orm.src;
import java.util.List;
import temper.core.Nullable;
import temper.core.Core;
import java.util.ArrayList;
import java.util.function.Function;
import java.util.function.Consumer;
public final class Query {
    public final SafeIdentifier tableName;
    public final List<SqlFragment> conditions;
    public final List<SafeIdentifier> selectedFields;
    public final List<OrderClause> orderClauses;
    public final @Nullable Integer limitVal;
    public final @Nullable Integer offsetVal;
    public Query where(SqlFragment condition__541) {
        List<SqlFragment> nb__543 = new ArrayList<>(this.conditions);
        Core.listAdd(nb__543, condition__541);
        return new Query(this.tableName, List.copyOf(nb__543), this.selectedFields, this.orderClauses, this.limitVal, this.offsetVal);
    }
    public Query select(List<SafeIdentifier> fields__545) {
        return new Query(this.tableName, this.conditions, fields__545, this.orderClauses, this.limitVal, this.offsetVal);
    }
    public Query orderBy(SafeIdentifier field__548, boolean ascending__549) {
        List<OrderClause> nb__551 = new ArrayList<>(this.orderClauses);
        Core.listAdd(nb__551, new OrderClause(field__548, ascending__549));
        return new Query(this.tableName, this.conditions, this.selectedFields, List.copyOf(nb__551), this.limitVal, this.offsetVal);
    }
    public Query limit(int n__553) {
        if (n__553 < 0) {
            throw Core.bubble();
        }
        return new Query(this.tableName, this.conditions, this.selectedFields, this.orderClauses, n__553, this.offsetVal);
    }
    public Query offset(int n__556) {
        if (n__556 < 0) {
            throw Core.bubble();
        }
        return new Query(this.tableName, this.conditions, this.selectedFields, this.orderClauses, this.limitVal, n__556);
    }
    public SqlFragment toSql() {
        int t_4185;
        SqlBuilder b__560 = new SqlBuilder();
        b__560.appendSafe("SELECT ");
        if (this.selectedFields.isEmpty()) {
            b__560.appendSafe("*");
        } else {
            Function<SafeIdentifier, String> fn__4170 = f__561 -> f__561.getSqlValue();
            b__560.appendSafe(Core.listJoinObj(this.selectedFields, ", ", fn__4170));
        }
        b__560.appendSafe(" FROM ");
        b__560.appendSafe(this.tableName.getSqlValue());
        if (!this.conditions.isEmpty()) {
            b__560.appendSafe(" WHERE ");
            b__560.appendFragment(Core.listGet(this.conditions, 0));
            int i__562 = 1;
            while (true) {
                t_4185 = this.conditions.size();
                if (i__562 >= t_4185) {
                    break;
                }
                b__560.appendSafe(" AND ");
                b__560.appendFragment(Core.listGet(this.conditions, i__562));
                i__562 = i__562 + 1;
            }
        }
        if (!this.orderClauses.isEmpty()) {
            b__560.appendSafe(" ORDER BY ");
            class Local_2 {
                boolean first__563 = true;
            }
            final Local_2 local$2 = new Local_2();
            Consumer<OrderClause> fn__4169 = oc__564 -> {
                String t_2283;
                if (!local$2.first__563) {
                    b__560.appendSafe(", ");
                }
                local$2.first__563 = false;
                String t_4164 = oc__564.getField().getSqlValue();
                b__560.appendSafe(t_4164);
                if (oc__564.isAscending()) {
                    t_2283 = " ASC";
                } else {
                    t_2283 = " DESC";
                }
                b__560.appendSafe(t_2283);
            };
            this.orderClauses.forEach(fn__4169);
        }
        @Nullable Integer lv__565 = this.limitVal;
        if (lv__565 != null) {
            int lv_1068 = lv__565;
            b__560.appendSafe(" LIMIT ");
            b__560.appendInt32(lv_1068);
        }
        @Nullable Integer ov__566 = this.offsetVal;
        if (ov__566 != null) {
            int ov_1069 = ov__566;
            b__560.appendSafe(" OFFSET ");
            b__560.appendInt32(ov_1069);
        }
        return b__560.getAccumulated();
    }
    public SqlFragment safeToSql(int defaultLimit__568) {
        SqlFragment return__212;
        Query t_2276;
        if (defaultLimit__568 < 0) {
            throw Core.bubble();
        }
        if (this.limitVal != null) {
            return__212 = this.toSql();
        } else {
            t_2276 = this.limit(defaultLimit__568);
            return__212 = t_2276.toSql();
        }
        return return__212;
    }
    public static final class Builder {
        SafeIdentifier tableName;
        public Builder tableName(SafeIdentifier tableName) {
            this.tableName = tableName;
            return this;
        }
        List<SqlFragment> conditions;
        public Builder conditions(List<SqlFragment> conditions) {
            this.conditions = conditions;
            return this;
        }
        List<SafeIdentifier> selectedFields;
        public Builder selectedFields(List<SafeIdentifier> selectedFields) {
            this.selectedFields = selectedFields;
            return this;
        }
        List<OrderClause> orderClauses;
        public Builder orderClauses(List<OrderClause> orderClauses) {
            this.orderClauses = orderClauses;
            return this;
        }
        @Nullable Integer limitVal;
        boolean limitVal__set;
        public Builder limitVal(@Nullable Integer limitVal) {
            limitVal__set = true;
            this.limitVal = limitVal;
            return this;
        }
        @Nullable Integer offsetVal;
        boolean offsetVal__set;
        public Builder offsetVal(@Nullable Integer offsetVal) {
            offsetVal__set = true;
            this.offsetVal = offsetVal;
            return this;
        }
        public Query build() {
            if (!limitVal__set || !offsetVal__set || tableName == null || conditions == null || selectedFields == null || orderClauses == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!limitVal__set) {
                    _message.append(" limitVal");
                }
                if (!offsetVal__set) {
                    _message.append(" offsetVal");
                }
                if (tableName == null) {
                    _message.append(" tableName");
                }
                if (conditions == null) {
                    _message.append(" conditions");
                }
                if (selectedFields == null) {
                    _message.append(" selectedFields");
                }
                if (orderClauses == null) {
                    _message.append(" orderClauses");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new Query(tableName, conditions, selectedFields, orderClauses, limitVal, offsetVal);
        }
    }
    public Query(SafeIdentifier tableName__571, List<SqlFragment> conditions__572, List<SafeIdentifier> selectedFields__573, List<OrderClause> orderClauses__574, @Nullable Integer limitVal__575, @Nullable Integer offsetVal__576) {
        this.tableName = tableName__571;
        this.conditions = conditions__572;
        this.selectedFields = selectedFields__573;
        this.orderClauses = orderClauses__574;
        this.limitVal = limitVal__575;
        this.offsetVal = offsetVal__576;
    }
    public SafeIdentifier getTableName() {
        return this.tableName;
    }
    public List<SqlFragment> getConditions() {
        return this.conditions;
    }
    public List<SafeIdentifier> getSelectedFields() {
        return this.selectedFields;
    }
    public List<OrderClause> getOrderClauses() {
        return this.orderClauses;
    }
    public @Nullable Integer getLimitVal() {
        return this.limitVal;
    }
    public @Nullable Integer getOffsetVal() {
        return this.offsetVal;
    }
}
