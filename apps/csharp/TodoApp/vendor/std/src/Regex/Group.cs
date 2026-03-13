namespace TemperLang.Std.Regex
{
    public class Group
    {
        readonly string name__244;
        readonly string value__245;
        readonly int begin__246;
        readonly int end__247;
        public Group(string name__249, string value__250, int begin__251, int end__252)
        {
            this.name__244 = name__249;
            this.value__245 = value__250;
            this.begin__246 = begin__251;
            this.end__247 = end__252;
        }
        public string Name
        {
            get
            {
                return this.name__244;
            }
        }
        public string Value
        {
            get
            {
                return this.value__245;
            }
        }
        public int Begin
        {
            get
            {
                return this.begin__246;
            }
        }
        public int End
        {
            get
            {
                return this.end__247;
            }
        }
    }
}
