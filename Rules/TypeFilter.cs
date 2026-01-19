using dnlib.DotNet;
using IL2Resolver.Mapping;
using ZLinq;

namespace IL2Resolver.Rules;

public static class TypeFilter
{
    public static List<TypeDef> GetPublicTypes(ModuleDef module)
    {
        var types = new List<TypeDef>();

        foreach (var t in module.GetTypes())
        {
            if (!IsValidType(t))
                continue;

            if (t.IsPublic && !t.IsNested)
                types.Add(t);

            if (t.IsNestedPublic)
                types.Add(t);
        }

        return [.. types.DistinctBy(t => t.FullName)];
    }

    private static bool IsValidType(TypeDef t)
    {
        var name = t.Name.String;
        var ns = t.Namespace?.String ?? "";

        if (t.IsInterface)
            return false;

        if (name.StartsWith('<') || name.Contains("__"))
            return false;

        if (name.Contains("e__FixedBuffer") || name.Contains("StaticArrayInit"))
            return false;

        if (name.EndsWith("Extensions") && t is { IsAbstract: true, IsSealed: true })
            return false;

        if (string.IsNullOrEmpty(ns) && !t.IsNested)
            return false;

        if (name.StartsWith("Baselib_") || name.StartsWith("XR"))
            return false;

        var fullName = $"{ns}.{name}";
        if (TypeMappings.UnityMath.ContainsKey(fullName))
            return false;

        return !t.IsValueType || t.IsEnum || !t.Fields.All(f => f.IsStatic) ||
               t.Methods.Any(m => m.IsPublic && m is { IsConstructor: false, IsGetter: false, IsSetter: false }) ||
               t.Properties.Count != 0;
    }

    public static List<TypeDef> Filter(
        List<TypeDef> types,
        string? namespaceFilter,
        string[]? typeNames)
    {
        var result = types.AsEnumerable();

        if (!string.IsNullOrEmpty(namespaceFilter))
            result = result.Where(t => t.Namespace?.String == namespaceFilter);

        if (typeNames is { Length: > 0 })
            result = result.Where(t => typeNames.Contains(t.Name.String));

        result = DeduplicateByName(result);

        return [.. result];
    }

    private static IEnumerable<TypeDef> DeduplicateByName(IEnumerable<TypeDef> types) =>
        types
            .GroupBy(t => t.Name.String)
            .Select(g => g.OrderBy(t => GetNamespaceDepth(t.Namespace?.String)).First());

    private static int GetNamespaceDepth(string? ns)
    {
        if (string.IsNullOrEmpty(ns))
            return 0;
        return ns.Count(c => c == '.') + 1;
    }

    private static bool IsSubTypeOf(TypeDef typeToCheck, string ancestorTypeName)
    {
        var currentBaseRef = typeToCheck.BaseType;

        while (currentBaseRef is not null)
        {
            if (currentBaseRef.Name.String == ancestorTypeName)
                return true;

            var currentBaseDef = currentBaseRef.ResolveTypeDef();
            if (currentBaseDef is null)
                break;

            currentBaseRef = currentBaseDef.BaseType;
        }

        return false;
    }

    public static List<TypeDef> CollectDependencies(List<TypeDef> requestedTypes, List<TypeDef> allTypes)
    {
        var typesByFullName = allTypes.GroupBy(t => t.FullName).ToDictionary(g => g.Key, g => g.First());
        var typesByName = allTypes.GroupBy(t => GetCleanTypeName(t.Name.String)).ToDictionary(g => g.Key, g => g.First());
        var result = new HashSet<TypeDef>(requestedTypes);
        var toProcess = new Queue<TypeDef>(requestedTypes);
        var processed = new HashSet<string>();

        while (toProcess.Count > 0)
        {
            var typeDef = toProcess.Dequeue();
            if (!processed.Add(typeDef.FullName))
                continue;

            foreach (var refTypeName in GetReferencedTypeNames(typeDef))
            {
                var refType = ResolveType(refTypeName, typesByFullName, typesByName);
                if (refType is not null && result.Add(refType))
                    toProcess.Enqueue(refType);
            }
        }

        return [.. result];
    }

