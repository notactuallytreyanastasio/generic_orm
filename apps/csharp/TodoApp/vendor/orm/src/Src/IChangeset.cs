using G = System.Collections.Generic;
namespace Orm.Src
{
    public interface IChangeset
    {
        TableDef TableDef
        {
            get;
        }
        G::IReadOnlyDictionary<string, string> Changes
        {
            get;
        }
        G::IReadOnlyList<ChangesetError> Errors
        {
            get;
        }
        bool IsValid
        {
            get;
        }
        IChangeset Cast(G::IReadOnlyList<ISafeIdentifier> allowedFields__314);
        IChangeset ValidateRequired(G::IReadOnlyList<ISafeIdentifier> fields__317);
        IChangeset ValidateLength(ISafeIdentifier field__320, int min__321, int max__322);
        IChangeset ValidateInt(ISafeIdentifier field__325);
        IChangeset ValidateInt64(ISafeIdentifier field__328);
        IChangeset ValidateFloat(ISafeIdentifier field__331);
        IChangeset ValidateBool(ISafeIdentifier field__334);
        SqlFragment ToInsertSql();
        SqlFragment ToUpdateSql(int id__339);
    }
}
