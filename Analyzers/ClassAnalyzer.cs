using CaseConverter;
using dnlib.DotNet;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Analyzers;

public static class ClassAnalyzer
{
    public static Il2CppClass Analyze(TypeDef typeDef)
    {
        var currentAssemblyName = typeDef.Module.Assembly.Name.String;

        var il2CppClass = new Il2CppClass
        {
            Name = TypeNameUtils.GetCleanName(typeDef.Name.String),
            Namespace = GetEffectiveNamespace(typeDef),
            BaseTypeName = GetBaseTypeName(typeDef),
            BaseTypeModule = TypeTracker.GetTypeModule(typeDef.BaseType, currentAssemblyName),
            IsValueType = typeDef is { IsValueType: true, IsEnum: false },
            IsStatic = typeDef is { IsAbstract: true, IsSealed: true }
        };

        BuildInheritanceChain(typeDef, il2CppClass, currentAssemblyName);

        foreach (var genericParam in typeDef.GenericParameters)
            il2CppClass.GenericParameters.Add(genericParam.Name.String);

        var propertyMethodNames = new HashSet<string>();
        AnalyzeProperties(typeDef, il2CppClass, currentAssemblyName, propertyMethodNames);
        AnalyzeMethods(typeDef, il2CppClass, currentAssemblyName, propertyMethodNames);
        AnalyzeNestedEnums(typeDef, il2CppClass);
        AnalyzeFields(typeDef, il2CppClass, currentAssemblyName);

        return il2CppClass;
    }

    private static void BuildInheritanceChain(TypeDef typeDef, Il2CppClass il2CppClass, string currentAssemblyName)
    {
        var current = typeDef.BaseType;
        while (current is not null)
        {
            var fullName = current.FullName;
            if (fullName is "System.Object" or "System.ValueType" or "System.Enum")
                break;

            var resolved = current.ResolveTypeDef();
            if (resolved is null)
                break;

            var name = current.Name.String;
            var backtickIndex = name.IndexOf('`');
            if (backtickIndex > 0)
                name = name[..backtickIndex];

            il2CppClass.InheritanceChain.Add(new Il2CppBaseType
            {
                Name = name,
                Namespace = resolved.Namespace?.String ?? "",
                Module = TypeTracker.GetTypeModule(current, currentAssemblyName)
            });

            current = resolved.BaseType;
        }
    }

    private static void AnalyzeProperties(TypeDef typeDef, Il2CppClass il2CppClass, string currentAssemblyName,
        HashSet<string> propertyMethodNames)
    {
        foreach (var property in typeDef.Properties)
        {
            if (!PropertyAnalyzer.IsPublic(property)) continue;

            var il2CppProperty = PropertyAnalyzer.Analyze(property);
            il2CppClass.Properties.Add(il2CppProperty);

            var propName = property.Name.String.ToPascalCase();
            if (il2CppProperty.HasGetter)
                propertyMethodNames.Add($"Get{propName}");
            if (il2CppProperty.HasSetter)
                propertyMethodNames.Add($"Set{propName}");

            if (il2CppProperty.GetterInjectedICallName is not null)
                propertyMethodNames.Add(il2CppProperty.GetterInjectedICallName);
            if (il2CppProperty.SetterInjectedICallName is not null)
                propertyMethodNames.Add(il2CppProperty.SetterInjectedICallName);

            var propType = property.PropertySig?.RetType;
            TypeTracker.TrackReferencedType(propType, il2CppClass.ReferencedTypes);
            TypeTracker.TrackExternalType(propType, currentAssemblyName, il2CppClass.ExternalTypes);
        }
    }

