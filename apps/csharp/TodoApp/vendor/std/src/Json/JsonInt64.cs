using S = System;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonInt64: IJsonNumeric
    {
        readonly long content__482;
        public void Produce(IJsonProducer p__484)
        {
            p__484.Int64_Value(this.content__482);
        }
        public string AsJsonNumericToken()
        {
            return S::Convert.ToString(this.content__482);
        }
        public int AsInt32()
        {
            return C::Core.ToInt(this.content__482);
        }
        public long AsInt64()
        {
            return this.content__482;
        }
        public double AsFloat64()
        {
            return C::Float64.ToFloat64(this.content__482);
        }
        public JsonInt64(long content__495)
        {
            this.content__482 = content__495;
        }
        public long Content
        {
            get
            {
                return this.content__482;
            }
        }
    }
}
