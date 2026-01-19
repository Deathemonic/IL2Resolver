using System.Collections.Frozen;
using IL2Resolver.Mapping;

namespace IL2Resolver.Rules;

public static class CopyableChecker
{
    public static bool IsCopyable(string rustType, FrozenSet<string>? validSchemaTypes = null)
    {
        if (TypeCategories.Copyable.Contains(rustType))
            return true;

        if (rustType.StartsWith("*mut ") || rustType.StartsWith("*const "))
            return true;

        if (rustType.StartsWith("Option<"))
            return true;

        if (rustType.StartsWith("Array<") || rustType.StartsWith("List<") || rustType.StartsWith("Dictionary<"))
            return true;

        if (rustType.StartsWith("ValueTuple"))
            return true;

        if (validSchemaTypes is not null && validSchemaTypes.Contains(rustType))
            return true;

        return false;
    }
}
