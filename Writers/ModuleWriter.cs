using System.Collections.Frozen;
using System.Text;
using CaseConverter;
using IL2Resolver.Context;
using IL2Resolver.Schema;

namespace IL2Resolver.Writers;

public static class ModuleWriter
{
    public static string GenerateClass(Il2CppClass cls, Il2CppSchema schema, FrozenSet<string> validTypeNames, IReadOnlySet<string> valueTypes, IReadOnlySet<string> enumTypes)
    {
        var sb = new StringBuilder();

        var currentModuleName = cls.Name.ToSnakeCase();

        var schemaTypes = new HashSet<string>();
        foreach (var c in schema.Classes)
            schemaTypes.Add(c.Name);
        foreach (var e in schema.Enums.Where(e => !e.IsNested))
            schemaTypes.Add(e.Name);

        var nestedEnumNames = cls.NestedEnums.Select(e => e.Name).ToHashSet();
        foreach (var name in nestedEnumNames)
            schemaTypes.Add(name);

        var externalTypeNames = cls.ExternalTypes.Keys.ToHashSet();

        var writerContext = new WriterContext(
            currentModuleName,
            schemaTypes.ToFrozenSet(),
            externalTypeNames.ToFrozenSet(),
            nestedEnumNames.ToFrozenSet(),
            valueTypes,
            enumTypes
        );

        sb.AppendLine("#![allow(non_camel_case_types)]");
        sb.AppendLine("#![allow(dead_code)]");
        sb.AppendLine();

        ImportWriter.Write(sb, cls, schema, writerContext);
        sb.AppendLine();

        if (cls.IsValueType)
            StructWriter.WriteValueType(sb, cls, schema, writerContext);
        else
            StructWriter.WriteReferenceType(sb, cls, schema);

        foreach (var nestedEnum in cls.NestedEnums)
        {
            sb.AppendLine();
            EnumWriter.WriteNested(sb, nestedEnum);
        }

        if (cls.Properties.Count <= 0 && cls.Methods.Count <= 0) return sb.ToString();
        sb.AppendLine();
        ImplWriter.Write(sb, cls, writerContext);

        return sb.ToString();
    }
}