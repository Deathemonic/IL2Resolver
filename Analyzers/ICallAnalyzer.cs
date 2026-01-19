using dnlib.DotNet;
using dnlib.DotNet.Emit;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;
using ZLinq;

namespace IL2Resolver.Analyzers;

public static class ICallAnalyzer
{
    public static bool IsICall(MethodDef methodDef)
    {
        if ((methodDef.ImplAttributes & MethodImplAttributes.InternalCall) != 0)
            return true;

        foreach (var attr in methodDef.CustomAttributes)
            if (attr.TypeFullName is "UnityEngine.Bindings.FreeFunctionAttribute"
                or "UnityEngine.Bindings.NativeMethodAttribute"
                or "FreeFunctionAttribute" or "NativeMethodAttribute")
                return true;

        return false;
    }

    public static bool IsExternMethod(MethodDef methodDef)
    {
        if (!methodDef.HasBody &&
            ((methodDef.ImplAttributes & MethodImplAttributes.InternalCall) != 0 ||
             (methodDef.Attributes & MethodAttributes.PinvokeImpl) != 0))
            return true;

        foreach (var arg in methodDef.CustomAttributes
                     .AsValueEnumerable()
                     .Where(attr => attr.TypeFullName == "System.Runtime.CompilerServices.MethodImplAttribute"
                                    && attr.ConstructorArguments.Count > 0)
                     .Select(attr => attr.ConstructorArguments[0]))
            switch (arg.Value)
            {
                case 4096:
                case short and 4096:
                    return true;
            }

        return false;
    }

    public static bool IsSimpleWrapper(MethodDef methodDef)
    {
        if (!methodDef.HasBody || methodDef.Body.Instructions.Count < 2)
            return false;

        return methodDef.Body.Instructions
            .AsValueEnumerable()
            .Select(instr => instr.OpCode.Code)
            .Select(code =>
                code is Code.Nop or Code.Ret or Code.Ldarg_0 or Code.Ldarg_1 or Code.Ldarg_2 or Code.Ldarg_3
                    or Code.Ldarg_S
                    or Code.Ldarg or Code.Ldarga_S or Code.Ldarga or Code.Call or Code.Callvirt or Code.Conv_I
                    or Code.Conv_I1 or Code.Conv_I2 or Code.Conv_I4 or Code.Conv_I8 or Code.Conv_U or Code.Conv_U1
                    or Code.Conv_U2 or Code.Conv_U4 or Code.Conv_U8 or Code.Conv_R4 or Code.Conv_R8 or Code.Ldc_I4_0
                    or Code.Ldc_I4_1 or Code.Ldc_I4_2 or Code.Ldc_I4_3 or Code.Ldc_I4_4 or Code.Ldc_I4_5
                    or Code.Ldc_I4_6
                    or Code.Ldc_I4_7 or Code.Ldc_I4_8 or Code.Ldc_I4_M1 or Code.Ldc_I4_S or Code.Ldc_I4 or Code.Ldc_I8
                    or Code.Ldc_R4 or Code.Ldc_R8 or Code.Ldnull or Code.Box or Code.Unbox or Code.Unbox_Any
                    or Code.Castclass or Code.Isinst or Code.Ldflda or Code.Ldfld)
            .All(isSimple => isSimple);
    }

    public static MethodDef? FindTargetICall(MethodDef methodDef) => FindTargetICallInternal(methodDef, []);

    public static bool HasMatchingParams(MethodDef wrapper, MethodDef icall)
    {
        var wrapperParams = wrapper.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var icallParams = icall.Parameters.Where(p => p.IsNormalMethodParameter).ToList();

        if (wrapper.IsStatic == icall.IsStatic)
        {
            if (wrapperParams.Count != icallParams.Count)
                return false;

            return !wrapperParams.Where((t, i) => t.Type.FullName != icallParams[i].Type.FullName).Any();
        }

        if (wrapper.IsStatic || !icall.IsStatic || icallParams.Count <= 0) return false;
        {
            if (wrapperParams.Count != icallParams.Count - 1)
                return false;

            return !wrapperParams.Where((t, i) => t.Type.FullName != icallParams[i + 1].Type.FullName).Any();
        }
    }

