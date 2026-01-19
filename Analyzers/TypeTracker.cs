using dnlib.DotNet;
using IL2Resolver.Mapping;

namespace IL2Resolver.Analyzers;

public static class TypeTracker
{
    public static void TrackReferencedType(TypeSig? typeSig, HashSet<string> referencedTypes)
    {
        if (typeSig is null) return;

        switch (typeSig)
        {
            case GenericInstSig genericSig:
                if (genericSig.GenericType is ITypeDefOrRef typeRef)
                    TrackReferencedType(typeRef.ToTypeSig(), referencedTypes);
                foreach (var arg in genericSig.GenericArguments)
                    TrackReferencedType(arg, referencedTypes);
                break;

            case ArraySigBase arraySig:
                TrackReferencedType(arraySig.Next, referencedTypes);
                break;

            case ByRefSig byRefSig:
                TrackReferencedType(byRefSig.Next, referencedTypes);
                break;

            case PtrSig ptrSig:
                TrackReferencedType(ptrSig.Next, referencedTypes);
                break;

            case GenericVar:
            case GenericMVar:
                break;

            default:
                if (IsPrimitiveOrSystemType(typeSig))
                    return;

                var fullName = typeSig.FullName;
                if (!string.IsNullOrEmpty(fullName))
                    referencedTypes.Add(fullName);
                break;
        }
    }

    public static void TrackExternalType(TypeSig? typeSig, string currentAssemblyName,
        Dictionary<string, string> externalTypes)
    {
        if (typeSig is null)
            return;

        switch (typeSig)
        {
            case GenericInstSig genericSig:
                if (genericSig.GenericType is ITypeDefOrRef extTypeRef)
                    TrackExternalType(extTypeRef.ToTypeSig(), currentAssemblyName, externalTypes);
                foreach (var arg in genericSig.GenericArguments)
                    TrackExternalType(arg, currentAssemblyName, externalTypes);
                break;

            case ArraySigBase arraySig:
                TrackExternalType(arraySig.Next, currentAssemblyName, externalTypes);
                break;

            case ByRefSig byRefSig:
                TrackExternalType(byRefSig.Next, currentAssemblyName, externalTypes);
                break;

            case PtrSig ptrSig:
                TrackExternalType(ptrSig.Next, currentAssemblyName, externalTypes);
                break;

            case GenericVar:
            case GenericMVar:
                break;

            default:
                if (IsPrimitiveOrSystemType(typeSig))
                    return;

                var module = GetTypeModule(typeSig, currentAssemblyName);
                if (module is not null)
                {
                    var typeName = GetCleanTypeNameFromSig(typeSig);
                    if (!string.IsNullOrEmpty(typeName))
                        externalTypes.TryAdd(typeName, module);
                }

                break;
        }
    }

    public static string? GetTypeModule(ITypeDefOrRef? typeRef, string currentAssemblyName)
    {
        if (typeRef is null)
            return null;

        if (IsPrimitiveOrSystemType(typeRef))
            return null;

        var scope = typeRef.Scope;
        if (scope is null)
            return null;

        var scopeName = GetAssemblyName(scope);

        if (typeRef is TypeSpec typeSpec && typeSpec.TypeSig is GenericInstSig genericSig)
        {
            scope = genericSig.GenericType?.Scope;
            scopeName = scope is not null ? GetAssemblyName(scope) : "";
        }

        if (string.IsNullOrEmpty(scopeName))
            return null;

        return string.Equals(scopeName, currentAssemblyName, StringComparison.OrdinalIgnoreCase)
            ? null
            : $"{scopeName}.dll";
    }

    private static string GetAssemblyName(IScope scope)
    {
        var scopeName = scope switch
        {
            AssemblyRef asmRef => asmRef.Name.String,
            ModuleDef modDef => modDef.Assembly?.Name.String ?? modDef.Name.String,
            ModuleRef modRef => modRef.Name.String,
            _ => scope.ScopeName
        };

        if (scopeName.EndsWith(".dll", StringComparison.OrdinalIgnoreCase))
            scopeName = scopeName[..^4];

        var commaIndex = scopeName.IndexOf(',');
        if (commaIndex > 0)
            scopeName = scopeName[..commaIndex];

        return scopeName;
    }

    private static string? GetTypeModule(TypeSig? typeSig, string currentAssemblyName)
    {
        if (typeSig is null)
            return null;

        var typeRef = typeSig.TryGetTypeRef() ?? (ITypeDefOrRef?)typeSig.TryGetTypeDef();
        return GetTypeModule(typeRef, currentAssemblyName);
    }

    private static bool IsPrimitiveOrSystemType(TypeSig typeSig)
    {
        var fullName = typeSig.FullName;

        if (TypeMappings.Primitives.ContainsKey(fullName))
            return true;

        return fullName is "System.String" or "System.Object" or "System.Void";
    }

    private static bool IsPrimitiveOrSystemType(ITypeDefOrRef typeRef)
    {
        var fullName = typeRef.FullName;

        if (TypeMappings.Primitives.ContainsKey(fullName))
            return true;

        return fullName is "System.String" or "System.Object" or "System.Void";
    }

    private static string GetCleanTypeNameFromSig(TypeSig typeSig)
    {
        var name = typeSig.TypeName;
        var backtickIndex = name.IndexOf('`');
        return backtickIndex > 0 ? name[..backtickIndex] : name;
    }
}