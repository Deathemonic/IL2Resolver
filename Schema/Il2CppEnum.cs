namespace IL2Resolver.Schema;

public sealed class Il2CppEnum
{
    public required string Name { get; init; }
    public required string Namespace { get; init; }
    public required string UnderlyingType { get; init; }
    public List<Il2CppEnumValue> Values { get; } = [];
    public bool IsNested { get; set; }
    public string? ParentTypeName { get; set; }
}

public sealed class Il2CppEnumValue
{
    public required string Name { get; init; }
    public required long Value { get; init; }
}