    private static TypeDef? ResolveType(
        string typeName,
        Dictionary<string, TypeDef> byFullName,
        Dictionary<string, TypeDef> byName)
    {
        if (byFullName.TryGetValue(typeName, out var result))
            return result;

        var cleanName = GetCleanTypeName(typeName);
        if (byFullName.TryGetValue(cleanName, out result))
            return result;

        var simpleName = cleanName.Contains('.') ? cleanName[(cleanName.LastIndexOf('.') + 1)..] : cleanName;
        return byName.GetValueOrDefault(simpleName);
    }

    private static string GetCleanTypeName(string name)
    {
        var idx = name.IndexOf('`');
        return idx > 0 ? name[..idx] : name;
    }

    private static IEnumerable<string> GetReferencedTypeNames(TypeDef typeDef)
    {
        if (typeDef.BaseType is not null && !IsSystemBaseType(typeDef.BaseType.FullName))
            yield return typeDef.BaseType.FullName;

        foreach (var field in typeDef.Fields.Where(f => f.IsPublic || (typeDef.IsValueType && !f.IsStatic)))
            foreach (var name in GetTypeNames(field.FieldType))
                yield return name;

        foreach (var prop in typeDef.Properties.Where(p =>
                     (p.GetMethod?.IsPublic ?? false) || (p.SetMethod?.IsPublic ?? false)))
            foreach (var name in GetTypeNames(prop.PropertySig?.RetType))
                yield return name;

        foreach (var method in typeDef.Methods.Where(m => !m.IsGetter && !m.IsSetter))
        {
            var isICall = IsICallMethod(method);
            if (!method.IsPublic && !isICall)
                continue;

            foreach (var name in GetTypeNames(method.ReturnType))
                yield return name;

            foreach (var param in method.Parameters.Where(p => p.IsNormalMethodParameter))
                foreach (var name in GetTypeNames(param.Type))
                    yield return name;
        }
    }

    private static bool IsICallMethod(MethodDef method)
    {
        if (!method.HasBody &&
            ((method.ImplAttributes & MethodImplAttributes.InternalCall) != 0 ||
             (method.Attributes & MethodAttributes.PinvokeImpl) != 0))
            return true;

        foreach (var arg in method.CustomAttributes
                     .AsValueEnumerable()
                     .Where(attr => attr.TypeFullName == "System.Runtime.CompilerServices.MethodImplAttribute"
                                    && attr.ConstructorArguments.Count > 0)
                     .Select(attr => attr.ConstructorArguments[0]))
            if (arg.Value is 4096 or short and 4096)
                return true;

        return false;
    }

    private static IEnumerable<string> GetTypeNames(TypeSig? typeSig)
    {
        if (typeSig is null)
            yield break;

        switch (typeSig)
        {
            case GenericInstSig genericSig:
                if (genericSig.GenericType is not null)
                    yield return genericSig.GenericType.FullName;
                foreach (var arg in genericSig.GenericArguments)
                    foreach (var name in GetTypeNames(arg))
                        yield return name;
                break;

            case ArraySigBase arraySig:
                foreach (var name in GetTypeNames(arraySig.Next))
                    yield return name;
                break;

            case ByRefSig byRefSig:
                foreach (var name in GetTypeNames(byRefSig.Next))
                    yield return name;
                break;

            case PtrSig ptrSig:
                foreach (var name in GetTypeNames(ptrSig.Next))
                    yield return name;
                break;

            case GenericVar:
            case GenericMVar:
                break;

            default:
                if (!typeSig.FullName.StartsWith("System."))
                    yield return typeSig.FullName;
                break;
        }
    }

    private static bool IsSystemBaseType(string fullName) =>
        fullName is "System.Object" or "System.ValueType" or "System.Enum" or "System.Attribute"
            or "System.MulticastDelegate" or "System.Delegate";
}