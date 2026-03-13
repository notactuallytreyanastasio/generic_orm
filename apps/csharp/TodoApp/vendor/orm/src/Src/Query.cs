using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace Orm.Src
{
    public class Query
    {
        readonly ISafeIdentifier tableName__534;
        readonly G::IReadOnlyList<SqlFragment> conditions__535;
        readonly G::IReadOnlyList<ISafeIdentifier> selectedFields__536;
        readonly G::IReadOnlyList<OrderClause> orderClauses__537;
        readonly int ? limitVal__538;
        readonly int ? offsetVal__539;
        public Query Where(SqlFragment condition__541)
        {
            G::IList<SqlFragment> nb__543 = L::Enumerable.ToList(this.conditions__535);
            C::Listed.Add(nb__543, condition__541);
            return new Query(this.tableName__534, C::Listed.ToReadOnlyList(nb__543), this.selectedFields__536, this.orderClauses__537, this.limitVal__538, this.offsetVal__539);
        }
        public Query Select(G::IReadOnlyList<ISafeIdentifier> fields__545)
        {
            return new Query(this.tableName__534, this.conditions__535, fields__545, this.orderClauses__537, this.limitVal__538, this.offsetVal__539);
        }
        public Query OrderBy(ISafeIdentifier field__548, bool ascending__549)
        {
            G::IList<OrderClause> nb__551 = L::Enumerable.ToList(this.orderClauses__537);
            C::Listed.Add(nb__551, new OrderClause(field__548, ascending__549));
            return new Query(this.tableName__534, this.conditions__535, this.selectedFields__536, C::Listed.ToReadOnlyList(nb__551), this.limitVal__538, this.offsetVal__539);
        }
        public Query Limit(int n__553)
        {
            if (n__553 < 0) throw new S::Exception();
            return new Query(this.tableName__534, this.conditions__535, this.selectedFields__536, this.orderClauses__537, n__553, this.offsetVal__539);
        }
        public Query Offset(int n__556)
        {
            if (n__556 < 0) throw new S::Exception();
            return new Query(this.tableName__534, this.conditions__535, this.selectedFields__536, this.orderClauses__537, this.limitVal__538, n__556);
        }
        public SqlFragment ToSql()
        {
            int t___4185;
            SqlBuilder b__560 = new SqlBuilder();
            b__560.AppendSafe("SELECT ");
            if (this.selectedFields__536.Count == 0) b__560.AppendSafe("*");
            else
            {
                string fn__4170(ISafeIdentifier f__561)
                {
                    return f__561.SqlValue;
                }
                b__560.AppendSafe(C::Listed.Join(this.selectedFields__536, ", ", (S::Func<ISafeIdentifier, string>) fn__4170));
            }
            b__560.AppendSafe(" FROM ");
            b__560.AppendSafe(this.tableName__534.SqlValue);
            if (!(this.conditions__535.Count == 0))
            {
                b__560.AppendSafe(" WHERE ");
                b__560.AppendFragment(this.conditions__535[0]);
                int i__562 = 1;
                while (true)
                {
                    t___4185 = this.conditions__535.Count;
                    if (!(i__562 < t___4185)) break;
                    b__560.AppendSafe(" AND ");
                    b__560.AppendFragment(this.conditions__535[i__562]);
                    i__562 = i__562 + 1;
                }
            }
            if (!(this.orderClauses__537.Count == 0))
            {
                b__560.AppendSafe(" ORDER BY ");
                bool first__563 = true;
                void fn__4169(OrderClause oc__564)
                {
                    string t___2283;
                    if (!first__563) b__560.AppendSafe(", ");
                    first__563 = false;
                    string t___4164 = oc__564.Field.SqlValue;
                    b__560.AppendSafe(t___4164);
                    if (oc__564.Ascending)
                    {
                        t___2283 = " ASC";
                    }
                    else
                    {
                        t___2283 = " DESC";
                    }
                    b__560.AppendSafe(t___2283);
                }
                C::Listed.ForEach(this.orderClauses__537, (S::Action<OrderClause>) fn__4169);
            }
            int ? lv__565 = this.limitVal__538;
            if (!(lv__565 == null))
            {
                int lv___1068 = lv__565.Value;
                b__560.AppendSafe(" LIMIT ");
                b__560.AppendInt32(lv___1068);
            }
            int ? ov__566 = this.offsetVal__539;
            if (!(ov__566 == null))
            {
                int ov___1069 = ov__566.Value;
                b__560.AppendSafe(" OFFSET ");
                b__560.AppendInt32(ov___1069);
            }
            return b__560.Accumulated;
        }
        public SqlFragment SafeToSql(int defaultLimit__568)
        {
            SqlFragment return__212;
            Query t___2276;
            if (defaultLimit__568 < 0) throw new S::Exception();
            if (!(this.limitVal__538 == null))
            {
                return__212 = this.ToSql();
            }
            else
            {
                t___2276 = this.Limit(defaultLimit__568);
                return__212 = t___2276.ToSql();
            }
            return return__212;
        }
        public Query(ISafeIdentifier tableName__571, G::IReadOnlyList<SqlFragment> conditions__572, G::IReadOnlyList<ISafeIdentifier> selectedFields__573, G::IReadOnlyList<OrderClause> orderClauses__574, int ? limitVal__575, int ? offsetVal__576)
        {
            this.tableName__534 = tableName__571;
            this.conditions__535 = conditions__572;
            this.selectedFields__536 = selectedFields__573;
            this.orderClauses__537 = orderClauses__574;
            this.limitVal__538 = limitVal__575;
            this.offsetVal__539 = offsetVal__576;
        }
        public ISafeIdentifier TableName
        {
            get
            {
                return this.tableName__534;
            }
        }
        public G::IReadOnlyList<SqlFragment> Conditions
        {
            get
            {
                return this.conditions__535;
            }
        }
        public G::IReadOnlyList<ISafeIdentifier> SelectedFields
        {
            get
            {
                return this.selectedFields__536;
            }
        }
        public G::IReadOnlyList<OrderClause> OrderClauses
        {
            get
            {
                return this.orderClauses__537;
            }
        }
        public int ? LimitVal
        {
            get
            {
                return this.limitVal__538;
            }
        }
        public int ? OffsetVal
        {
            get
            {
                return this.offsetVal__539;
            }
        }
    }
}
