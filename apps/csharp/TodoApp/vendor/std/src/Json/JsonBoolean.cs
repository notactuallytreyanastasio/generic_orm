namespace TemperLang.Std.Json
{
    public class JsonBoolean: IJsonSyntaxTree
    {
        readonly bool content__444;
        public void Produce(IJsonProducer p__446)
        {
            p__446.BooleanValue(this.content__444);
        }
        public JsonBoolean(bool content__449)
        {
            this.content__444 = content__449;
        }
        public bool Content
        {
            get
            {
                return this.content__444;
            }
        }
    }
}
