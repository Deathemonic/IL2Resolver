using System.Text.Json;
using System.Text.Json.Serialization;
using IL2Resolver.Schema;

namespace IL2Resolver.Context;

[JsonSerializable(typeof(RuntimeDump))]
internal partial class RuntimeDumpJsonContext : JsonSerializerContext;

public sealed class ValidationContext
{
    private readonly Dictionary<string, RuntimeClass> _classes = new();
    private readonly Dictionary<string, HashSet<string>> _methods = new();
    private readonly HashSet<string> _valueTypes = new();
    private readonly HashSet<string> _enums = new();

    public bool IsEnabled { get; }

    private ValidationContext(bool enabled) => IsEnabled = enabled;

    public static ValidationContext Disabled() => new(false);

    public static ValidationContext Load(string jsonPath)
    {
        var json = File.ReadAllText(jsonPath);
        var dump = JsonSerializer.Deserialize(json, RuntimeDumpJsonContext.Default.RuntimeDump);

        if (dump is null)
            throw new InvalidOperationException($"Failed to parse runtime dump: {jsonPath}");

        var context = new ValidationContext(true);

        foreach (var assembly in dump.Assemblies)
        {
            foreach (var cls in assembly.Classes)
            {
                var fullName = string.IsNullOrEmpty(cls.Namespace)
                    ? cls.Name
                    : $"{cls.Namespace}.{cls.Name}";

                context._classes[fullName] = cls;

                if (cls.IsValueType && !cls.IsEnum)
                    context._valueTypes.Add(fullName);

                if (cls.IsEnum)
                    context._enums.Add(fullName);

                var methodSignatures = new HashSet<string>();
                foreach (var method in cls.Methods.Where(m => m.IsICall))
                {
                    var signature = BuildMethodSignature(method);
                    methodSignatures.Add(signature);
                }

                context._methods[fullName] = methodSignatures;
            }
        }

        return context;
    }

    public bool IsValueType(string fullTypeName)
    {
        if (!IsEnabled)
            return false;

        return _valueTypes.Contains(fullTypeName);
    }

    public bool IsEnum(string fullTypeName)
    {
        if (!IsEnabled)
            return false;

        return _enums.Contains(fullTypeName);
    }

    public IReadOnlySet<string> GetValueTypes() => _valueTypes;

    public IReadOnlySet<string> GetEnums() => _enums;

    public bool ClassExists(string fullName) =>
        !IsEnabled || _classes.ContainsKey(fullName);

    public bool MethodExists(string classFullName, string methodName, IEnumerable<string> paramTypes)
    {
        if (!IsEnabled)
            return true;

        if (!_methods.TryGetValue(classFullName, out var methods))
            return false;

        var signature = BuildSignature(methodName, paramTypes);
        return methods.Contains(signature);
    }

    public bool MethodExists(string classFullName, string methodName)
    {
        if (!IsEnabled)
            return true;

        if (!_methods.TryGetValue(classFullName, out var methods))
            return false;

        return methods.Any(m => m.StartsWith($"{methodName}("));
    }

    public bool MethodExistsWithSignature(string classFullName, string methodName, IReadOnlyList<string> paramTypeNames)
    {
        if (!IsEnabled)
            return true;

        if (!_methods.TryGetValue(classFullName, out var methods))
            return false;

        var signature = BuildSignature(methodName, paramTypeNames);
        if (methods.Contains(signature))
            return true;

        var signatureWithRef = BuildSignatureWithRefs(methodName, paramTypeNames);
        return methods.Any(m => m == signature || m == signatureWithRef || MatchesSignatureLoosely(m, methodName, paramTypeNames));
    }

    private static bool MatchesSignatureLoosely(string storedSignature, string methodName, IReadOnlyList<string> paramTypeNames)
    {
        if (!storedSignature.StartsWith($"{methodName}("))
            return false;

        var storedParamsStart = storedSignature.IndexOf('(') + 1;
        var storedParamsEnd = storedSignature.LastIndexOf(')');
        if (storedParamsEnd <= storedParamsStart)
            return paramTypeNames.Count == 0;

        var storedParams = storedSignature[storedParamsStart..storedParamsEnd]
            .Split(',')
            .Select(p => p.Trim().TrimEnd('&').Split('.').Last())
            .ToList();

        if (storedParams.Count != paramTypeNames.Count)
            return false;

        for (var i = 0; i < storedParams.Count; i++)
        {
            var stored = storedParams[i];
            var expected = paramTypeNames[i].TrimEnd('&').Split('.').Last();
            if (stored != expected)
                return false;
        }

        return true;
    }

    private static string BuildMethodSignature(RuntimeMethod method)
    {
        var paramTypes = method.Parameters.Select(p => p.IsByRef ? $"{p.Type}&" : p.Type);
        return BuildSignature(method.Name, paramTypes);
    }

    private static string BuildSignature(string name, IEnumerable<string> paramTypes) =>
        $"{name}({string.Join(",", paramTypes)})";

    private static string BuildSignatureWithRefs(string name, IEnumerable<string> paramTypes) =>
        $"{name}({string.Join(",", paramTypes.Select(t => $"{t}&"))})";
}
