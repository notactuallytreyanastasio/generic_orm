using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonFloat64: IJsonNumeric
    {
        readonly double content__496;
        public void Produce(IJsonProducer p__498)
        {
            p__498.Float64_Value(this.content__496);
        }
        public string AsJsonNumericToken()
        {
            return C::Float64.Format(this.content__496);
        }
        public int AsInt32()
        {
            return C::Float64.ToInt(this.content__496);
        }
        public long AsInt64()
        {
            return C::Float64.ToInt64(this.content__496);
        }
        public double AsFloat64()
        {
            return this.content__496;
        }
        public JsonFloat64(double content__509)
        {
            this.content__496 = content__509;
        }
        public double Content
        {
            get
            {
                return this.content__496;
            }
        }
    }
}
