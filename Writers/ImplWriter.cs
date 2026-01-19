using System.Text;
using CaseConverter;
using IL2Resolver.Context;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Writers;

public static class ImplWriter
{
    public static void Write(StringBuilder sb, Il2CppClass cls, WriterContext context)
    {
        sb.AppendLine("#[unity_impl]");
        sb.AppendLine($"impl {cls.Name} {{");

        var methodNameCounts = new Dictionary<string, int>();
        var csharpNameToRustName = new Dictionary<string, string>();

        if (cls.Constructors.Count > 0 && !cls.IsValueType)
            WriteConstructors(sb, cls, context.CurrentModuleName);

        foreach (var prop in cls.Properties)
        {
            var propName = prop.Name.ToSnakeCase();
            if (prop.HasGetter)
                GetUniqueMethodName($"get_{propName}", methodNameCounts);
            if (prop.HasSetter)
                GetUniqueMethodName($"set_{propName}", methodNameCounts);
        }

        foreach (var method in cls.Methods)
        {
            if (method.GenericParameters.Count > 0 || method.RequiresTodo || method.Name == "IsValid")
                continue;
            var baseMethodName = RustKeywords.Escape(method.Name.ToSnakeCase());
            var rustMethodName = GetUniqueMethodName(baseMethodName, methodNameCounts);
            csharpNameToRustName[method.IlName] = rustMethodName;
        }

        methodNameCounts.Clear();

        foreach (var prop in cls.Properties)
            WritePropertyMethods(sb, prop, cls, methodNameCounts, context.CurrentModuleName);

        foreach (var method in cls.Methods)
            WriteMethod(sb, method, cls, methodNameCounts, csharpNameToRustName, context.CurrentModuleName);

        sb.AppendLine("}");
    }

    private static void WriteConstructors(StringBuilder sb, Il2CppClass cls, string currentModuleName)
    {
        var sortedCtors = cls.Constructors
            .OrderBy(c => c.Parameters.Count == 0 ? 0 : 1)
            .ThenBy(c => c.Parameters.Count)
            .ToList();

        var ctorIndex = 0;
        foreach (var ctor in sortedCtors)
        {
            var methodName = ctorIndex == 0 ? "new" : $"new_{ctorIndex}";
            ctorIndex++;

            var parameters = BuildParameterList(ctor.Parameters, false, currentModuleName);

            sb.AppendLine("    #[unity_ctor]");
            sb.AppendLine($"    pub fn {methodName}({parameters}) -> Option<Self> {{}}");
            sb.AppendLine();
        }
    }

