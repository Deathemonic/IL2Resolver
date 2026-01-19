namespace IL2Resolver.Context;

public readonly record struct AnalysisContext(
    string[] DllPaths,
    string OutputPath,
    string? NamespaceFilter,
    string[]? TypeFilter,
    bool IncludeDependencies,
    bool Verbose,
    bool SuppressWarnings
);
