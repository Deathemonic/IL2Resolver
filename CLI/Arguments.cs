namespace IL2Resolver.CLI;

public static class Arguments
{
    public static void Run(
        string[]? dll = null,
        string output = "./output",
        string? @namespace = null,
        string[]? type = null,
        bool depends = false,
        bool verbose = false,
        bool suppressWarnings = false)
    {
        Parser.Execute(dll, output, @namespace, type, depends, verbose, suppressWarnings);
    }
}