    private static void WritePropertyMethods(StringBuilder sb, Il2CppProperty prop, Il2CppClass cls, Dictionary<string, int> methodNameCounts, string currentModuleName)
    {
        var propName = prop.Name.ToSnakeCase();
        var propType = TypeNameUtils.StripModulePrefix(prop.Type, currentModuleName);
        var ilName = prop.IlName;

        var setterParamType = propType == "Option<SystemString>" ? "&str" : propType;

        if (prop.HasGetter)
        {
            var baseGetterName = $"get_{propName}";
            var getterName = GetUniqueMethodName(baseGetterName, methodNameCounts);
            var selfParam = prop.IsStatic ? "" : "&self";

            if (prop.GetterIsICall)
            {
                var icallName = BuildICallName(cls, $"get_{ilName}", []);
                sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");
                sb.AppendLine($"    pub fn {getterName}({selfParam}) -> {propType} {{}}");
            }
            else if (prop.GetterInjectedICallName is not null)
            {
                var icallName = BuildInjectedICallName(cls, prop.GetterInjectedICallName, prop.GetterInjectedParams);
                sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");
                var selfParamWithComma = prop.IsStatic ? "" : "&self, ";
                sb.AppendLine($"    pub fn {getterName}({selfParamWithComma}ret: &mut {propType}) {{}}");
            }
            else if (prop.IsStatic)
            {
                sb.AppendLine($"    #[unity_method(name = \"get_{ilName}\", static)]");
                sb.AppendLine($"    pub fn {getterName}({selfParam}) -> {propType} {{}}");
            }
            else
            {
                sb.AppendLine($"    #[unity_method(name = \"get_{ilName}\")]");
                sb.AppendLine($"    pub fn {getterName}({selfParam}) -> {propType} {{}}");
            }
            sb.AppendLine();
        }

        if (!prop.HasSetter) return;

        var baseSetterName = $"set_{propName}";
        var setterName = GetUniqueMethodName(baseSetterName, methodNameCounts);
        var setterSelfParam = prop.IsStatic ? "" : "&self, ";

        if (prop.SetterIsICall)
        {
            var icallName = BuildICallNameForProperty(cls, $"set_{ilName}", prop.CSharpType);
            sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");
            sb.AppendLine($"    pub fn {setterName}({setterSelfParam}value: {setterParamType}) {{}}");
        }
        else if (prop.SetterInjectedICallName is not null)
        {
            var icallName = BuildInjectedICallName(cls, prop.SetterInjectedICallName, prop.SetterInjectedParams);
            sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");
            sb.AppendLine($"    pub fn {setterName}({setterSelfParam}value: &mut {propType}) {{}}");
        }
        else if (prop.IsStatic)
        {
            sb.AppendLine($"    #[unity_method(name = \"set_{ilName}\", static)]");
            sb.AppendLine($"    pub fn {setterName}({setterSelfParam}value: {setterParamType}) {{}}");
        }
        else
        {
            sb.AppendLine($"    #[unity_method(name = \"set_{ilName}\")]");
            sb.AppendLine($"    pub fn {setterName}({setterSelfParam}value: {setterParamType}) {{}}");
        }
        sb.AppendLine();
    }

    private static void WriteMethod(StringBuilder sb, Il2CppMethod method, Il2CppClass cls,
        Dictionary<string, int> methodNameCounts, Dictionary<string, string> csharpNameToRustName, string currentModuleName)
    {
        if (method.GenericParameters.Count > 0 || method.RequiresTodo || method.Name == "IsValid")
            return;

        var baseMethodName = RustKeywords.Escape(method.Name.ToSnakeCase());
        var methodName = GetUniqueMethodName(baseMethodName, methodNameCounts);

        var returnType = TypeNameUtils.StripModulePrefix(method.ReturnType, currentModuleName);
        var ilName = method.IlName;

        var parameters = BuildParameterList(method.Parameters, !method.IsStatic, currentModuleName);

        if (method is { StaticDelegateMethod: not null, StaticDelegateField: not null, StaticDelegateParams: not null })
        {
            var staticMethodName = method.StaticDelegateMethod.ToSnakeCase();
            var fieldName = method.StaticDelegateField.ToSnakeCase();

            var argParts = new List<string> { $"&mut self.{fieldName}" };
            foreach (var param in method.Parameters)
            {
                var paramName = RustKeywords.Escape(param.Name.ToSnakeCase());
                argParts.Add(paramName);
            }

            var wrapperArgs = string.Join(", ", argParts);

            var mutSelfParams = BuildParameterList(method.Parameters, false, currentModuleName);
            var selfPart = string.IsNullOrEmpty(mutSelfParams) ? "&mut self" : "&mut self, ";

            var wrapperReturnClause = returnType == "()" ? "" : $" -> {returnType}";
            sb.AppendLine($"    pub fn {methodName}({selfPart}{mutSelfParams}){wrapperReturnClause} {{");
            sb.AppendLine($"        Self::{staticMethodName}({wrapperArgs})");
            sb.AppendLine("    }");
            sb.AppendLine();
            return;
        }

        if (method is { WrappedICallName: not null, WrappedICallArgs: not null })
        {
            if (method.WrappedICallArgs is ["__injected__"])
            {
                var icallName = BuildInjectedICallName(cls, method.WrappedICallName, method.InjectedICallParams);
                sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");

                var injectedReturnClause = returnType == "()" ? "" : $" -> {returnType}";
                sb.AppendLine($"    pub fn {methodName}({parameters}){injectedReturnClause} {{}}");
                sb.AppendLine();
                return;
            }

            if (!csharpNameToRustName.TryGetValue(method.WrappedICallName, out var internalMethodName))
                internalMethodName = method.WrappedICallName.ToSnakeCase();

            var wrapperArgs = string.Join(", ", method.WrappedICallArgs.Select(a =>
                IsLiteralValue(a) ? a : RustKeywords.Escape(a.ToSnakeCase())));
            var selfPrefix = method.IsStatic ? "Self::" : "self.";

            var wrapperReturnClause = returnType == "()" ? "" : $" -> {returnType}";
            sb.AppendLine($"    pub fn {methodName}({parameters}){wrapperReturnClause} {{");
            sb.AppendLine($"        {selfPrefix}{internalMethodName}({wrapperArgs})");
            sb.AppendLine("    }");
            sb.AppendLine();
            return;
        }

        if (method.IsICall)
        {
            var icallName = BuildICallName(cls, ilName, method.Parameters);
            sb.AppendLine($"    #[unity_icall(\"{icallName}\")]");
        }
        else if (method.IsStatic)
        {
            sb.AppendLine($"    #[unity_method(name = \"{ilName}\", static)]");
        }
        else
        {
            sb.AppendLine($"    #[unity_method(name = \"{ilName}\")]");
        }

        var returnClause = returnType == "()" ? "" : $" -> {returnType}";
        sb.AppendLine($"    pub fn {methodName}({parameters}){returnClause} {{}}");
        sb.AppendLine();
    }