    private static void AnalyzeMethods(TypeDef typeDef, Il2CppClass il2CppClass, string currentAssemblyName,
        HashSet<string> propertyMethodNames)
    {
        var wrappedIcalls = new HashSet<string>();
        foreach (var method in typeDef.Methods)
        {
            if (!method.IsPublic || method.IsGetter || method.IsSetter || method.IsConstructor)
                continue;

            var isICall = ICallAnalyzer.IsICall(method) || ICallAnalyzer.IsExternMethod(method);
            if (isICall)
                continue;

            if (!ICallAnalyzer.IsSimpleWrapper(method)) continue;
            var targetICall = ICallAnalyzer.FindTargetICall(method);
            if (targetICall is not null)
                wrappedIcalls.Add(targetICall.FullName);
        }

        foreach (var method in typeDef.Methods)
        {
            if (method.IsGetter || method.IsSetter)
                continue;
            if (method.IsConstructor)
            {
                if (method is { IsPublic: true, IsStatic: false } && !il2CppClass.IsValueType)
                {
                    var ctor = MethodAnalyzer.AnalyzeConstructor(method);
                    il2CppClass.Constructors.Add(ctor);
                    foreach (var param in method.Parameters.Where(p => p.IsNormalMethodParameter))
                    {
                        TypeTracker.TrackReferencedType(param.Type, il2CppClass.ReferencedTypes);
                        TypeTracker.TrackExternalType(param.Type, currentAssemblyName, il2CppClass.ExternalTypes);
                    }
                }

                continue;
            }

            if (MethodAnalyzer.IsOperatorMethod(method))
                continue;

            var isICall = ICallAnalyzer.IsICall(method) || ICallAnalyzer.IsExternMethod(method);

            if (!method.IsPublic && !isICall)
                continue;

            if (isICall && wrappedIcalls.Contains(method.FullName))
                continue;

            MethodDef methodToAnalyze;
            string? overrideName = null;

            if (method.IsPublic && !isICall)
            {
                var targetICall = ICallAnalyzer.FindTargetICall(method);
                if (targetICall is not null)
                {
                    if (ICallAnalyzer.IsSimpleWrapper(method))
                    {
                        methodToAnalyze = targetICall;
                        overrideName = method.Name.String;
                    }
                    else
                    {
                        continue;
                    }
                }
                else
                {
                    methodToAnalyze = method;
                }
            }
            else
            {
                methodToAnalyze = method;
            }

            var il2CppMethod = MethodAnalyzer.Analyze(methodToAnalyze, overrideName);

            var isPropertyGetter = propertyMethodNames.Contains(il2CppMethod.Name) &&
                                   il2CppMethod.Name.StartsWith("Get") &&
                                   il2CppMethod.Parameters.Count == 0;
            var isPropertySetter = propertyMethodNames.Contains(il2CppMethod.Name) &&
                                   il2CppMethod.Name.StartsWith("Set") &&
                                   il2CppMethod.Parameters.Count == 1;
            var isPropertyInjectedICall = propertyMethodNames.Contains(method.Name.String);

            if (isPropertyGetter || isPropertySetter || isPropertyInjectedICall)
                continue;

            il2CppClass.Methods.Add(il2CppMethod);
            TrackMethodTypes(methodToAnalyze, il2CppClass, currentAssemblyName);
        }
    }

    private static void TrackMethodTypes(MethodDef method, Il2CppClass il2CppClass, string currentAssemblyName)
    {
        TypeTracker.TrackReferencedType(method.ReturnType, il2CppClass.ReferencedTypes);
        TypeTracker.TrackExternalType(method.ReturnType, currentAssemblyName, il2CppClass.ExternalTypes);
        foreach (var param in method.Parameters.Where(p => p.IsNormalMethodParameter))
        {
            TypeTracker.TrackReferencedType(param.Type, il2CppClass.ReferencedTypes);
            TypeTracker.TrackExternalType(param.Type, currentAssemblyName, il2CppClass.ExternalTypes);
        }
    }

    private static void AnalyzeNestedEnums(TypeDef typeDef, Il2CppClass il2CppClass)
    {
        foreach (var nestedType in typeDef.NestedTypes)
        {
            if (!nestedType.IsNestedPublic && !nestedType.IsNestedAssembly) continue;
            if (!nestedType.IsEnum) continue;

            var nestedEnum = EnumAnalyzer.Analyze(nestedType);
            nestedEnum.IsNested = true;
            nestedEnum.ParentTypeName = il2CppClass.Name;
            il2CppClass.NestedEnums.Add(nestedEnum);
        }
    }

    private static void AnalyzeFields(TypeDef typeDef, Il2CppClass il2CppClass, string currentAssemblyName)
    {
        foreach (var field in typeDef.Fields)
        {
            if (field.IsStatic) continue;
            if (field.Name.String == "value__") continue;

            if (il2CppClass.IsValueType)
            {
                var il2CppField = FieldAnalyzer.Analyze(field);
                il2CppClass.Fields.Add(il2CppField);
            }
            else
            {
                if (!field.IsPublic) continue;
                if (field.IsSpecialName) continue;

                var il2CppField = FieldAnalyzer.Analyze(field);
                il2CppClass.Fields.Add(il2CppField);
            }

            TypeTracker.TrackReferencedType(field.FieldType, il2CppClass.ReferencedTypes);
            TypeTracker.TrackExternalType(field.FieldType, currentAssemblyName, il2CppClass.ExternalTypes);
        }
    }

    private static string? GetBaseTypeName(TypeDef typeDef)
    {
        if (typeDef.BaseType is null)
            return null;

        var baseFullName = typeDef.BaseType.FullName;

        if (baseFullName is "System.Object" or "System.ValueType" or "System.Enum")
            return null;

        if (typeDef.BaseType.TryGetGenericInstSig() is { } genericSig)
        {
            var baseName = genericSig.GenericType?.TypeName ?? "";
            var backtickIndex = baseName.IndexOf('`');
            if (backtickIndex > 0)
                baseName = baseName[..backtickIndex];

            var args = genericSig.GenericArguments
                .Select(a => RustTypeMapper.Map(a))
                .ToArray();

            return $"{baseName}<{string.Join(", ", args)}>";
        }

        var name = typeDef.BaseType.Name.String;
        var idx = name.IndexOf('`');
        if (idx > 0)
            name = name[..idx];

        return name;
    }

    private static string GetEffectiveNamespace(TypeDef typeDef)
    {
        if (!string.IsNullOrEmpty(typeDef.Namespace?.String))
            return typeDef.Namespace.String;

        if (typeDef is { IsNested: true, DeclaringType: not null })
            return GetEffectiveNamespace(typeDef.DeclaringType);

        return "";
    }
}