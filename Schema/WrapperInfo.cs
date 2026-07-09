namespace IL2Resolver.Schema;

public sealed class WrapperInfo
{
    public required string ICallName { get; init; }
    public required List<string> ICallCSharpParams { get; init; }
    public required List<string> ICallRustParams { get; init; }
    public required List<string> ICallParamNames { get; init; }
    public required List<ICallArgument> Arguments { get; init; }
    public bool IsOutReturn { get; init; }
    public string? OutReturnType { get; init; }
    public string? OutReturnRustType { get; init; }
}

public sealed class ICallArgument
{
    public required string Value { get; init; }
    public required bool IsDefault { get; init; }
    public required bool NeedsMutCopy { get; init; }
    public string? SourceParam { get; init; }
    /// <summary>
    /// When set, indicates that the argument needs to be converted to this target type using .into()
    /// This happens when a derived type (e.g., Cubemap) is passed to a parameter expecting a base type (e.g., Texture)
    /// </summary>
    public bool NeedsIntoConversion { get; init; }
}