    private static string BuildParameterList(List<Il2CppParameter> parameters, bool hasSelf, string? currentModuleName = null)
    {
        var parts = new List<string>();

        if (hasSelf)
            parts.Add("&self");

        foreach (var param in parameters)
        {
            var paramName = RustKeywords.Escape(param.Name.ToSnakeCase());
            var paramType = currentModuleName is not null 
                ? TypeNameUtils.StripModulePrefix(param.Type, currentModuleName) 
                : param.Type;

            if (param.IsOut || param.IsRef)
            {
                if (paramType.StartsWith("&mut "))
                    paramType = paramType[5..].Trim();
                parts.Add($"{paramName}: &mut {paramType}");
            }
            else
            {
                parts.Add($"{paramName}: {paramType}");
            }
        }

        return string.Join(", ", parts);
    }

    private static string BuildICallName(Il2CppClass cls, string methodName, List<Il2CppParameter> parameters)
    {
        var ns = cls.Namespace;
        var fullClassName = string.IsNullOrEmpty(ns) ? cls.Name : $"{ns}.{cls.Name}";

        if (parameters.Count == 0)
            return $"{fullClassName}::{methodName}";

        var paramTypes = parameters.Select(p => p.CSharpType);
        return $"{fullClassName}::{methodName}({string.Join(",", paramTypes)})";
    }

    private static string BuildInjectedICallName(Il2CppClass cls, string methodName, List<string>? paramTypes)
    {
        var ns = cls.Namespace;
        var fullClassName = string.IsNullOrEmpty(ns) ? cls.Name : $"{ns}.{cls.Name}";

        if (paramTypes is null or { Count: 0 })
            return $"{fullClassName}::{methodName}";

        return $"{fullClassName}::{methodName}({string.Join(",", paramTypes)})";
    }

    private static string BuildICallNameForProperty(Il2CppClass cls, string methodName, string csharpType)
    {
        var ns = cls.Namespace;
        var fullClassName = string.IsNullOrEmpty(ns) ? cls.Name : $"{ns}.{cls.Name}";
        return $"{fullClassName}::{methodName}({csharpType})";
    }

    private static string GetUniqueMethodName(string baseName, Dictionary<string, int> methodNameCounts)
    {
        if (!methodNameCounts.TryGetValue(baseName, out var count))
        {
            methodNameCounts[baseName] = 1;
            return baseName;
        }

        methodNameCounts[baseName] = count + 1;
        return $"{baseName}_{count}";
    }

    private static bool IsLiteralValue(string value)
    {
        if (value is "None" or "true" or "false")
            return true;
        if (value.StartsWith('*'))
            return true;
        if (char.IsDigit(value[0]) || value[0] == '-')
            return true;
        return false;
    }
}
