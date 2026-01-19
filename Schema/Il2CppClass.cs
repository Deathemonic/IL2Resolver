namespace IL2Resolver.Schema;

public sealed class Il2CppClass
{
    public required string Name { get; init; }
    public required string Namespace { get; init; }
    public string? BaseTypeName { get; init; }
    public string? BaseTypeModule { get; init; }
    public List<Il2CppBaseType> InheritanceChain { get; } = [];
    public bool IsValueType { get; init; }
    public bool IsStatic { get; init; }
    public List<Il2CppField> Fields { get; } = [];
    public List<Il2CppProperty> Properties { get; } = [];
    public List<Il2CppMethod> Methods { get; } = [];
    public List<Il2CppConstructor> Constructors { get; } = [];
    public List<Il2CppEnum> NestedEnums { get; } = [];
    public List<string> GenericParameters { get; } = [];
    public HashSet<string> ReferencedTypes { get; } = [];
    public Dictionary<string, string> ExternalTypes { get; } = [];
}

public sealed class Il2CppBaseType
{
    public required string Name { get; init; }
    public required string Namespace { get; init; }
    public string? Module { get; init; }
}