    private static MethodDef? FindTargetICallInternal(MethodDef methodDef, HashSet<string> visited)
    {
        if (!methodDef.HasBody)
            return null;

        if (!visited.Add(methodDef.FullName))
            return null;

        foreach (var instr in methodDef.Body.Instructions)
        {
            if (instr.OpCode.Code is not (Code.Call or Code.Callvirt))
                continue;

            if (instr.Operand is not IMethodDefOrRef methodRef)
                continue;

            var resolved = methodRef.ResolveMethodDef();
            if (resolved is null)
                continue;

            if (resolved.DeclaringType?.FullName != methodDef.DeclaringType?.FullName)
                continue;

            if (IsICall(resolved) || IsExternMethod(resolved))
                return resolved;

            if (!resolved.HasBody)
                continue;

            var nested = FindTargetICallInternal(resolved, visited);
            if (nested is not null)
                return nested;
        }

        return null;
    }

    public static (string? Name, List<string>? Args) GetWrappedICallInfo(MethodDef methodDef)
    {
        if (!methodDef.HasBody || methodDef.Body.Instructions.Count < 2)
            return (null, null);

        var instructions = methodDef.Body.Instructions;

        MethodDef? calledMethod = null;
        var callCount = 0;

        foreach (var instr in instructions)
            if (instr.OpCode.Code == Code.Call && instr.Operand is IMethodDefOrRef methodRef)
            {
                callCount++;
                if (callCount > 1)
                    return (null, null);
                calledMethod = methodRef.ResolveMethodDef();
            }

        if (calledMethod is null || calledMethod.DeclaringType.FullName != methodDef.DeclaringType.FullName ||
            calledMethod.FullName == methodDef.FullName || methodDef.IsStatic != calledMethod.IsStatic)
            return (null, null);

        var isInjectedWrapper = calledMethod.Name.String.EndsWith("_Injected") && IsExternMethod(calledMethod);
        var isRegularICallWrapper = IsICall(calledMethod) && (calledMethod.IsPrivate || calledMethod.IsAssembly);

        if ((!isInjectedWrapper && !isRegularICallWrapper) ||
            (isRegularICallWrapper && calledMethod.Name.String == methodDef.Name.String))
            return (null, null);

        if (isInjectedWrapper)
            return (calledMethod.Name.String, ["__injected__"]);

        if (!AreTypesCompatible(methodDef.ReturnType, calledMethod.ReturnType))
            return (null, null);

        var args = ExtractCallArguments(methodDef, calledMethod);
        return args is null ? (null, null) : (calledMethod.Name.String, args);
    }

    public static List<string>? GetInjectedICallParams(MethodDef methodDef)
    {
        if (!methodDef.HasBody || methodDef.Body.Instructions.Count < 2)
            return null;

        MethodDef? calledMethod = null;
        var callCount = 0;

        foreach (var instr in methodDef.Body.Instructions)
            if (instr.OpCode.Code == Code.Call && instr.Operand is IMethodDefOrRef methodRef)
            {
                callCount++;
                if (callCount > 1)
                    return null;
                calledMethod = methodRef.ResolveMethodDef();
            }

        if (calledMethod is null || !calledMethod.Name.String.EndsWith("_Injected") || !IsExternMethod(calledMethod))
            return null;

        return calledMethod.Parameters
            .Where(p => p.IsNormalMethodParameter)
            .Select(p => GetCSharpTypeName(p.Type))
            .ToList();
    }

