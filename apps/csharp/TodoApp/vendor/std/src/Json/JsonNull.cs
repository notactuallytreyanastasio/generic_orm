namespace TemperLang.Std.Json
{
    public class JsonNull: IJsonSyntaxTree
    {
        public void Produce(IJsonProducer p__451)
        {
            p__451.NullValue();
        }
        public JsonNull()
        {
        }
    }
}
