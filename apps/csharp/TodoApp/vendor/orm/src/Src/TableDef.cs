using S = System;
using G = System.Collections.Generic;
namespace Orm.Src
{
    public class TableDef
    {
        readonly ISafeIdentifier tableName__646;
        readonly G::IReadOnlyList<FieldDef> fields__647;
        public FieldDef Field(string name__649)
        {
            FieldDef return__241;
            {
                {
                    G::IReadOnlyList<FieldDef> this__2984 = this.fields__647;
                    int n__2985 = this__2984.Count;
                    int i__2986 = 0;
                    while (i__2986 < n__2985)
                    {
                        FieldDef el__2987 = this__2984[i__2986];
                        i__2986 = i__2986 + 1;
                        FieldDef f__651 = el__2987;
                        if (f__651.Name.SqlValue == name__649)
                        {
                            return__241 = f__651;
                            goto fn__650;
                        }
                    }
                    throw new S::Exception();
                }
                fn__650:
                {
                }
            }
            return return__241;
        }
        public TableDef(ISafeIdentifier tableName__653, G::IReadOnlyList<FieldDef> fields__654)
        {
            this.tableName__646 = tableName__653;
            this.fields__647 = fields__654;
        }
        public ISafeIdentifier TableName
        {
            get
            {
                return this.tableName__646;
            }
        }
        public G::IReadOnlyList<FieldDef> Fields
        {
            get
            {
                return this.fields__647;
            }
        }
    }
}
