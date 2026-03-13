using S = System;
using J = TemperLang.Std.Json;
using T = TemperLang.Std.Temporal;
namespace TemperLang.Std.Temporal
{
    class DateJsonAdapter: J::IJsonAdapter<S::DateTime>
    {
        public void EncodeToJson(S::DateTime x__116, J::IJsonProducer p__117)
        {
            T::TemporalGlobal.encodeToJson__90(x__116, p__117);
        }
        public S::DateTime DecodeFromJson(J::IJsonSyntaxTree t__118, J::IInterchangeContext ic__119)
        {
            return T::TemporalGlobal.decodeFromJson__93(t__118, ic__119);
        }
        public DateJsonAdapter()
        {
        }
    }
}
