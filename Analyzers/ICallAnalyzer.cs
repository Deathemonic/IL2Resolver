using CaseConverter;
using dnlib.DotNet;
using dnlib.DotNet.Emit;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;
using ZLinq;

namespace IL2Resolver.Analyzers;

public static class ICallAnalyzer
{
    public static bool IsICall(MethodDef methodDef) =>
        (methodDef.ImplAttributes & MethodImplAttributes.InternalCall) != 0;

    public static bool IsCallableMethod(MethodDef methodDef)
    {
        if (IsICall(methodDef) || IsExternMethod(methodDef))
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
        return AreTypesCompatible(wrapperType, targetType, out _);
    }

    private static bool AreTypesCompatible(TypeSig wrapperType, TypeSig targetType, out bool needsIntoConversion)
    {
        needsIntoConversion = false;
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

        // Check if wrapper type inherits from target type (e.g., Cubemap inherits from Texture)
        if (wrapperDef.IsClass && InheritsFrom(wrapperDef, targetDef))
        {
            needsIntoConversion = true;
            return true;
        }

        return false;
    }

    private static bool InheritsFrom(TypeDef derivedType, TypeDef baseType)
    {
        var current = derivedType.BaseType;
        while (current is not null)
        {
            if (current.FullName == baseType.FullName)
                return true;

            var currentDef = current.ResolveTypeDef();
            if (currentDef is null)
                break;

            current = currentDef.BaseType;
        }

        return false;
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

    public static WrapperInfo? AnalyzeWrapperChain(MethodDef methodDef)
    {
        if (!methodDef.HasBody || methodDef.Body.Instructions.Count < 2)
            return null;

        var (targetICall, argMappings) = WalkChainToICall(methodDef, []);
        if (targetICall is null || argMappings is null)
            return null;

        var icallParams = targetICall.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var wrapperParams = methodDef.Parameters.Where(p => p.IsNormalMethodParameter).ToList();

        var isOutReturn = DetectOutReturnPattern(methodDef, targetICall);
        string? outReturnType = null;
        string? outReturnRustType = null;

        if (isOutReturn && icallParams.Count > 0)
        {
            var outParam = icallParams.FirstOrDefault(p => p.ParamDef?.IsOut == true);
            if (outParam is not null)
            {
                var baseType = outParam.Type is ByRefSig byRef ? byRef.Next : outParam.Type;
                outReturnType = GetCSharpTypeName(baseType);
                outReturnRustType = RustTypeMapper.Map(baseType);
            }
        }

        var arguments = BuildICallArguments(wrapperParams, icallParams, argMappings, isOutReturn);
        if (arguments is null)
            return null;

        var hasDefaults = arguments.Any(a => a.IsDefault);
        var hasMutCopy = arguments.Any(a => a.NeedsMutCopy);
        var hasIntoConversion = arguments.Any(a => a.NeedsIntoConversion);
        var hasThisArg = arguments.Any(a => a.Value == "__this__" || a.SourceParam == "__this__");

        if (hasThisArg)
            return null;

        if (icallParams.Count > 0 && targetICall.IsStatic && !methodDef.IsStatic)
        {
            var firstParamType = icallParams[0].Type;
            var baseType = firstParamType is ByRefSig byRef ? byRef.Next : firstParamType;
            if (baseType.FullName == methodDef.DeclaringType?.FullName)
                return null;
        }

        if (!isOutReturn && !hasDefaults && !hasMutCopy && !hasIntoConversion)
            return null;

        var icallCSharpParams = icallParams.Select(p => GetCSharpTypeName(p.Type)).ToList();
        var icallParamNames = icallParams.Select(p => p.Name).ToList();

        return new WrapperInfo
        {
            ICallName = targetICall.Name.String,
            ICallCSharpParams = icallCSharpParams,
            ICallParamNames = icallParamNames,
            Arguments = arguments,
            IsOutReturn = isOutReturn,
            OutReturnType = outReturnType,
            OutReturnRustType = outReturnRustType
        };
    }

    private static (MethodDef? ICall, Dictionary<int, ArgumentMapping>? Mappings) WalkChainToICall(
        MethodDef methodDef, HashSet<string> visited)
    {
        if (!visited.Add(methodDef.FullName))
            return (null, null);

        if (!methodDef.HasBody)
            return (null, null);

        var instructions = methodDef.Body.Instructions;
        MethodDef? calledMethod = null;
        var callInstruction = -1;

        for (var i = 0; i < instructions.Count; i++)
        {
            var instr = instructions[i];
            if (instr.OpCode.Code is not (Code.Call or Code.Callvirt))
                continue;

            if (instr.Operand is not IMethodDefOrRef methodRef)
                continue;

            var resolved = methodRef.ResolveMethodDef();
            if (resolved is null)
                continue;

            if (resolved.DeclaringType?.FullName != methodDef.DeclaringType?.FullName)
                continue;

            if (calledMethod is not null)
                return (null, null);

            calledMethod = resolved;
            callInstruction = i;
        }

        if (calledMethod is null || callInstruction < 0)
            return (null, null);

        var argMappings = ExtractArgumentMappings(methodDef, calledMethod, instructions, callInstruction);
        if (argMappings is null)
            return (null, null);

        if (IsICall(calledMethod) || IsExternMethod(calledMethod))
            return (calledMethod, argMappings);

        var (nestedICall, nestedMappings) = WalkChainToICall(calledMethod, visited);
        if (nestedICall is null || nestedMappings is null)
            return (null, null);

        var combinedMappings = CombineMappings(argMappings, nestedMappings, methodDef, calledMethod);
        return (nestedICall, combinedMappings);
    }

    private static Dictionary<int, ArgumentMapping>? ExtractArgumentMappings(
        MethodDef wrapper, MethodDef target, IList<Instruction> instructions, int callIndex)
    {
        var wrapperParams = wrapper.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var targetParams = target.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var mappings = new Dictionary<int, ArgumentMapping>();

        var argStack = new List<ArgumentMapping>();

        for (var i = 0; i < callIndex; i++)
        {
            var instr = instructions[i];
            var (mapping, stackEffect) = InstructionToArgumentMapping(instr, wrapperParams, wrapper.IsStatic, argStack);

            if (stackEffect < 0)
            {
                for (var j = 0; j < -stackEffect && argStack.Count > 0; j++)
                    argStack.RemoveAt(argStack.Count - 1);
            }

            if (mapping is null && stackEffect > 0)
                return null;

            if (mapping is not null)
                argStack.Add(mapping);
        }

        var expectedArgs = targetParams.Count;
        if (!target.IsStatic)
            expectedArgs++;

        if (argStack.Count < expectedArgs)
            return null;

        var startIndex = argStack.Count - expectedArgs;

        if (!target.IsStatic)
            startIndex++;

        for (var i = 0; i < targetParams.Count; i++)
        {
            var targetParam = targetParams[i];
            var sourceMapping = argStack[startIndex + i];

            if (sourceMapping.Value == "__out_local__")
            {
                mappings[i] = new ArgumentMapping("__out_local__", false, false);
                continue;
            }

            if (sourceMapping.SourceParam is null)
            {
                mappings[i] = sourceMapping;
                continue;
            }

            var srcParam = wrapperParams.FirstOrDefault(p => p.Name == sourceMapping.SourceParam);
            if (srcParam is null)
            {
                mappings[i] = sourceMapping;
                continue;
            }

            if (!AreTypesCompatible(srcParam.Type, targetParam.Type, out var needsIntoConversion))
                return null;

            var needsMutCopy = targetParam.Type is ByRefSig && srcParam.Type is not ByRefSig;

            mappings[i] = sourceMapping with
            {
                NeedsMutCopy = needsMutCopy,
                NeedsIntoConversion = needsIntoConversion
            };
        }

        return mappings;
    }

    private static (ArgumentMapping? Mapping, int StackEffect) InstructionToArgumentMapping(
        Instruction instr, List<Parameter> wrapperParams, bool isStatic, List<ArgumentMapping> currentStack)
    {
        switch (instr.OpCode.Code)
        {
            case Code.Ldarg_0:
                if (isStatic && wrapperParams.Count > 0)
                    return (new ArgumentMapping(wrapperParams[0].Name, false, false), 1);
                if (!isStatic)
                    return (new ArgumentMapping("__this__", false, false), 1);
                return (null, 0);

            case Code.Ldarg_1:
                var idx1 = isStatic ? 1 : 0;
                if (idx1 < wrapperParams.Count)
                    return (new ArgumentMapping(wrapperParams[idx1].Name, false, false), 1);
                return (null, 0);

            case Code.Ldarg_2:
                var idx2 = isStatic ? 2 : 1;
                if (idx2 < wrapperParams.Count)
                    return (new ArgumentMapping(wrapperParams[idx2].Name, false, false), 1);
                return (null, 0);

            case Code.Ldarg_3:
                var idx3 = isStatic ? 3 : 2;
                if (idx3 < wrapperParams.Count)
                    return (new ArgumentMapping(wrapperParams[idx3].Name, false, false), 1);
                return (null, 0);

            case Code.Ldarg_S or Code.Ldarg:
                if (instr.Operand is Parameter param && wrapperParams.Contains(param))
                    return (new ArgumentMapping(param.Name, false, false), 1);
                return (null, 0);

            case Code.Ldarga_S or Code.Ldarga:
                if (instr.Operand is Parameter refParam && wrapperParams.Contains(refParam))
                    return (new ArgumentMapping(refParam.Name, false, true), 1);
                return (null, 0);

            case Code.Ldloca_S or Code.Ldloca:
                return (new ArgumentMapping("__out_local__", false, false), 1);

            case Code.Ldc_I4_0:
                return (new ArgumentMapping("0", true, false), 1);
            case Code.Ldc_I4_1:
                return (new ArgumentMapping("1", true, false), 1);
            case Code.Ldc_I4_2:
                return (new ArgumentMapping("2", true, false), 1);
            case Code.Ldc_I4_3:
                return (new ArgumentMapping("3", true, false), 1);
            case Code.Ldc_I4_4:
                return (new ArgumentMapping("4", true, false), 1);
            case Code.Ldc_I4_5:
                return (new ArgumentMapping("5", true, false), 1);
            case Code.Ldc_I4_6:
                return (new ArgumentMapping("6", true, false), 1);
            case Code.Ldc_I4_7:
                return (new ArgumentMapping("7", true, false), 1);
            case Code.Ldc_I4_8:
                return (new ArgumentMapping("8", true, false), 1);
            case Code.Ldc_I4_M1:
                return (new ArgumentMapping("-1", true, false), 1);

            case Code.Ldc_I4_S:
                return (new ArgumentMapping(((sbyte)instr.Operand).ToString(), true, false), 1);
            case Code.Ldc_I4:
                return (new ArgumentMapping(((int)instr.Operand).ToString(), true, false), 1);
            case Code.Ldc_I8:
                return (new ArgumentMapping(((long)instr.Operand).ToString(), true, false), 1);
            case Code.Ldc_R4:
                return (new ArgumentMapping(TypeMappings.FormatFloat((float)instr.Operand), true, false), 1);
            case Code.Ldc_R8:
                return (new ArgumentMapping(TypeMappings.FormatDouble((double)instr.Operand), true, false), 1);

            case Code.Ldnull:
                return (new ArgumentMapping("None", true, false), 1);

            case Code.Call when instr.Operand is IMethodDefOrRef methodRef:
            {
                var resolved = methodRef.ResolveMethodDef();
                var paramCount = resolved?.Parameters.Count(p => p.IsNormalMethodParameter) ?? 0;
                var isInstance = resolved is { IsStatic: false };
                if (isInstance) paramCount++;
                
                var typeName = methodRef.DeclaringType?.Name.String ?? "";
                var methodName = methodRef.Name.String;
                var rustDefault = TypeMappings.GetDefaultMethod(typeName, methodName);
                if (rustDefault is not null)
                    return (new ArgumentMapping(rustDefault, true, false), 1 - paramCount);
                return (null, 1);
            }

            case Code.Ldsfld when instr.Operand is IField field:
                var fieldTypeName = field.DeclaringType?.Name.String ?? "";
                var fieldName = field.Name.String;
                var fieldDefault = TypeMappings.GetDefaultField(fieldTypeName, fieldName);
                if (fieldDefault is not null)
                    return (new ArgumentMapping(fieldDefault, true, false), 1);
                return (null, 1);

            case Code.Ldfld:
                return (null, 1);

            case Code.Nop or Code.Ret:
                return (null, 0);

            case Code.Conv_I or Code.Conv_I1 or Code.Conv_I2 or Code.Conv_I4 or Code.Conv_I8
                or Code.Conv_U or Code.Conv_U1 or Code.Conv_U2 or Code.Conv_U4 or Code.Conv_U8
                or Code.Conv_R4 or Code.Conv_R8:
                return (null, 0);

            case Code.Box or Code.Unbox or Code.Unbox_Any or Code.Castclass or Code.Isinst:
                return (null, 0);

            case Code.Dup:
                if (currentStack.Count > 0)
                    return (currentStack[^1], 1);
                return (null, 1);

            default:
                return (null, 0);
        }
    }

    private static Dictionary<int, ArgumentMapping>? CombineMappings(
        Dictionary<int, ArgumentMapping> outerMappings,
        Dictionary<int, ArgumentMapping> innerMappings,
        MethodDef outerMethod,
        MethodDef innerMethod)
    {
        var innerParams = innerMethod.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var combined = new Dictionary<int, ArgumentMapping>();

        foreach (var (icallIdx, innerMapping) in innerMappings)
        {
            if (innerMapping.IsDefault)
            {
                combined[icallIdx] = innerMapping;
                continue;
            }

            var innerParamIdx = innerParams.FindIndex(p => p.Name == innerMapping.SourceParam);
            if (innerParamIdx < 0 || !outerMappings.TryGetValue(innerParamIdx, out var outerMapping))
                return null;

            var needsMutCopy = innerMapping.NeedsMutCopy || outerMapping.NeedsMutCopy;
            var needsIntoConversion = innerMapping.NeedsIntoConversion || outerMapping.NeedsIntoConversion;
            combined[icallIdx] = outerMapping with { NeedsMutCopy = needsMutCopy, NeedsIntoConversion = needsIntoConversion };
        }

        return combined;
    }

    private static List<ICallArgument>? BuildICallArguments(
        List<Parameter> wrapperParams,
        List<Parameter> icallParams,
        Dictionary<int, ArgumentMapping> mappings,
        bool isOutReturn)
    {
        var arguments = new List<ICallArgument>();

        for (var i = 0; i < icallParams.Count; i++)
        {
            var icallParam = icallParams[i];

            if (!mappings.TryGetValue(i, out var mapping))
                return null;

            if (mapping.Value == "__out_local__")
            {
                if (isOutReturn && icallParam.ParamDef?.IsOut == true)
                {
                    arguments.Add(new ICallArgument
                    {
                        Value = "__out_return__",
                        IsDefault = false,
                        NeedsMutCopy = false,
                        SourceParam = null
                    });
                }
                else
                {
                    return null;
                }
                continue;
            }

            if (isOutReturn && icallParam.ParamDef?.IsOut == true)
            {
                arguments.Add(new ICallArgument
                {
                    Value = "__out_return__",
                    IsDefault = false,
                    NeedsMutCopy = false,
                    SourceParam = null
                });
                continue;
            }

            var rustValue = mapping.IsDefault
                ? ConvertDefaultValue(mapping.Value, icallParam.Type)
                : mapping.Value.ToSnakeCase();

            arguments.Add(new ICallArgument
            {
                Value = rustValue,
                IsDefault = mapping.IsDefault,
                NeedsMutCopy = mapping.NeedsMutCopy,
                NeedsIntoConversion = mapping.NeedsIntoConversion,
                SourceParam = mapping.IsDefault ? null : mapping.Value
            });
        }

        return arguments;
    }

    private static string ConvertDefaultValue(string value, TypeSig paramType)
    {
        var baseType = paramType is ByRefSig byRef ? byRef.Next : paramType;
        var typeName = baseType.FullName;

        if (typeName == "System.Boolean")
        {
            return value switch
            {
                "0" => "false",
                "1" => "true",
                _ => value
            };
        }

        var typeDef = baseType.TryGetTypeDef();
        if (typeDef is { IsEnum: true })
        {
            var rustTypeName = typeDef.Name.String.ToPascalCase();
            return $"unsafe {{ std::mem::transmute::<i32, {rustTypeName}>({value}) }}";
        }

        return value;
    }

    private static bool DetectOutReturnPattern(MethodDef wrapper, MethodDef icall)
    {
        if (wrapper.ReturnType.FullName == "System.Void")
            return false;

        var icallParams = icall.Parameters.Where(p => p.IsNormalMethodParameter).ToList();
        var outParam = icallParams.FirstOrDefault(p => p.ParamDef?.IsOut == true);

        if (outParam is null)
            return false;

        var outType = outParam.Type is ByRefSig byRef ? byRef.Next : outParam.Type;
        return AreTypesCompatible(wrapper.ReturnType, outType);
    }

    private record ArgumentMapping(string Value, bool IsDefault, bool NeedsMutCopy, bool NeedsIntoConversion = false, string? SourceParam = null)
    {
        public string? SourceParam { get; init; } = SourceParam ?? (IsDefault ? null : Value);
    }
}