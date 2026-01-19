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
        var schemas = new List<Il2CppSchema>();
        var resolver = new AssemblyResolver();

        foreach (var dllPath in context.DllPaths)
        {
            var dllDir = Path.GetDirectoryName(dllPath) ?? ".";
            resolver.PostSearchPaths.Add(dllDir);
        }

        var moduleContext = new ModuleContext(resolver);

        foreach (var dllPath in context.DllPaths)
        {
            Log.Info($"Reading assembly: {dllPath}");
            var module = ModuleDefMD.Load(dllPath, moduleContext);
            var schema = AnalyzeModule(module, dllPath, context);

            if (schema.Classes.Count > 0 || schema.Enums.Count > 0)
                schemas.Add(schema);
        }

        return schemas;
    }

    private static Il2CppSchema AnalyzeModule(ModuleDefMD module, string dllPath, AnalysisContext context)
    {
        var dllName = Path.GetFileName(dllPath);
        var assemblyName = Path.GetFileNameWithoutExtension(dllPath);

        var schema = new Il2CppSchema
        {
            AssemblyName = assemblyName,
            DllName = dllName
        };

        var types = TypeFilter.GetPublicTypes(module);
        var filtered = TypeFilter.Filter(types, context.NamespaceFilter, context.TypeFilter);

        if (context.IncludeDependencies && context.TypeFilter is { Length: > 0 })
            filtered = TypeFilter.CollectDependencies(filtered, types);

        Log.Info($"[{dllName}] Found {filtered.Count} types to process");

        foreach (var typeDef in filtered)
        {
            try
            {
                if (typeDef.IsEnum)
                {
                    var enumDef = EnumAnalyzer.Analyze(typeDef);
                    schema.Enums.Add(enumDef);
                }
                else if (!typeDef.IsInterface && !InheritsFromAttribute(typeDef))
                {
                    var classDef = ClassAnalyzer.Analyze(typeDef);
                    schema.Classes.Add(classDef);
                }
            }
            catch (Exception ex)
            {
                if (!context.SuppressWarnings)
                    Log.Warning($"Failed to analyze {typeDef.FullName}: {ex.Message}");
            }
        }

        Log.Info($"[{dllName}] Analyzed {schema.Classes.Count} classes and {schema.Enums.Count} enums");
        return schema;
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
}
