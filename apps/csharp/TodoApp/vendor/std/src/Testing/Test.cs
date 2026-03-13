using U = Microsoft.VisualStudio.TestTools.UnitTesting;
using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Testing
{
    public class Test
    {
        public void Assert(bool success__43, S::Func<string> message__44)
        {
            string t___406;
            if (!success__43)
            {
                this._passing__66 = false;
                t___406 = message__44();
                C::Listed.Add(this._messages__67, t___406);
            }
        }
        public void AssertHard(bool success__47, S::Func<string> message__48)
        {
            this.Assert(success__47, (S::Func<string>) message__48);
            if (!success__47)
            {
                this._failedOnAssert__65 = true;
                throw new U::AssertFailedException(this.MessagesCombined() ?? "");
            }
        }
        public void SoftFailToHard()
        {
            if (this.HasUnhandledFail)
            {
                this._failedOnAssert__65 = true;
                throw new U::AssertFailedException(this.MessagesCombined() ?? "");
            }
        }
        public bool Passing
        {
            get
            {
                return this._passing__66;
            }
        }
        public G::IReadOnlyList<string> Messages()
        {
            return C::Listed.ToReadOnlyList(this._messages__67);
        }
        public bool FailedOnAssert
        {
            get
            {
                return this._failedOnAssert__65;
            }
        }
        public bool HasUnhandledFail
        {
            get
            {
                bool t___264;
                if (this._failedOnAssert__65)
                {
                    t___264 = true;
                }
                else
                {
                    t___264 = this._passing__66;
                }
                return !t___264;
            }
        }
        public string ? MessagesCombined()
        {
            string ? return__35;
            if (C::Listed.AsReadOnly(this._messages__67).Count == 0)
            {
                return__35 = null;
            }
            else
            {
                string fn__399(string it__64)
                {
                    return it__64;
                }
                return__35 = C::Listed.Join(this._messages__67, ", ", (S::Func<string, string>) fn__399);
            }
            return return__35;
        }
        bool _failedOnAssert__65;
        bool _passing__66;
        readonly G::IList<string> _messages__67;
        public Test()
        {
            this._failedOnAssert__65 = false;
            this._passing__66 = true;
            G::IList<string> t___398 = new G::List<string>();
            this._messages__67 = t___398;
        }
    }
}
