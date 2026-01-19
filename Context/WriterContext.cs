using System.Collections.Frozen;

namespace IL2Resolver.Context;

public readonly record struct WriterContext(
    string CurrentModuleName,
    FrozenSet<string> ValidSchemaTypes,
    FrozenSet<string> ExternalTypeNames,
    FrozenSet<string> NestedEnumNames
);
