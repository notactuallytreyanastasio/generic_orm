using S = System;
using G = System.Collections.Generic;
using T = System.Text;
using C = TemperLang.Core;
using J = TemperLang.Std.Json;
namespace TemperLang.Std.Json
{
    public class JsonTextProducer: IJsonProducer
    {
        readonly IInterchangeContext interchangeContext__524;
        readonly T::StringBuilder buffer__525;
        readonly G::IList<int> stack__526;
        bool wellFormed__527;
        public JsonTextProducer(IInterchangeContext ? interchangeContext = null)
        {
            IInterchangeContext interchangeContext__529;
            if (interchangeContext == null)
            {
                interchangeContext__529 = NullInterchangeContext.Instance;
            }
            else
            {
                interchangeContext__529 = interchangeContext!;
            }
            this.interchangeContext__524 = interchangeContext__529;
            T::StringBuilder t___2470 = new T::StringBuilder();
            this.buffer__525 = t___2470;
            G::IList<int> t___2471 = new G::List<int>();
            this.stack__526 = t___2471;
            C::Listed.Add(this.stack__526, 5);
            this.wellFormed__527 = true;
        }
        int State()
        {
            int t___2468 = this.stack__526.Count;
            return C::Listed.GetOr(this.stack__526, t___2468 - 1, -1);
        }
        void BeforeValue()
        {
            int t___2461;
            int t___2464;
            int t___2466;
            bool t___1728;
            int currentState__535 = this.State();
            if (currentState__535 == 3)
            {
                t___2461 = this.stack__526.Count;
                this.stack__526[t___2461 - 1] = 4;
            }
            else if (currentState__535 == 4) this.buffer__525.Append(",");
            else if (currentState__535 == 1)
            {
                t___2464 = this.stack__526.Count;
                this.stack__526[t___2464 - 1] = 2;
            }
            else if (currentState__535 == 5)
            {
                t___2466 = this.stack__526.Count;
                this.stack__526[t___2466 - 1] = 6;
            }
            else
            {
                if (currentState__535 == 6)
                {
                    t___1728 = true;
                }
                else
                {
                    t___1728 = currentState__535 == 2;
                }
                if (t___1728) this.wellFormed__527 = false;
            }
        }
        public void StartObject()
        {
            this.BeforeValue();
            this.buffer__525.Append("{");
            C::Listed.Add(this.stack__526, 0);
        }
        public void EndObject()
        {
            bool t___1716;
            this.buffer__525.Append("}");
            int currentState__540 = this.State();
            if (0 == currentState__540)
            {
                t___1716 = true;
            }
            else
            {
                t___1716 = 2 == currentState__540;
            }
            if (t___1716) C::Listed.RemoveLast(this.stack__526);
            else this.wellFormed__527 = false;
        }
        public void ObjectKey(string key__542)
        {
            int t___2452;
            int currentState__544 = this.State();
            if (!(currentState__544 == 0)) if (currentState__544 == 2) this.buffer__525.Append(",");
            else this.wellFormed__527 = false;
            J::JsonGlobal.encodeJsonString__351(key__542, this.buffer__525);
            this.buffer__525.Append(":");
            if (currentState__544 >= 0)
            {
                t___2452 = this.stack__526.Count;
                this.stack__526[t___2452 - 1] = 1;
            }
        }
        public void StartArray()
        {
            this.BeforeValue();
            this.buffer__525.Append("[");
            C::Listed.Add(this.stack__526, 3);
        }
        public void EndArray()
        {
            bool t___1704;
            this.buffer__525.Append("]");
            int currentState__549 = this.State();
            if (3 == currentState__549)
            {
                t___1704 = true;
            }
            else
            {
                t___1704 = 4 == currentState__549;
            }
            if (t___1704) C::Listed.RemoveLast(this.stack__526);
            else this.wellFormed__527 = false;
        }
        public void NullValue()
        {
            this.BeforeValue();
            this.buffer__525.Append("null");
        }
        public void BooleanValue(bool x__553)
        {
            string t___1700;
            this.BeforeValue();
            if (x__553)
            {
                t___1700 = "true";
            }
            else
            {
                t___1700 = "false";
            }
            this.buffer__525.Append(t___1700);
        }
        public void Int32_Value(int x__556)
        {
            this.BeforeValue();
            string t___2436 = S::Convert.ToString(x__556);
            this.buffer__525.Append(t___2436);
        }
        public void Int64_Value(long x__559)
        {
            this.BeforeValue();
            string t___2433 = S::Convert.ToString(x__559);
            this.buffer__525.Append(t___2433);
        }
        public void Float64_Value(double x__562)
        {
            this.BeforeValue();
            string t___2430 = C::Float64.Format(x__562);
            this.buffer__525.Append(t___2430);
        }
        public void NumericTokenValue(string x__565)
        {
            this.BeforeValue();
            this.buffer__525.Append(x__565);
        }
        public void StringValue(string x__568)
        {
            this.BeforeValue();
            J::JsonGlobal.encodeJsonString__351(x__568, this.buffer__525);
        }
        public string ToJsonString()
        {
            string return__272;
            int t___2423;
            bool t___1689;
            bool t___1690;
            if (this.wellFormed__527)
            {
                if (this.stack__526.Count == 1)
                {
                    t___2423 = this.State();
                    t___1689 = t___2423 == 6;
                }
                else
                {
                    t___1689 = false;
                }
                t___1690 = t___1689;
            }
            else
            {
                t___1690 = false;
            }
            if (t___1690)
            {
                return__272 = this.buffer__525.ToString();
            }
            else throw new S::Exception();
            return return__272;
        }
        public IInterchangeContext InterchangeContext
        {
            get
            {
                return this.interchangeContext__524;
            }
        }
        public IJsonParseErrorReceiver ? ParseErrorReceiver
        {
            get
            {
                return IJsonProducer.GetParseErrorReceiverDefault(this);
            }
        }
    }
}
