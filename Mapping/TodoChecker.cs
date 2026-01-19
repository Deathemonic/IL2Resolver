using dnlib.DotNet;
using IL2Resolver.Utils;

namespace IL2Resolver.Mapping;

public static class TodoChecker
{
    public static (bool RequiresTodo, string? Reason) Check(TypeSig? typeSig)
    {
        switch (typeSig)
        {
            case null:
                return (false, null);
            case PtrSig ptrSig:
                return Check(ptrSig.Next);
            case ByRefSig byRefSig:
                return Check(byRefSig.Next);
        }

        if (typeSig.IsSZArray && typeSig is ArraySigBase arraySig)
            return Check(arraySig.Next);

        switch (typeSig)
        {
            case GenericInstSig genericSig:
            {
                var elementTypeName = genericSig.GenericType?.TypeName ?? "";

                if (elementTypeName == "Dictionary`2")
                    return (true, "Dictionary<K,V> types require manual implementation");

                if (elementTypeName.StartsWith("IEnumerable") ||
                    elementTypeName.StartsWith("ICollection") ||
                    elementTypeName.StartsWith("IList"))
                    return (true, "Collection interface types require manual implementation");

                if (elementTypeName is "NativeArray`1" or "NativeSlice`1" or "ReadOnlySpan`1" or "Span`1")
                    return (true, $"{elementTypeName} not yet supported");

                foreach (var arg in genericSig.GenericArguments)
                {
                    var (argRequiresTodo, argReason) = Check(arg);
                    if (argRequiresTodo)
                        return (true, argReason);
                }

                return (false, null);
            }
            case GenericVar or GenericMVar:
                return (false, null);
        }

        if (IsPrimitiveOrSystemType(typeSig) || TypeMappings.UnityMath.ContainsKey(typeSig.FullName))
            return (false, null);

        var typeDef = TypeNameUtils.ResolveTypeDef(typeSig);
        if (typeDef is null)
            return (true, $"Cannot resolve type {typeSig.TypeName}");

        if (typeDef.IsInterface)
            return (false, null);

        return !IsAccessible(typeDef) ? (true, $"{typeSig.TypeName} is not accessible") : (false, null);
    }

    private static bool IsPrimitiveOrSystemType(TypeSig typeSig)
    {
        var fullName = typeSig.FullName;
        return fullName.StartsWith("System.") || TypeMappings.Primitives.ContainsKey(fullName);
    }

    private static bool IsAccessible(TypeDef typeDef)
    {
        if (typeDef.IsNested)
            return typeDef.IsNestedPublic || typeDef.IsNestedFamily || typeDef.IsNestedAssembly;

        return typeDef.IsPublic;
    }
}