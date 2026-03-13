using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    class ListJsonAdapter<T__180>: IJsonAdapter<G::IReadOnlyList<T__180>>
    {
        readonly IJsonAdapter<T__180> adapterForT__822;
        public void EncodeToJson(G::IReadOnlyList<T__180> x__824, IJsonProducer p__825)
        {
            p__825.StartArray();
            void fn__2174(T__180 el__827)
            {
                this.adapterForT__822.EncodeToJson(el__827, p__825);
            }
            C::Listed.ForEach(x__824, (S::Action<T__180>) fn__2174);
            p__825.EndArray();
        }
        public G::IReadOnlyList<T__180> DecodeFromJson(IJsonSyntaxTree t__829, IInterchangeContext ic__830)
        {
            T__180 t___1369;
            G::IList<T__180> b__832 = new G::List<T__180>();
            JsonArray t___1364;
            t___1364 = (JsonArray) t__829;
            G::IReadOnlyList<IJsonSyntaxTree> elements__833 = t___1364.Elements;
            int n__834 = elements__833.Count;
            int i__835 = 0;
            while (i__835 < n__834)
            {
                IJsonSyntaxTree el__836 = elements__833[i__835];
                i__835 = i__835 + 1;
                t___1369 = this.adapterForT__822.DecodeFromJson(el__836, ic__830);
                C::Listed.Add(b__832, t___1369);
            }
            return C::Listed.ToReadOnlyList(b__832);
        }
        public ListJsonAdapter(IJsonAdapter<T__180> adapterForT__838)
        {
            this.adapterForT__822 = adapterForT__838;
        }
    }
}
