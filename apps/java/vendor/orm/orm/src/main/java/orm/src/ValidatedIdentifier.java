package orm.src;
final class ValidatedIdentifier implements SafeIdentifier {
    final String _value;
    public String getSqlValue() {
        return this._value;
    }
    public ValidatedIdentifier(String _value__625) {
        this._value = _value__625;
    }
}
