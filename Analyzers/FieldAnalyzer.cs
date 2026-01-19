using dnlib.DotNet;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;

namespace IL2Resolver.Analyzers;

public static class FieldAnalyzer
{
    public static Il2CppField Analyze(FieldDef fieldDef)
    {
        var rawName = fieldDef.Name.String;
        var fieldName = rawName;

        if (rawName.StartsWith('<') && rawName.Contains(">k__BackingField"))
        {
            var endIndex = rawName.IndexOf('>');
            if (endIndex > 1)
                fieldName = rawName[1..endIndex];
        }

        return new Il2CppField
        {
            Name = fieldName,
            Type = RustTypeMapper.Map(fieldDef.FieldType),
            IsStatic = fieldDef.IsStatic,
            IsConst = fieldDef.HasConstant,
            DefaultValue = fieldDef.HasConstant ? GetDefaultValueString(fieldDef.Constant.Value) : null
        };
    }

    private static string? GetDefaultValueString(object? value) => value switch
    {
        null => "None",
        bool b => b ? "true" : "false",
        string s => $"\"{s}\"",
        char c => $"'{c}'",
        float f => $"{f}_f32",
        double d => $"{d}_f64",
        _ => value.ToString()
    };
}
