using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonArray: IJsonSyntaxTree
    {
        readonly G::IReadOnlyList<IJsonSyntaxTree> elements__437;
        public void Produce(IJsonProducer p__439)
        {
            p__439.StartArray();
            void fn__2503(IJsonSyntaxTree v__441)
            {
                v__441.Produce(p__439);
            }
            C::Listed.ForEach(this.elements__437, (S::Action<IJsonSyntaxTree>) fn__2503);
            p__439.EndArray();
        }
        public JsonArray(G::IReadOnlyList<IJsonSyntaxTree> elements__443)
        {
            this.elements__437 = elements__443;
        }
        public G::IReadOnlyList<IJsonSyntaxTree> Elements
        {
            get
            {
                return this.elements__437;
            }
        }
    }
}