    public static StaticDelegateInfo? GetStaticDelegateInfo(MethodDef methodDef)
    {
        if (methodDef.IsStatic || !methodDef.HasBody || methodDef.Body.Instructions.Count < 3)
            return null;

        var instructions = methodDef.Body.Instructions;

        MethodDef? calledMethod = null;
        string? fieldName = null;
        var callCount = 0;
        var hasLdflda = false;

        foreach (var instr in instructions)
            switch (instr.OpCode.Code)
            {
                case Code.Ldarg_0:
                    continue;
                case Code.Ldflda when instr.Operand is IField fieldRef:
                    fieldName = fieldRef.Name.String;
                    hasLdflda = true;
                    continue;
                case Code.Call when instr.Operand is IMethodDefOrRef methodRef:
                {
                    callCount++;
                    if (callCount > 1)
                        return null;
                    calledMethod = methodRef.ResolveMethodDef();
                    break;
                }
            }

        if (calledMethod is null || !hasLdflda || fieldName is null)
            return null;

        if (calledMethod.DeclaringType.FullName != methodDef.DeclaringType.FullName)
            return null;

        if (!calledMethod.IsStatic)
            return null;

        if (!IsICall(calledMethod) && !IsExternMethod(calledMethod))
            return null;

        var calledParams = calledMethod.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        if (calledParams.Count == 0)
            return null;

        var firstParam = calledParams[0];
        if (firstParam.Type is not ByRefSig)
            return null;

        var wrapperParams = methodDef.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        if (wrapperParams.Count != calledParams.Count - 1)
            return null;

        for (var i = 0; i < wrapperParams.Count; i++)
            if (!AreTypesCompatible(wrapperParams[i].Type, calledParams[i + 1].Type))
                return null;

        var delegateParams = calledParams.Select(param =>
        {
            var isOut = param.ParamDef?.IsOut ?? false;
            var isRef = param.Type is ByRefSig && !isOut;
            return new Il2CppParameter
            {
                Name = param.Name,
                Type = RustTypeMapper.Map(param.Type, true, isOut),
                CSharpType = GetCSharpTypeName(param.Type),
                IsOut = isOut,
                IsRef = isRef
            };
        }).ToList();

        return new StaticDelegateInfo(fieldName, calledMethod.Name.String, delegateParams);
    }

    private static List<string>? ExtractCallArguments(MethodDef methodDef, MethodDef calledMethod)
    {
        var args = new List<string>();
        var wrapperParams = methodDef.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var targetParams = calledMethod.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var expectedArgCount = targetParams.Count;

        if (targetParams.Any(targetParam => targetParam.Type.IsSZArray)) return null;

        foreach (var instr in methodDef.Body.Instructions)
        {
            var argIndex = args.Count;
            var targetParam = argIndex < targetParams.Count ? targetParams[argIndex] : null;
            var targetType = targetParam?.Type;
            var isBoolTarget = targetType?.FullName == "System.Boolean";

            switch (instr.OpCode.Code)
            {
                case Code.Ldarg_0:
                    if (methodDef.IsStatic && wrapperParams.Count > 0)
                    {
                        if (targetType is not null && !AreTypesCompatible(wrapperParams[0].Type, targetType))
                            return null;
                        args.Add(GetArgWithDeref(wrapperParams[0], targetType));
                    }

                    break;
                case Code.Ldarg_1:
                    var idx1 = methodDef.IsStatic ? 1 : 0;
                    if (idx1 < wrapperParams.Count)
                    {
                        if (targetType is not null && !AreTypesCompatible(wrapperParams[idx1].Type, targetType))
                            return null;
                        args.Add(GetArgWithDeref(wrapperParams[idx1], targetType));
                    }

                    break;
                case Code.Ldarg_2:
                    var idx2 = methodDef.IsStatic ? 2 : 1;
                    if (idx2 < wrapperParams.Count)
                    {
                        if (targetType is not null && !AreTypesCompatible(wrapperParams[idx2].Type, targetType))
                            return null;
                        args.Add(GetArgWithDeref(wrapperParams[idx2], targetType));
                    }

                    break;
                case Code.Ldarg_3:
                    var idx3 = methodDef.IsStatic ? 3 : 2;
                    if (idx3 < wrapperParams.Count)
                    {
                        if (targetType is not null && !AreTypesCompatible(wrapperParams[idx3].Type, targetType))
                            return null;
                        args.Add(GetArgWithDeref(wrapperParams[idx3], targetType));
                    }

                    break;
                case Code.Ldarg_S or Code.Ldarg:
                    if (instr.Operand is Parameter param && wrapperParams.Contains(param))
                    {
                        if (targetType is not null && !AreTypesCompatible(param.Type, targetType))
                            return null;
                        args.Add(GetArgWithDeref(param, targetType));
                    }

                    break;
                case Code.Ldc_I4_0:
                    if (targetType is null) return null;
                    args.Add(isBoolTarget ? "false" : IsIntegerType(targetType) ? "0" : null!);
                    break;
                case Code.Ldc_I4_1:
                    if (targetType is null) return null;
                    args.Add(isBoolTarget ? "true" : IsIntegerType(targetType) ? "1" : null!);
                    break;
                case Code.Ldc_I4_S or Code.Ldc_I4:
                case Code.Ldc_I8:
                    if (targetType is null || !IsIntegerType(targetType)) return null;
                    args.Add(instr.Operand?.ToString() ?? "0");
                    break;
                case Code.Ldc_R4:
                case Code.Ldc_R8:
                    if (targetType is null || !IsFloatType(targetType)) return null;
                    args.Add(instr.Operand?.ToString() ?? "0.0");
                    break;
                case Code.Ldnull:
                    args.Add("None");
                    break;
                case Code.Conv_U4 or Code.Conv_U8 or Code.Conv_I4 or Code.Conv_I8 or Code.Conv_R4 or Code.Conv_R8:
                case Code.Call or Code.Ret:
                    break;
            }
        }

        return args.Count != expectedArgCount ? null : args;
    }

