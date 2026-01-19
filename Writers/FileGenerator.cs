using System.Collections.Frozen;
using CaseConverter;
using IL2Resolver.Context;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Writers;

public static class FileGenerator
{
    public static void Generate(GenerationContext context)
    {
        var schema = context.Schema;
        Log.Info($"Generating Rust files to: {context.OutputPath}");
        Log.Info($"Processing {schema.Classes.Count} classes and {schema.Enums.Count} enums");

        var moduleFolderName = GetModuleFolderName(schema.DllName);
        var moduleOutputPath = Path.Combine(context.OutputPath, moduleFolderName);

        GenerateEnumFiles(schema, moduleOutputPath);
        GenerateClassFiles(schema, moduleOutputPath, context.ValidTypeNames);
        GenerateModFile(schema, moduleOutputPath);

        Log.Info("Generation complete");
    }

    private static void GenerateClassFiles(Il2CppSchema schema, string outputPath, FrozenSet<string> validTypeNames)
    {
        foreach (var cls in schema.Classes)
            try
            {
                var fileName = cls.Name.ToSnakeCase();
                var filePath = Path.Combine(outputPath, fileName + ".rs");
                var content = ModuleWriter.GenerateClass(cls, schema, validTypeNames);
                WriteFile(filePath, content);
                Log.Verbose($"Generated: {cls.Name}");
            }
            catch (Exception ex)
            {
                Log.Warning($"Failed to generate files for {cls.Namespace}.{cls.Name}: {ex.Message}");
            }
    }

    private static void GenerateEnumFiles(Il2CppSchema schema, string outputPath)
    {
        foreach (var enumDef in schema.Enums.Where(e => !e.IsNested))
            try
            {
                var fileName = enumDef.Name.ToSnakeCase();
                var filePath = Path.Combine(outputPath, fileName + ".rs");
                var content = EnumWriter.GenerateModule(enumDef);
                WriteFile(filePath, content);
                Log.Verbose($"Generated enum: {enumDef.Name}");
            }
            catch (Exception ex)
            {
                Log.Warning($"Failed to generate file for enum {enumDef.Namespace}.{enumDef.Name}: {ex.Message}");
            }
    }

    private static void GenerateModFile(Il2CppSchema schema, string outputPath)
    {
        var filePath = Path.Combine(outputPath, "mod.rs");
        var content = ModFileWriter.Generate(schema);
        WriteFile(filePath, content);
        Log.Verbose("Generated mod.rs");
    }

    private static string GetModuleFolderName(string dllName)
    {
        var name = Path.GetFileNameWithoutExtension(dllName);

        if (name.StartsWith("UnityEngine."))
            name = name["UnityEngine.".Length..];

        return name.ToSnakeCase();
    }

    private static void WriteFile(string path, string content)
    {
        var directory = Path.GetDirectoryName(path);
        if (!string.IsNullOrEmpty(directory) && !Directory.Exists(directory))
            Directory.CreateDirectory(directory);

        File.WriteAllText(path, content);
    }
}