using S = System;
namespace TemperLang.Std.Json
{
    public class JsonInt32: IJsonNumeric
    {
        readonly int content__468;
        public void Produce(IJsonProducer p__470)
        {
            p__470.Int32_Value(this.content__468);
        }
        public string AsJsonNumericToken()
        {
            return S::Convert.ToString(this.content__468);
        }
        public int AsInt32()
        {
            return this.content__468;
        }
        public long AsInt64()
        {
            return (long) this.content__468;
        }
        public double AsFloat64()
        {
            return (double) this.content__468;
        }
        public JsonInt32(int content__481)
        {
            this.content__468 = content__481;
        }
        public int Content
        {
            get
            {
                return this.content__468;
            }
        }
    }
}