    private static string GetArgWithDeref(Parameter wrapperParam, TypeSig? targetType)
    {
        var paramName = wrapperParam.Name;
        var wrapperType = wrapperParam.Type;

        if (wrapperType is ByRefSig && targetType is not ByRefSig)
            return $"*{paramName}";

        return paramName;
    }

    private static bool IsIntegerType(TypeSig typeSig) =>
        typeSig.FullName is "System.Int32" or "System.UInt32" or "System.Int64" or "System.UInt64"
            or "System.Int16" or "System.UInt16" or "System.Byte" or "System.SByte" or "System.Boolean";

    private static bool IsFloatType(TypeSig typeSig) => typeSig.FullName is "System.Single" or "System.Double";

    private static bool AreTypesCompatible(TypeSig wrapperType, TypeSig targetType)
    {
        var wrapperIsRef = wrapperType is ByRefSig;
        var targetIsRef = targetType is ByRefSig;

        if (wrapperIsRef)
            wrapperType = ((ByRefSig)wrapperType).Next;
        if (targetIsRef)
            targetType = ((ByRefSig)targetType).Next;

        if (!wrapperIsRef && targetIsRef)
            return false;

        if (wrapperType.FullName == targetType.FullName)
            return true;

        if (wrapperType.FullName == "System.Void" || targetType.FullName == "System.Void" ||
            wrapperType is GenericInstSig || targetType is GenericInstSig)
            return wrapperType.FullName == targetType.FullName;

        var wrapperDef = wrapperType.TryGetTypeDef();
        var targetDef = targetType.TryGetTypeDef();

        if (wrapperDef is null || targetDef is null)
            return false;

        return wrapperDef.IsClass && targetDef.FullName == "System.Object";
    }

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
                var genBaseName = genericSig.GenericType?.TypeName ?? "";
                var backtick = genBaseName.IndexOf('`');
                if (backtick > 0) genBaseName = genBaseName[..backtick];
                var args = string.Join(",", genericSig.GenericArguments.Select(GetCSharpTypeName));
                return $"{genBaseName}<{args}>";
            }
        }

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
        var idx = typeName.IndexOf('`');
        return idx > 0 ? typeName[..idx] : typeName;
    }

    public record StaticDelegateInfo(string FieldName, string MethodName, List<Il2CppParameter> Params);
}