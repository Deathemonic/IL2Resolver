namespace IL2Resolver.Schema;

public sealed class Il2CppProperty
{
    public required string Name { get; init; }
    public required string IlName { get; init; }
    public required string Type { get; init; }
    public required string CSharpType { get; init; }
    public bool HasGetter { get; init; }
    public bool HasSetter { get; init; }
    public bool IsStatic { get; init; }
    public bool GetterIsICall { get; init; }
    public bool SetterIsICall { get; init; }
    public string? GetterInjectedICallName { get; init; }
    public List<string>? GetterInjectedParams { get; init; }
    public string? SetterInjectedICallName { get; init; }
    public List<string>? SetterInjectedParams { get; init; }
}
