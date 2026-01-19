using System.Text;
using CaseConverter;
using IL2Resolver.Schema;

namespace IL2Resolver.Writers;

public static class ModFileWriter
{
    public static string Generate(Il2CppSchema schema)
    {
        var sb = new StringBuilder();

        sb.AppendLine("#![allow(non_camel_case_types)]");
        sb.AppendLine("#![allow(dead_code)]");
        sb.AppendLine();

        var moduleNames = new HashSet<string>();
        var classModules = new List<(string ModName, string TypeName)>();
        var enumModules = new List<(string ModName, string TypeName)>();

        foreach (var cls in schema.Classes.OrderBy(c => c.Name))
        {
            var modName = cls.Name.ToSnakeCase();
            if (moduleNames.Add(modName))
                classModules.Add((modName, cls.Name));
        }

        foreach (var enumDef in schema.Enums.Where(e => !e.IsNested).OrderBy(e => e.Name))
        {
            var modName = enumDef.Name.ToSnakeCase();
            if (moduleNames.Add(modName))
                enumModules.Add((modName, enumDef.Name));
        }

        foreach (var (modName, _) in classModules.Concat(enumModules).OrderBy(m => m.ModName))
            sb.AppendLine($"pub mod {modName};");

        sb.AppendLine();

        foreach (var (modName, typeName) in classModules.OrderBy(m => m.TypeName))
            sb.AppendLine($"pub use {modName}::{typeName};");

        foreach (var (modName, typeName) in enumModules.OrderBy(m => m.TypeName))
            sb.AppendLine($"pub use {modName}::{typeName};");

        return sb.ToString();
    }
}
