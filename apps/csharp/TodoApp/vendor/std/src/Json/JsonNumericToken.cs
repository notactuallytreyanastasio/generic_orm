using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonNumericToken: IJsonNumeric
    {
        readonly string content__510;
        public void Produce(IJsonProducer p__512)
        {
            p__512.NumericTokenValue(this.content__510);
        }
        public string AsJsonNumericToken()
        {
            return this.content__510;
        }
        public int AsInt32()
        {
            int return__254;
            int t___1774;
            double t___1775;
            try
            {
                t___1774 = C::Core.ToInt(this.content__510);
                return__254 = t___1774;
            }
            catch
            {
                t___1775 = C::Float64.ToFloat64(this.content__510);
                return__254 = C::Float64.ToInt(t___1775);
            }
            return return__254;
        }
        public long AsInt64()
        {
            long return__255;
            long t___1770;
            double t___1771;
            try
            {
                t___1770 = C::Core.ToInt64(this.content__510);
                return__255 = t___1770;
            }
            catch
            {
                t___1771 = C::Float64.ToFloat64(this.content__510);
                return__255 = C::Float64.ToInt64(t___1771);
            }
            return return__255;
        }
        public double AsFloat64()
        {
            return C::Float64.ToFloat64(this.content__510);
        }
        public JsonNumericToken(string content__523)
        {
            this.content__510 = content__523;
        }
        public string Content
        {
            get
            {
                return this.content__510;
            }
        }
    }
}
