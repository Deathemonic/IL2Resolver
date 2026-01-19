using dnlib.DotNet;
using IL2Resolver.Context;
using IL2Resolver.Rules;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Analyzers;

public static class AssemblyAnalyzer
{
    public static List<Il2CppSchema> Analyze(AnalysisContext context)
    {
        var resolver = new AssemblyResolver();

        foreach (var dllPath in context.DllPaths)
        {
            var dllDir = Path.GetDirectoryName(dllPath) ?? ".";
            resolver.PostSearchPaths.Add(dllDir);
        }

        var moduleContext = new ModuleContext(resolver);
        var moduleInfos = LoadModules(context.DllPaths, moduleContext);
        var allPublicTypes = moduleInfos.SelectMany(m => m.PublicTypes).ToList();

        var filtered = TypeFilter.Filter(allPublicTypes, context.NamespaceFilter, context.TypeFilter);
        var typesToGenerate = context is { IncludeDependencies: true, TypeFilter.Length: > 0 }
            ? ResolveDependencies(filtered, allPublicTypes, moduleInfos.Count)
            : filtered;

        return GenerateSchemas(moduleInfos, typesToGenerate, context);
    }

    private static List<ModuleInfo> LoadModules(string[] dllPaths, ModuleContext moduleContext)
    {
        var moduleInfos = new List<ModuleInfo>();

        foreach (var dllPath in dllPaths)
        {
            Log.Info($"Reading assembly: {dllPath}");
            var module = ModuleDefMD.Load(dllPath, moduleContext);
            var publicTypes = TypeFilter.GetPublicTypes(module);
            moduleInfos.Add(new ModuleInfo(dllPath, module, publicTypes));
        }

        return moduleInfos;
    }

    private static List<TypeDef> ResolveDependencies(List<TypeDef> filtered, List<TypeDef> allTypes, int moduleCount)
    {
        Log.Info($"Resolving dependencies across {moduleCount} modules...");
        var resolved = TypeFilter.CollectDependencies(filtered, allTypes);
        Log.Info($"Resolved {resolved.Count} types (from {filtered.Count} initial types)");
        return resolved;
    }

    private static List<Il2CppSchema> GenerateSchemas(
        List<ModuleInfo> moduleInfos,
        List<TypeDef> typesToGenerate,
        AnalysisContext context)
    {
        var typesByModule = typesToGenerate
            .GroupBy(t => t.Module.Name.String)
            .ToDictionary(g => g.Key, g => g.ToList());

        var schemas = new List<Il2CppSchema>();

        foreach (var info in moduleInfos)
        {
            if (!typesByModule.TryGetValue(info.Module.Name.String, out var typesForModule))
                continue;

            var schema = BuildSchema(info, typesForModule, context);
            if (schema.Classes.Count > 0 || schema.Enums.Count > 0)
                schemas.Add(schema);
        }

        return schemas;
    }

    private static Il2CppSchema BuildSchema(ModuleInfo info, List<TypeDef> types, AnalysisContext context)
    {
        var dllName = Path.GetFileName(info.DllPath);
        var assemblyName = Path.GetFileNameWithoutExtension(info.DllPath);

        var schema = new Il2CppSchema
        {
            AssemblyName = assemblyName,
            DllName = dllName
        };

        Log.Info($"[{dllName}] Processing {types.Count} types");

        foreach (var typeDef in types)
            AnalyzeType(typeDef, schema, context);

        Log.Info($"[{dllName}] Analyzed {schema.Classes.Count} classes and {schema.Enums.Count} enums");
        return schema;
    }

    private static void AnalyzeType(TypeDef typeDef, Il2CppSchema schema, AnalysisContext context)
    {
        try
        {
            if (typeDef.IsEnum)
                schema.Enums.Add(EnumAnalyzer.Analyze(typeDef));
            else if (!typeDef.IsInterface && !InheritsFromAttribute(typeDef))
                schema.Classes.Add(ClassAnalyzer.Analyze(typeDef));
        }
        catch (Exception ex)
        {
            if (!context.SuppressWarnings)
                Log.Warning($"Failed to analyze {typeDef.FullName}: {ex.Message}");
        }
    }

    private static bool InheritsFromAttribute(TypeDef typeDef)
    {
        var current = typeDef.BaseType;
        while (current is not null)
        {
            if (current.FullName == "System.Attribute")
                return true;

            var resolved = current.ResolveTypeDef();
            if (resolved is null)
                break;

            current = resolved.BaseType;
        }

        return false;
    }

    private readonly record struct ModuleInfo(string DllPath, ModuleDefMD Module, List<TypeDef> PublicTypes);
}