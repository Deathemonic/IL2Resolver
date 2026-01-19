using System.Text;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Writers;

public static class EnumWriter
{
    public static void WriteNested(StringBuilder sb, Il2CppEnum enumDef)
    {
        var underlyingType = TypeMappings.GetEnumUnderlyingType(enumDef.UnderlyingType);
        var (uniqueValues, defaultIndex) = GetUniqueValues(enumDef);

        sb.AppendLine($"#[repr({underlyingType})]");
        sb.AppendLine("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]");
        sb.AppendLine($"pub enum {enumDef.Name} {{");

        for (var i = 0; i < uniqueValues.Count; i++)
        {
            var value = uniqueValues[i];
            var valueName = SanitizeVariant(value.Name);
            var defaultAttr = i == defaultIndex ? "    #[default]\n" : "";
            sb.Append(defaultAttr);
            sb.AppendLine($"    {valueName} = {value.Value},");
        }

        sb.AppendLine("}");
    }

    public static string GenerateModule(Il2CppEnum enumDef)
    {
        var sb = new StringBuilder();

        sb.AppendLine("#![allow(non_camel_case_types)]");
        sb.AppendLine("#![allow(dead_code)]");
        sb.AppendLine();

        var underlyingType = TypeMappings.GetEnumUnderlyingType(enumDef.UnderlyingType);
        var (uniqueValues, defaultIndex) = GetUniqueValues(enumDef);

        sb.AppendLine($"#[repr({underlyingType})]");
        sb.AppendLine("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]");
        sb.AppendLine($"pub enum {enumDef.Name} {{");

        for (var i = 0; i < uniqueValues.Count; i++)
        {
            var value = uniqueValues[i];
            var valueName = SanitizeVariant(value.Name);
            var defaultAttr = i == defaultIndex ? "    #[default]\n" : "";
            sb.Append(defaultAttr);
            sb.AppendLine($"    {valueName} = {value.Value},");
        }

        sb.AppendLine("}");

        return sb.ToString();
    }

    private static (List<Il2CppEnumValue> Values, int DefaultIndex) GetUniqueValues(Il2CppEnum enumDef)
    {
        var seenValues = new HashSet<long>();
        var uniqueValues = enumDef.Values.Where(value => seenValues.Add(value.Value)).ToList();

        var defaultIndex = uniqueValues.FindIndex(v => v.Value == 0);
        if (defaultIndex < 0) defaultIndex = 0;

        return (uniqueValues, defaultIndex);
    }

    private static string SanitizeVariant(string name) =>
        name == "Self" ? "Self_" : TypeNameUtils.SanitizeIdentifier(name);
}
