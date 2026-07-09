using CaseConverter;
using dnlib.DotNet;
using IL2Resolver.Context;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;

namespace IL2Resolver.Analyzers;

public static class MethodAnalyzer
{
    public static Il2CppMethod Analyze(MethodDef methodDef, ValidationContext? validation = null, string? classFullName = null)
    {
        var (requiresTodo, todoReason) = TodoChecker.Check(methodDef.ReturnType);

        var isICall = ICallAnalyzer.IsICall(methodDef) || ICallAnalyzer.IsExternMethod(methodDef);

        WrapperInfo? wrapperInfo = null;
        var existsInRuntime = validation is null || !validation.IsEnabled ||
                              validation.MethodExists(classFullName ?? "", methodDef.Name.String);

        if (!isICall && existsInRuntime)
        {
            var candidateWrapper = ICallAnalyzer.AnalyzeWrapperChain(methodDef);
            if (candidateWrapper is { IsOutReturn: true } &&
                candidateWrapper.ICallName.EndsWith("_Injected", StringComparison.Ordinal) &&
                candidateWrapper.Arguments.All(arg => !arg.IsDefault))
                wrapperInfo = candidateWrapper;
        }

        var (wrappedICallName, wrappedICallArgs) = (null as string, null as List<string>);
        List<string>? injectedICallParams = null;
        var staticDelegate = !isICall && wrapperInfo is null ? ICallAnalyzer.GetStaticDelegateInfo(methodDef) : null;

        var methodName = methodDef.Name.String;

        if (methodName.EndsWith("_Injected"))
            methodName = methodName[..^9];

        var il2CppMethod = new Il2CppMethod
        {
            Name = methodName.ToPascalCase(),
            IlName = methodDef.Name.String,
            ReturnType = RustTypeMapper.Map(methodDef.ReturnType),
            IsStatic = methodDef.IsStatic,
            IsICall = isICall,
            WrappedICallName = wrappedICallName,
            WrappedICallArgs = wrappedICallArgs,
            InjectedICallParams = injectedICallParams,
            StaticDelegateField = staticDelegate?.FieldName,
            StaticDelegateMethod = staticDelegate?.MethodName,
            StaticDelegateParams = staticDelegate?.Params,
            RequiresTodo = requiresTodo,
            TodoReason = todoReason,
            WrapperInfo = wrapperInfo,
            ExistsInRuntime = existsInRuntime
        };

        foreach (var genericParam in methodDef.GenericParameters)
            il2CppMethod.GenericParameters.Add(genericParam.Name.String);

        foreach (var param in methodDef.Parameters.Where(p => p.IsNormalMethodParameter))
        {
            var isOut = param.ParamDef?.IsOut ?? false;
            var isRef = param.Type is ByRefSig && !isOut;

            var (paramRequiresTodo, paramTodoReason) = TodoChecker.Check(param.Type);
            if (paramRequiresTodo && !il2CppMethod.RequiresTodo)
            {
                requiresTodo = true;
                todoReason = paramTodoReason;
            }

            il2CppMethod.Parameters.Add(new Il2CppParameter
            {
                Name = param.Name,
                Type = RustTypeMapper.Map(param.Type, true, isOut),
                CSharpType = GetCSharpTypeName(param.Type),
                IsOut = isOut,
                IsRef = isRef,
                DefaultValue = param.ParamDef?.HasDefault == true
                    ? GetDefaultValueString(param.ParamDef.Constant?.Value)
                    : null
            });
        }

        return il2CppMethod;
    }

    public static Il2CppConstructor AnalyzeConstructor(MethodDef methodDef)
    {
        var ctor = new Il2CppConstructor();

        foreach (var param in methodDef.Parameters.Where(p => p.IsNormalMethodParameter))
        {
            var isOut = param.ParamDef?.IsOut ?? false;
            var isRef = param.Type is ByRefSig && !isOut;

            ctor.Parameters.Add(new Il2CppParameter
            {
                Name = param.Name,
                Type = RustTypeMapper.Map(param.Type, true, isOut),
                CSharpType = GetCSharpTypeName(param.Type),
                IsOut = isOut,
                IsRef = isRef,
                DefaultValue = param.ParamDef?.HasDefault == true
                    ? GetDefaultValueString(param.ParamDef.Constant?.Value)
                    : null
            });
        }

        return ctor;
    }

    public static bool IsOperatorMethod(MethodDef methodDef) =>
        methodDef.IsSpecialName && methodDef.Name.String.StartsWith("op_");

    private static string GetCSharpTypeName(TypeSig typeSig)
    {
        if (typeSig is ByRefSig byRef)
            return GetCSharpTypeName(byRef.Next) + "&";

        if (typeSig.IsSZArray && typeSig is ArraySigBase arraySig)
            return GetCSharpTypeName(arraySig.Next) + "[]";

        switch (typeSig)
        {
            case PtrSig ptrSig:
                return GetCSharpTypeName(ptrSig.Next) + "*";
            case GenericInstSig genericSig:
            {
                var baseName = genericSig.GenericType?.TypeName ?? "";
                var backtickIdx = baseName.IndexOf('`');
                if (backtickIdx > 0) baseName = baseName[..backtickIdx];
                var args = string.Join(",", genericSig.GenericArguments.Select(GetCSharpTypeName));
                return $"{baseName}<{args}>";
            }
            default:
                return GetSimpleTypeName(typeSig);
        }
    }

    private static string GetGenericBaseName(ITypeDefOrRef? typeRef)
    {
        if (typeRef is null)
            return "";

        var name = typeRef.Name.String;
        var backtick = name.IndexOf('`');
        return backtick > 0 ? name[..backtick] : name;
    }

    private static string GetSimpleTypeName(TypeSig typeSig)
    {
        var fullName = typeSig.FullName;

        if (fullName.StartsWith("System."))
            return fullName;

        if (fullName.Contains('/'))
        {
            var lastDot = fullName.LastIndexOf('.');
            if (lastDot > 0)
                fullName = fullName[(lastDot + 1)..];
            return fullName.Replace('/', '.');
        }

        var typeName = typeSig.TypeName;
        var backtick = typeName.IndexOf('`');
        return backtick > 0 ? typeName[..backtick] : typeName;
    }

    private static string? GetDefaultValueString(object? value) =>
        value switch
        {
            null => "None",
            bool b => b ? "true" : "false",
            string s => $"\"{s}\"",
            char c => $"'{c}'",
            float f => $"{f}_f32",
            double d => $"{d}_f64",
            _ => value.ToString()
        };
}