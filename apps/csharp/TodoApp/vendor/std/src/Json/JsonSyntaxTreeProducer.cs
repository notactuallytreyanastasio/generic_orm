using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public class JsonSyntaxTreeProducer: IJsonProducer, IJsonParseErrorReceiver
    {
        readonly G::IList<G::IList<IJsonSyntaxTree>> stack__590;
        string ? error__591;
        public IInterchangeContext InterchangeContext
        {
            get
            {
                return NullInterchangeContext.Instance;
            }
        }
        public JsonSyntaxTreeProducer()
        {
            G::IList<G::IList<IJsonSyntaxTree>> t___2419 = new G::List<G::IList<IJsonSyntaxTree>>();
            this.stack__590 = t___2419;
            G::IList<IJsonSyntaxTree> t___2420 = new G::List<IJsonSyntaxTree>();
            C::Listed.Add(this.stack__590, t___2420);
            this.error__591 = null;
        }
        void StoreValue(IJsonSyntaxTree v__597)
        {
            int t___2416;
            if (!(C::Listed.AsReadOnly(this.stack__590).Count == 0))
            {
                t___2416 = this.stack__590.Count;
                C::Listed.Add(this.stack__590[t___2416 - 1], v__597);
            }
        }
        public void StartObject()
        {
            G::IList<IJsonSyntaxTree> t___2413 = new G::List<IJsonSyntaxTree>();
            C::Listed.Add(this.stack__590, t___2413);
        }
        public void EndObject()
        {
            G::IDictionary<string, G::IList<IJsonSyntaxTree>> ? t___2402;
            JsonObject t___2411;
            JsonString t___1656;
            JsonString t___1658;
            G::IDictionary<string, G::IList<IJsonSyntaxTree>> t___1664;
            G::IDictionary<string, G::IList<IJsonSyntaxTree>> t___1666;
            G::IReadOnlyList<IJsonSyntaxTree> t___1668;
            G::IReadOnlyList<IJsonSyntaxTree> t___1669;
            G::IList<IJsonSyntaxTree> t___1671;
            G::IList<IJsonSyntaxTree> t___1672;
            {
                {
                    if (C::Listed.AsReadOnly(this.stack__590).Count == 0) goto fn__602;
                    G::IList<IJsonSyntaxTree> ls__603 = C::Listed.RemoveLast(this.stack__590);
                    G::IDictionary<string, G::IReadOnlyList<IJsonSyntaxTree>> m__604 = new C::OrderedDictionary<string, G::IReadOnlyList<IJsonSyntaxTree>>();
                    G::IDictionary<string, G::IList<IJsonSyntaxTree>> ? multis__605 = null;
                    int i__606 = 0;
                    int n__607 = ls__603.Count & -2;
                    while (i__606 < n__607)
                    {
                        int postfixReturn___40 = i__606;
                        i__606 = i__606 + 1;
                        IJsonSyntaxTree keyTree__608 = ls__603[postfixReturn___40];
                        if (!(keyTree__608 is JsonString)) break;
                        t___1656 = (JsonString) keyTree__608;
                        t___1658 = t___1656;
                        string key__609 = t___1658.Content;
                        int postfixReturn___41 = i__606;
                        i__606 = i__606 + 1;
                        IJsonSyntaxTree value__610 = ls__603[postfixReturn___41];
                        if (C::Mapped.ContainsKey(m__604, key__609))
                        {
                            if (multis__605 == null)
                            {
                                t___2402 = new C::OrderedDictionary<string, G::IList<IJsonSyntaxTree>>();
                                multis__605 = t___2402;
                            }
                            if (multis__605 == null) throw new S::Exception();
                            else
                            {
                                t___1664 = multis__605!;
                            }
                            t___1666 = t___1664;
                            G::IDictionary<string, G::IList<IJsonSyntaxTree>> mb__611 = t___1666;
                            if (!C::Mapped.ContainsKey(mb__611, key__609))
                            {
                                t___1668 = m__604[key__609];
                                t___1669 = t___1668;
                                mb__611[key__609] = L::Enumerable.ToList(t___1669);
                            }
                            t___1671 = mb__611[key__609];
                            t___1672 = t___1671;
                            C::Listed.Add(t___1672, value__610);
                        }
                        else m__604[key__609] = C::Listed.CreateReadOnlyList<IJsonSyntaxTree>(value__610);
                    }
                    G::IDictionary<string, G::IList<IJsonSyntaxTree>> ? multis__612 = multis__605;
                    if (!(multis__612 == null))
                    {
                        void fn__2392(string k__613, G::IList<IJsonSyntaxTree> vs__614)
                        {
                            G::IReadOnlyList<IJsonSyntaxTree> t___2390 = C::Listed.ToReadOnlyList(vs__614);
                            m__604[k__613] = t___2390;
                        }
                        C::Mapped.ForEach(multis__612!, (S::Action<string, G::IList<IJsonSyntaxTree>>) fn__2392);
                    }
                    t___2411 = new JsonObject(C::Mapped.ToMap(m__604));
                    this.StoreValue(t___2411);
                }
                fn__602:
                {
                }
            }
        }
        public void ObjectKey(string key__616)
        {
            JsonString t___2388 = new JsonString(key__616);
            this.StoreValue(t___2388);
        }
        public void StartArray()
        {
            G::IList<IJsonSyntaxTree> t___2386 = new G::List<IJsonSyntaxTree>();
            C::Listed.Add(this.stack__590, t___2386);
        }
        public void EndArray()
        {
            JsonArray t___2384;
            {
                {
                    if (C::Listed.AsReadOnly(this.stack__590).Count == 0) goto fn__621;
                    G::IList<IJsonSyntaxTree> ls__622 = C::Listed.RemoveLast(this.stack__590);
                    t___2384 = new JsonArray(C::Listed.ToReadOnlyList(ls__622));
                    this.StoreValue(t___2384);
                }
                fn__621:
                {
                }
            }
        }
        public void NullValue()
        {
            JsonNull t___2379 = new JsonNull();
            this.StoreValue(t___2379);
        }
        public void BooleanValue(bool x__626)
        {
            JsonBoolean t___2377 = new JsonBoolean(x__626);
            this.StoreValue(t___2377);
        }
        public void Int32_Value(int x__629)
        {
            JsonInt32 t___2375 = new JsonInt32(x__629);
            this.StoreValue(t___2375);
        }
        public void Int64_Value(long x__632)
        {
            JsonInt64 t___2373 = new JsonInt64(x__632);
            this.StoreValue(t___2373);
        }
        public void Float64_Value(double x__635)
        {
            JsonFloat64 t___2371 = new JsonFloat64(x__635);
            this.StoreValue(t___2371);
        }
        public void NumericTokenValue(string x__638)
        {
            JsonNumericToken t___2369 = new JsonNumericToken(x__638);
            this.StoreValue(t___2369);
        }
        public void StringValue(string x__641)
        {
            JsonString t___2367 = new JsonString(x__641);
            this.StoreValue(t___2367);
        }
        public IJsonSyntaxTree ToJsonSyntaxTree()
        {
            bool t___1629;
            if (this.stack__590.Count != 1)
            {
                t___1629 = true;
            }
            else
            {
                t___1629 = !(this.error__591 == null);
            }
            if (t___1629) throw new S::Exception();
            G::IList<IJsonSyntaxTree> ls__645 = this.stack__590[0];
            if (ls__645.Count != 1) throw new S::Exception();
            return ls__645[0];
        }
        public string ? JsonError
        {
            get
            {
                return this.error__591;
            }
        }
        public IJsonParseErrorReceiver ParseErrorReceiver
        {
            get
            {
                return this;
            }
        }
        public void ExplainJsonError(string error__651)
        {
            this.error__591 = error__651;
        }
    }
}
