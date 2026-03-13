namespace TemperLang.Std.Json
{
    public class JsonString: IJsonSyntaxTree
    {
        readonly string content__454;
        public void Produce(IJsonProducer p__456)
        {
            p__456.StringValue(this.content__454);
        }
        public JsonString(string content__459)
        {
            this.content__454 = content__459;
        }
        public string Content
        {
            get
            {
                return this.content__454;
            }
        }
    }
}
