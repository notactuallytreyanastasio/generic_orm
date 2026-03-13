using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class OrNullJsonAdapter<T__184>: IJsonAdapter<C::Optional<T__184>>
    {
        readonly IJsonAdapter<T__184> adapterForT__841;
        public void EncodeToJson(C::Optional<T__184> x__843, IJsonProducer p__844)
        {
            if (!x__843.HasValue) p__844.NullValue();
            else
            {
                T__184 x___962 = x__843.Value;
                this.adapterForT__841.EncodeToJson(x___962, p__844);
            }
        }
        public C::Optional<T__184> DecodeFromJson(IJsonSyntaxTree t__847, IInterchangeContext ic__848)
        {
            C::Optional<T__184> return__350;
            if (t__847 is JsonNull)
            {
                return__350 = C::Optional<T__184>.None;
            }
            else
            {
                return__350 = this.adapterForT__841.DecodeFromJson(t__847, ic__848);
            }
            return return__350;
        }
        public OrNullJsonAdapter(IJsonAdapter<T__184> adapterForT__851)
        {
            this.adapterForT__841 = adapterForT__851;
        }
    }
}
