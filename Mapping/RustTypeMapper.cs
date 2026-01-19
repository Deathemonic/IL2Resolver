using CaseConverter;
using dnlib.DotNet;
using IL2Resolver.Utils;

namespace IL2Resolver.Mapping;

public static class RustTypeMapper
{
    public static string Map(TypeSig? typeSig, bool isParameter = false, bool isOut = false)
    {
        if (typeSig is null)
            return "()";

        var fullName = typeSig.FullName;

        if (fullName.Contains("e__FixedBuffer") || fullName.StartsWith('<'))
            return "*mut c_void";

        if (isOut)
        {
            var elementType = typeSig is ByRefSig byRef ? byRef.Next : typeSig;
            return $"&mut {Map(elementType)}";
        }

        if (typeSig is ByRefSig byRefSig)
            return $"&mut {Map(byRefSig.Next)}";

        if (typeSig.IsSZArray && typeSig is ArraySigBase arraySig)
        {
            var innerType = Map(arraySig.Next);
            if (innerType.StartsWith("Option<") && innerType.EndsWith(">"))
                innerType = innerType[7..^1];
            return $"Array<{innerType}>";
        }

        if (typeSig is GenericInstSig genericInstSig)
            return GenericTypeMapper.Map(genericInstSig, isParameter);

        if (typeSig is GenericVar or GenericMVar)
            return "*mut c_void";

        if (typeSig is PtrSig ptrSig)
            return $"*mut {Map(ptrSig.Next)}";

        if (TypeMappings.Primitives.TryGetValue(fullName, out var primitive))
            return primitive;

        if (fullName == "System.String")
            return isParameter ? "&str" : "Option<SystemString>";

        if (TypeMappings.SystemTypes.TryGetValue(fullName, out var systemType))
            return $"Option<{systemType}>";

        if (fullName.StartsWith("System.") && IsSystemInterface(typeSig))
            return "*mut c_void";

        if (TypeMappings.UnityMath.TryGetValue(fullName, out var mathType))
            return mathType;

        var typeDef = TypeNameUtils.ResolveTypeDef(typeSig);

        if (typeDef is { IsInterface: true })
            return "*mut c_void";

        if (typeDef is null)
            return "*mut c_void";

        if (!IsAccessible(typeDef))
            return "*mut c_void";

        var typeName = GetCleanTypeName(typeSig);

        if (typeDef.IsEnum || ValueTypeChecker.IsValueType(typeDef))
            return typeName;

        return $"Option<{typeName}>";
    }

    private static bool IsAccessible(TypeDef typeDef)
    {
        if (typeDef.IsNested)
            return typeDef.IsNestedPublic;

        return typeDef.IsPublic;
    }

    private static bool IsSystemInterface(TypeSig typeSig)
    {
        var name = typeSig.TypeName;
        var baseName = name.Contains('`') ? name[..name.IndexOf('`')] : name;
        return TypeMappings.SystemInterfaces.Contains(baseName);
    }

    private static string GetCleanTypeName(TypeSig typeSig)
    {
        var name = typeSig.TypeName;
        var backtickIndex = name.IndexOf('`');
        if (backtickIndex > 0)
            name = name[..backtickIndex];

        var typeDef = TypeNameUtils.ResolveTypeDef(typeSig);
        if (typeDef is { IsNested: true, DeclaringType: not null } && typeDef.IsEnum)
        {
            var parentName = typeDef.DeclaringType.Name.String;
            var parentBacktick = parentName.IndexOf('`');
            if (parentBacktick > 0)
                parentName = parentName[..parentBacktick];

            var parentModule = parentName.ToSnakeCase();
            return $"{parentModule}::{name}";
        }

        return name;
    }
}
