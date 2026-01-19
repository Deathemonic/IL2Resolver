namespace IL2Resolver.Schema;

public sealed class Il2CppSchema
{
    public required string AssemblyName { get; init; }
    public required string DllName { get; init; }
    public List<Il2CppClass> Classes { get; } = [];
    public List<Il2CppEnum> Enums { get; } = [];
}
