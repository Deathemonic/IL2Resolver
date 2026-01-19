using System.Collections.Frozen;
using IL2Resolver.Analyzers;
using IL2Resolver.Context;
using IL2Resolver.Utils;
using IL2Resolver.Writers;

namespace IL2Resolver.CLI;

public static class Parser
{
    public static void Execute(
        string[]? dll,
        string output,
        string? @namespace,
        string[]? type,
        bool depends,
        bool verbose,
        bool suppressWarnings)
    {
        if (verbose) Log.EnableDebugLogging();
        if (suppressWarnings) Log.SetSuppressWarnings(true);

        if (dll is null or { Length: 0 })
        {
            Log.Error("No DLL specified. Use --dll <path>");
            Log.Shutdown();
            Environment.Exit(1);
            return;
        }

        foreach (var dllPath in dll)
        {
            if (File.Exists(dllPath)) continue;

            Log.Error($"DLL not found: {dllPath}");
            Log.Shutdown();
            Environment.Exit(1);
        }

        try
        {
            var analysisContext = new AnalysisContext(
                dll,
                output,
                @namespace,
                type,
                depends,
                verbose,
                suppressWarnings
            );

            var schemas = AssemblyAnalyzer.Analyze(analysisContext);

            if (!Directory.Exists(output))
                Directory.CreateDirectory(output);

            foreach (var schema in schemas)
            {
                var validTypeNames = schema.Classes
                    .Select(c => c.Name)
                    .Concat(schema.Enums.Where(e => !e.IsNested).Select(e => e.Name))
                    .ToFrozenSet();

                var generationContext = new GenerationContext(
                    schema,
                    validTypeNames,
                    output
                );

                FileGenerator.Generate(generationContext);
            }

            Log.Info("Done.");
        }
        catch (Exception ex)
        {
            Log.Error($"Failed to process assemblies: {ex.Message}");
            Log.Shutdown();
            Environment.Exit(1);
        }
    }
}