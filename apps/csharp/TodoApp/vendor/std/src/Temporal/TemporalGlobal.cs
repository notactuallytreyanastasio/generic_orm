using S = System;
using G = System.Collections.Generic;
using T0 = System.Text;
using T1 = TemperLang.Std.Temporal;
using C = TemperLang.Core;
using J = TemperLang.Std.Json;
namespace TemperLang.Std.Temporal
{
    public static class TemporalGlobal
    {
        internal static void encodeToJson__90(S::DateTime this__20, J::IJsonProducer p__91)
        {
            string t___313 = this__20.ToString("yyyy-MM-dd");
            p__91.StringValue(t___313);
        }
        internal static S::DateTime decodeFromJson__93(J::IJsonSyntaxTree t__94, J::IInterchangeContext ic__95)
        {
            J::JsonString t___190;
            t___190 = (J::JsonString) t__94;
            return T1::TemporalSupport.FromIsoString(t___190.Content);
        }
        internal static J::IJsonAdapter<S::DateTime> jsonAdapter__124()
        {
            return new DateJsonAdapter();
        }
        internal static G::IReadOnlyList<int> daysInMonth__34;
        internal static bool isLeapYear__32(int year__41)
        {
            bool return__21;
            int t___263;
            if (C::Core.ModSafe(year__41, 4) == 0) if (C::Core.ModSafe(year__41, 100) != 0)
            {
                return__21 = true;
            }
            else
            {
                t___263 = C::Core.ModSafe(year__41, 400);
                return__21 = t___263 == 0;
            }
            else
            {
                return__21 = false;
            }
            return return__21;
        }
        internal static void padTo__33(int minWidth__43, int num__44, T0::StringBuilder sb__45)
        {
            int t___346;
            int t___348;
            bool t___257;
            string decimal__47 = S::Convert.ToString(num__44, 10);
            int decimalIndex__48 = 0;
            int decimalEnd__49 = decimal__47.Length;
            if (decimalIndex__48 < decimalEnd__49)
            {
                t___346 = C::StringUtil.Get(decimal__47, decimalIndex__48);
                t___257 = t___346 == 45;
            }
            else
            {
                t___257 = false;
            }
            if (t___257)
            {
                sb__45.Append("-");
                t___348 = C::StringUtil.Next(decimal__47, decimalIndex__48);
                decimalIndex__48 = t___348;
            }
            int t___349 = C::StringUtil.CountBetween(decimal__47, decimalIndex__48, decimalEnd__49);
            int nNeeded__50 = minWidth__43 - t___349;
            while (nNeeded__50 > 0)
            {
                sb__45.Append("0");
                nNeeded__50 = nNeeded__50 - 1;
            }
            C::StringUtil.AppendBetween(sb__45, decimal__47, decimalIndex__48, decimalEnd__49);
        }
        internal static G::IReadOnlyList<int> dayOfWeekLookupTableLeapy__35;
        internal static G::IReadOnlyList<int> dayOfWeekLookupTableNotLeapy__36;
        static TemporalGlobal()
        {
            daysInMonth__34 = C::Listed.CreateReadOnlyList<int>(0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31);
            dayOfWeekLookupTableLeapy__35 = C::Listed.CreateReadOnlyList<int>(0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6);
            dayOfWeekLookupTableNotLeapy__36 = C::Listed.CreateReadOnlyList<int>(0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5);
        }
    }
}
