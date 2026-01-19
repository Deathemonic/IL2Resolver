using dnlib.DotNet;

namespace IL2Resolver.Mapping;

public static class GenericTypeMapper
{
    public static string Map(GenericInstSig genericSig, bool isParameter)
    {
        var elementType = genericSig.GenericType;
        var fullName = elementType?.FullName ?? "";

        var typeDef = elementType?.ToTypeDefOrRef().ResolveTypeDef();
        if (typeDef is { IsInterface: true })
            return "*mut c_void";

        var baseName = GetBaseName(elementType?.TypeName);

        if (baseName.StartsWith("IEnumerable") || baseName.StartsWith("IEnumerator") ||
            baseName.StartsWith("ICollection") || baseName.StartsWith("IList") ||
            baseName.StartsWith("IDictionary"))
            return "*mut c_void";

        switch (baseName)
        {
            case "List" when genericSig.GenericArguments.Count == 1:
            {
                var innerType = RustTypeMapper.Map(genericSig.GenericArguments[0]);
                if (innerType.StartsWith("Option<") && innerType.EndsWith(">"))
                    innerType = innerType[7..^1];
                return $"List<{innerType}>";
            }
            case "Dictionary" when genericSig.GenericArguments.Count == 2:
            {
                var keyType = RustTypeMapper.Map(genericSig.GenericArguments[0]);
                var valType = RustTypeMapper.Map(genericSig.GenericArguments[1]);
                if (keyType.StartsWith("Option<") && keyType.EndsWith(">"))
                    keyType = keyType[7..^1];
                if (valType.StartsWith("Option<") && valType.EndsWith(">"))
                    valType = valType[7..^1];
                return $"Dictionary<{keyType}, {valType}>";
            }
            case "ValueTuple":
            {
                var argCount = genericSig.GenericArguments.Count;
                if (argCount < 2 || argCount > 4) return "*mut c_void";
                var args = genericSig.GenericArguments
                    .Select(a => RustTypeMapper.Map(a))
                    .ToArray();
                return $"ValueTuple{argCount}<{string.Join(", ", args)}>";
            }
            case "Nullable" when genericSig.GenericArguments.Count == 1:
            {
                var innerType = RustTypeMapper.Map(genericSig.GenericArguments[0]);
                return $"Nullable<{innerType}>";
            }
        }

        if (fullName.StartsWith("System.Action") || fullName.StartsWith("System.Func"))
            return "*mut c_void";

        return "*mut c_void";
    }

    private static string GetBaseName(string? typeName)
    {
        if (string.IsNullOrEmpty(typeName))
            return "";

        var backtickIndex = typeName.IndexOf('`');
        return backtickIndex > 0 ? typeName[..backtickIndex] : typeName;
    }
}