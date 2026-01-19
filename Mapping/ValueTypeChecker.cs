using dnlib.DotNet;

namespace IL2Resolver.Mapping;

public static class ValueTypeChecker
{
    public static bool IsValueType(TypeSig? typeSig)
    {
        if (typeSig is null)
            return false;

        if (TypeMappings.Primitives.ContainsKey(typeSig.FullName))
            return true;

        if (typeSig is GenericVar or GenericMVar)
            return false;

        var typeDef = typeSig.TryGetTypeDef();
        return typeDef is not null && IsValueType(typeDef);
    }

    public static bool IsValueType(TypeDef? typeDef)
    {
        if (typeDef is null)
            return false;

        return typeDef.IsValueType || typeDef.IsEnum;
    }
}
