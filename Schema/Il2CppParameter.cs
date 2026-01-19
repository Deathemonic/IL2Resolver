namespace IL2Resolver.Schema;

public sealed class Il2CppParameter
{
    public required string Name { get; init; }
    public required string Type { get; init; }
    public required string CSharpType { get; init; }
    public bool IsOut { get; init; }
    public bool IsRef { get; init; }
    public string? DefaultValue { get; init; }
}
