using dnlib.DotNet;

namespace IL2Resolver.Utils;

public static class TypeNameUtils
{
    public static TypeDef? ResolveTypeDef(TypeSig? typeSig)
    {
        if (typeSig is null)
            return null;

        var typeDef = typeSig.TryGetTypeDef();
        if (typeDef is not null)
            return typeDef;

        var typeRef = typeSig.TryGetTypeRef();
        return typeRef?.ResolveTypeDef();
    }

    public static string GetCleanName(string name)
    {
        var backtickIndex = name.IndexOf('`');
        return backtickIndex > 0 ? name[..backtickIndex] : name;
    }

    public static string GetFullName(string ns, string name) =>
        string.IsNullOrEmpty(ns) ? name : $"{ns}.{name}";

    public static string ExtractBaseName(string rustType)
    {
        if (string.IsNullOrEmpty(rustType))
            return "";

        var name = rustType.Trim();

        if (name.StartsWith("Option<") && name.EndsWith(">"))
            name = name[7..^1].Trim();

        if (name.StartsWith("*mut "))
            name = name[5..].Trim();
        else if (name.StartsWith("*const "))
            name = name[7..].Trim();

        if (name.StartsWith("&mut "))
            name = name[5..].Trim();
        else if (name.StartsWith("&"))
            name = name[1..].Trim();

        var templateIndex = name.IndexOf('<');
        if (templateIndex > 0)
            name = name[..templateIndex];

        return name;
    }

    public static string UnwrapOption(string rustType)
    {
        if (rustType.StartsWith("Option<") && rustType.EndsWith(">"))
            return rustType[7..^1];
        return rustType;
    }

    public static string? GetNestedTypeParent(string qualifiedName)
    {
        var colonIndex = qualifiedName.IndexOf("::", StringComparison.Ordinal);
        return colonIndex >= 0 ? qualifiedName[..colonIndex] : null;
    }

    public static string GetNestedTypeName(string qualifiedName)
    {
        var colonIndex = qualifiedName.LastIndexOf("::", StringComparison.Ordinal);
        return colonIndex >= 0 ? qualifiedName[(colonIndex + 2)..] : qualifiedName;
    }

    public static string StripModulePrefix(string rustType, string currentModuleName)
    {
        var prefix = $"{currentModuleName}::";
        if (rustType.StartsWith(prefix))
            return rustType[prefix.Length..];

        if (rustType.StartsWith("Option<") && rustType.EndsWith(">"))
        {
            var inner = rustType[7..^1];
            if (inner.StartsWith(prefix))
                return $"Option<{inner[prefix.Length..]}>";
        }

        return rustType;
    }

    public static string SanitizeIdentifier(string name) => name
        .Replace('<', '_')
        .Replace('>', '_')
        .Replace('`', '_')
        .Replace('.', '_')
        .Replace('-', '_')
        .Replace('/', '_')
        .Replace('+', '_');
}
