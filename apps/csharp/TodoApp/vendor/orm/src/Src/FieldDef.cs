namespace Orm.Src
{
    public class FieldDef
    {
        readonly ISafeIdentifier name__639;
        readonly IFieldType fieldType__640;
        readonly bool nullable__641;
        public FieldDef(ISafeIdentifier name__643, IFieldType fieldType__644, bool nullable__645)
        {
            this.name__639 = name__643;
            this.fieldType__640 = fieldType__644;
            this.nullable__641 = nullable__645;
        }
        public ISafeIdentifier Name
        {
            get
            {
                return this.name__639;
            }
        }
        public IFieldType FieldType
        {
            get
            {
                return this.fieldType__640;
            }
        }
        public bool Nullable
        {
            get
            {
                return this.nullable__641;
            }
        }
    }
}
