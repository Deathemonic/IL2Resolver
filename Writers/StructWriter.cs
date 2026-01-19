using System.Text;
using CaseConverter;
using IL2Resolver.Context;
using IL2Resolver.Rules;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Writers;

public static class StructWriter
{
    public static void WriteValueType(StringBuilder sb, Il2CppClass cls, Il2CppSchema schema, WriterContext context)
    {
        var assembly = schema.DllName.Replace(".dll", "");
        var ns = cls.Namespace;
        var className = cls.Name;

        var hasCopyableFields = cls.Fields
            .Where(f => !f.IsStatic)
            .All(f => CopyableChecker.IsCopyable(f.Type, context.ValidSchemaTypes));

        sb.AppendLine("#[repr(C)]");
        sb.AppendLine(hasCopyableFields
            ? "#[derive(Clone, Copy, Default, PartialEq, UnityClass)]"
            : "#[derive(Clone, Default, PartialEq, UnityClass)]");

        sb.AppendLine(
            $"#[unity(assembly = \"{assembly}\", class = \"{className}\", namespace = \"{ns}\", value_type)]");
        sb.AppendLine($"pub struct {cls.Name} {{");

        foreach (var field in cls.Fields.Where(f => !f.IsStatic))
        {
            var fieldName = RustKeywords.Escape(field.Name.ToSnakeCase());
            var fieldType = TypeNameUtils.StripModulePrefix(field.Type, context.CurrentModuleName);
            sb.AppendLine($"    pub {fieldName}: {fieldType},");
        }

        sb.AppendLine("}");
    }

    public static void WriteReferenceType(StringBuilder sb, Il2CppClass cls, Il2CppSchema schema)
    {
        var assembly = schema.DllName.Replace(".dll", "");
        var ns = cls.Namespace;
        var className = cls.Name;

        var isComponentHost = IsComponentHost(cls);

        var attrParts = new List<string>
        {
            $"assembly = \"{assembly}\"",
            $"class = \"{className}\"",
            $"namespace = \"{ns}\""
        };

        if (isComponentHost)
            attrParts.Add("component_host");

        if (cls.InheritanceChain.Count > 0)
        {
            var baseNames = cls.InheritanceChain.Select(b => b.Name).ToList();
            attrParts.Add($"inherit = \"{string.Join(",", baseNames)}\"");
        }

        sb.AppendLine("#[repr(transparent)]");
        sb.AppendLine("#[derive(UnityClass)]");
        sb.AppendLine($"#[unity({string.Join(", ", attrParts)})]");
        sb.AppendLine($"pub struct {cls.Name}(pub *mut c_void);");
    }

    private static bool IsComponentHost(Il2CppClass cls) =>
        cls.Methods.Any(m =>
            m is { Name: "GetComponent", Parameters.Count: 1 } &&
            m.Parameters[0].Type.Contains("SystemType"));
}