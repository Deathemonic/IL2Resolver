using dnlib.DotNet;

namespace IL2Resolver.Rules;

public static class MethodFilter
{
    public static bool ShouldSkip(MethodDef method)
    {
        if (method.IsGetter || method.IsSetter)
            return true;

        if (method.Name.String.EndsWith("_Injected"))
            return true;

        if (IsOperator(method))
            return true;

        return false;
    }

    public static bool IsOperator(MethodDef method) =>
        method.IsSpecialName && method.Name.String.StartsWith("op_");

    public static bool IsPublicOrICall(MethodDef method, Func<MethodDef, bool> isICallCheck) =>
        method.IsPublic || ((method.IsAssembly || method.IsPrivate) && isICallCheck(method));
}
