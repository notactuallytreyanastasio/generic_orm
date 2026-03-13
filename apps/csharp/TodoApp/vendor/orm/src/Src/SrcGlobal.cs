using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Orm.Src
{
    public static class SrcGlobal
    {
        public static IChangeset Changeset(TableDef tableDef__435, G::IReadOnlyDictionary<string, string> params__436)
        {
            G::IReadOnlyDictionary<string, string> t___4591 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>());
            return new ChangesetImpl(tableDef__435, params__436, t___4591, C::Listed.CreateReadOnlyList<ChangesetError>(), true);
        }
        internal static bool isIdentStart__296(int c__626)
        {
            bool return__221;
            bool t___2636;
            bool t___2637;
            if (c__626 >= 97)
            {
                t___2636 = c__626 <= 122;
            }
            else
            {
                t___2636 = false;
            }
            if (t___2636)
            {
                return__221 = true;
            }
            else
            {
                if (c__626 >= 65)
                {
                    t___2637 = c__626 <= 90;
                }
                else
                {
                    t___2637 = false;
                }
                if (t___2637)
                {
                    return__221 = true;
                }
                else
                {
                    return__221 = c__626 == 95;
                }
            }
            return return__221;
        }
        internal static bool isIdentPart__297(int c__628)
        {
            bool return__222;
            if (isIdentStart__296(c__628))
            {
                return__222 = true;
            }
            else if (c__628 >= 48)
            {
                return__222 = c__628 <= 57;
            }
            else
            {
                return__222 = false;
            }
            return return__222;
        }
        public static ISafeIdentifier SafeIdentifier(string name__630)
        {
            int t___4589;
            if (string.IsNullOrEmpty(name__630)) throw new S::Exception();
            int idx__632 = 0;
            if (!isIdentStart__296(C::StringUtil.Get(name__630, idx__632))) throw new S::Exception();
            int t___4586 = C::StringUtil.Next(name__630, idx__632);
            idx__632 = t___4586;
            while (true)
            {
                if (!C::StringUtil.HasIndex(name__630, idx__632)) break;
                if (!isIdentPart__297(C::StringUtil.Get(name__630, idx__632))) throw new S::Exception();
                t___4589 = C::StringUtil.Next(name__630, idx__632);
                idx__632 = t___4589;
            }
            return new ValidatedIdentifier(name__630);
        }
        public static SqlFragment DeleteSql(TableDef tableDef__525, int id__526)
        {
            SqlBuilder b__528 = new SqlBuilder();
            b__528.AppendSafe("DELETE FROM ");
            b__528.AppendSafe(tableDef__525.TableName.SqlValue);
            b__528.AppendSafe(" WHERE id = ");
            b__528.AppendInt32(id__526);
            return b__528.Accumulated;
        }
        public static Query From(ISafeIdentifier tableName__577)
        {
            return new Query(tableName__577, C::Listed.CreateReadOnlyList<SqlFragment>(), C::Listed.CreateReadOnlyList<ISafeIdentifier>(), C::Listed.CreateReadOnlyList<OrderClause>(), null, null);
        }
    }
}
