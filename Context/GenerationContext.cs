using System.Collections.Frozen;
using IL2Resolver.Schema;

namespace IL2Resolver.Context;

public readonly record struct GenerationContext(
    Il2CppSchema Schema,
    FrozenSet<string> ValidTypeNames,
    string OutputPath,
    IReadOnlySet<string> ValueTypes,
    IReadOnlySet<string> EnumTypes
);