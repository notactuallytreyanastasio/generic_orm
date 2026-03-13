using R = Orm;
using System.Runtime.CompilerServices;

namespace Orm.Support
{
    /// <summary>
    /// Default to initializing all modules for a library when no top is given.
    /// </summary>
    public static class OrmGlobal
    {
        static OrmGlobal()
        {
            RuntimeHelpers.RunClassConstructor(typeof(R::Src.SrcGlobal).TypeHandle);
        }
    }
}
