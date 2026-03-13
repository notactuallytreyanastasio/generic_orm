namespace TemperLang.Std.Json
{
    public interface IJsonNumeric: IJsonSyntaxTree
    {
        string AsJsonNumericToken();
        int AsInt32();
        long AsInt64();
        double AsFloat64();
    }
}
