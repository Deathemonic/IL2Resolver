using System.Text.Json.Serialization;

namespace IL2Resolver.Schema;

public sealed class RuntimeDump
{
    [JsonPropertyName("total_assemblies")]
    public int TotalAssemblies { get; init; }

    [JsonPropertyName("assemblies")]
    public List<RuntimeAssembly> Assemblies { get; init; } = [];
}

public sealed class RuntimeAssembly
{
    [JsonPropertyName("name")]
    public string Name { get; init; } = "";

    [JsonPropertyName("class_count")]
    public int ClassCount { get; init; }

    [JsonPropertyName("classes")]
    public List<RuntimeClass> Classes { get; init; } = [];
}

public sealed class RuntimeClass
{
    [JsonPropertyName("name")]
    public string Name { get; init; } = "";

    [JsonPropertyName("namespace")]
    public string Namespace { get; init; } = "";

    [JsonPropertyName("full_name")]
    public string FullName { get; init; } = "";

    [JsonPropertyName("is_enum")]
    public bool IsEnum { get; init; }

    [JsonPropertyName("is_value_type")]
    public bool IsValueType { get; init; }

    [JsonPropertyName("methods")]
    public List<RuntimeMethod> Methods { get; init; } = [];

    [JsonPropertyName("fields")]
    public List<RuntimeField> Fields { get; init; } = [];
}

public sealed class RuntimeMethod
{
    [JsonPropertyName("name")]
    public string Name { get; init; } = "";

    [JsonPropertyName("return_type")]
    public string ReturnType { get; init; } = "";

    [JsonPropertyName("is_static")]
    public bool IsStatic { get; init; }

    [JsonPropertyName("is_icall")]
    public bool IsICall { get; init; }

    [JsonPropertyName("parameters")]
    public List<RuntimeParameter> Parameters { get; init; } = [];
}

public sealed class RuntimeParameter
{
    [JsonPropertyName("name")]
    public string Name { get; init; } = "";

    [JsonPropertyName("type")]
    public string Type { get; init; } = "";

    [JsonPropertyName("is_by_ref")]
    public bool IsByRef { get; init; }
}

public sealed class RuntimeField
{
    [JsonPropertyName("name")]
    public string Name { get; init; } = "";

    [JsonPropertyName("type")]
    public string Type { get; init; } = "";

    [JsonPropertyName("is_static")]
    public bool IsStatic { get; init; }
}
