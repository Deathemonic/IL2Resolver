namespace IL2Resolver.Schema;

public sealed class Il2CppField
{
    public required string Name { get; init; }
    public required string Type { get; init; }
    public bool IsStatic { get; init; }
    public bool IsConst { get; init; }
    public string? DefaultValue { get; init; }
}
