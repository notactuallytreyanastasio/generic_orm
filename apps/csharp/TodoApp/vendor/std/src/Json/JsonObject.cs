using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonObject: IJsonSyntaxTree
    {
        readonly G::IReadOnlyDictionary<string, G::IReadOnlyList<IJsonSyntaxTree>> properties__420;
        public IJsonSyntaxTree ? PropertyValueOrNull(string propertyKey__422)
        {
            IJsonSyntaxTree ? return__209;
            G::IReadOnlyList<IJsonSyntaxTree> treeList__424 = C::Mapped.GetOrDefault(this.properties__420, propertyKey__422, C::Listed.CreateReadOnlyList<IJsonSyntaxTree>());
            int lastIndex__425 = treeList__424.Count - 1;
            if (lastIndex__425 >= 0)
            {
                return__209 = treeList__424[lastIndex__425];
            }
            else
            {
                return__209 = null;
            }
            return return__209;
        }
        public IJsonSyntaxTree PropertyValueOrBubble(string propertyKey__427)
        {
            IJsonSyntaxTree return__210;
            IJsonSyntaxTree ? t___2518 = this.PropertyValueOrNull(propertyKey__427);
            if (t___2518 == null) throw new S::Exception();
            else
            {
                return__210 = t___2518!;
            }
            return return__210;
        }
        public void Produce(IJsonProducer p__430)
        {
            p__430.StartObject();
            void fn__2513(string k__432, G::IReadOnlyList<IJsonSyntaxTree> vs__433)
            {
                void fn__2510(IJsonSyntaxTree v__434)
                {
                    p__430.ObjectKey(k__432);
                    v__434.Produce(p__430);
                }
                C::Listed.ForEach(vs__433, (S::Action<IJsonSyntaxTree>) fn__2510);
            }
            C::Mapped.ForEach(this.properties__420, (S::Action<string, G::IReadOnlyList<IJsonSyntaxTree>>) fn__2513);
            p__430.EndObject();
        }
        public JsonObject(G::IReadOnlyDictionary<string, G::IReadOnlyList<IJsonSyntaxTree>> properties__436)
        {
            this.properties__420 = properties__436;
        }
        public G::IReadOnlyDictionary<string, G::IReadOnlyList<IJsonSyntaxTree>> Properties
        {
            get
            {
                return this.properties__420;
            }
        }
    }
}
