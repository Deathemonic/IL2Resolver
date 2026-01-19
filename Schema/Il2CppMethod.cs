namespace IL2Resolver.Schema;

public sealed class Il2CppMethod
{
    public required string Name { get; init; }
    public required string IlName { get; init; }
    public required string ReturnType { get; init; }
    public bool IsStatic { get; init; }
    public bool IsICall { get; init; }
    public string? WrappedICallName { get; init; }
    public List<string>? WrappedICallArgs { get; init; }
    public List<string>? InjectedICallParams { get; init; }
    public string? StaticDelegateField { get; init; }
    public string? StaticDelegateMethod { get; init; }
    public List<Il2CppParameter>? StaticDelegateParams { get; init; }
    public bool RequiresTodo { get; init; }
    public string? TodoReason { get; init; }
    public List<Il2CppParameter> Parameters { get; } = [];
    public List<string> GenericParameters { get; } = [];
}
