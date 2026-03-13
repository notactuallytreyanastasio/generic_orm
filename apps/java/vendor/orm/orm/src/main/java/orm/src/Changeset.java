package orm.src;
import java.util.List;
import java.util.Map;
public interface Changeset {
    TableDef getTableDef();
    Map<String, String> getChanges();
    List<ChangesetError> getErrors();
    boolean isValid();
    Changeset cast(List<SafeIdentifier> allowedFields__314);
    Changeset validateRequired(List<SafeIdentifier> fields__317);
    Changeset validateLength(SafeIdentifier field__320, int min__321, int max__322);
    Changeset validateInt(SafeIdentifier field__325);
    Changeset validateInt64(SafeIdentifier field__328);
    Changeset validateFloat(SafeIdentifier field__331);
    Changeset validateBool(SafeIdentifier field__334);
    SqlFragment toInsertSql();
    SqlFragment toUpdateSql(int id__339);
}
