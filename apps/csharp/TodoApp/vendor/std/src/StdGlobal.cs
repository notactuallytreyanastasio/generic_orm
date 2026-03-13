using R = TemperLang.Std;
using System.Runtime.CompilerServices;

namespace TemperLang.Std.Support
{
    /// <summary>
    /// Default to initializing all modules for a library when no top is given.
    /// </summary>
    public static class StdGlobal
    {
        static StdGlobal()
        {
            RuntimeHelpers.RunClassConstructor(typeof(R::Json.JsonGlobal).TypeHandle);
            RuntimeHelpers.RunClassConstructor(typeof(R::Temporal.TemporalGlobal).TypeHandle);
            RuntimeHelpers.RunClassConstructor(typeof(R::Testing.TestingGlobal).TypeHandle);
            RuntimeHelpers.RunClassConstructor(typeof(R::Net.NetGlobal).TypeHandle);
            RuntimeHelpers.RunClassConstructor(typeof(R::Regex.RegexGlobal).TypeHandle);
        }
    }
}
