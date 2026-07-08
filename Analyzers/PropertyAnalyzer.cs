using dnlib.DotNet;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;

namespace IL2Resolver.Analyzers;

public static class PropertyAnalyzer
{
    public static Il2CppProperty Analyze(PropertyDef propDef)
    {
        var getterIsICall = propDef.GetMethod is not null && ICallAnalyzer.IsICall(propDef.GetMethod);
        var setterIsICall = propDef.SetMethod is not null && ICallAnalyzer.IsICall(propDef.SetMethod);

        var (getterInjectedName, getterInjectedParams) = GetInjectedICallInfo(propDef.GetMethod);
        var (setterInjectedName, setterInjectedParams) = GetInjectedICallInfo(propDef.SetMethod);

        var getterWrapperInfo = GetWrapperInfo(propDef.GetMethod, getterIsICall);
        var setterWrapperInfo = GetWrapperInfo(propDef.SetMethod, setterIsICall);

        return new Il2CppProperty
        {
            Name = propDef.Name.String,
            IlName = propDef.Name.String,
            Type = RustTypeMapper.Map(propDef.PropertySig?.RetType),
            CSharpType = GetSimpleTypeName(propDef.PropertySig?.RetType),
            HasGetter = propDef.GetMethod is not null,
            HasSetter = propDef.SetMethod is not null,
            IsStatic = propDef.GetMethod?.IsStatic ?? propDef.SetMethod?.IsStatic ?? false,
            GetterIsICall = getterIsICall,
            SetterIsICall = setterIsICall,
            GetterInjectedICallName = getterInjectedName,
            GetterInjectedParams = getterInjectedParams,
            SetterInjectedICallName = setterInjectedName,
            SetterInjectedParams = setterInjectedParams,
            GetterWrapperInfo = getterWrapperInfo,
            SetterWrapperInfo = setterWrapperInfo
        };
    }

    private static (string? Name, List<string>? Params) GetInjectedICallInfo(MethodDef? method)
    {
        if (method is null || !method.HasBody || ICallAnalyzer.IsICall(method))
            return (null, null);

        var injectedParams = ICallAnalyzer.GetInjectedICallParams(method);
        if (injectedParams is null)
            return (null, null);

        var (wrappedName, _) = ICallAnalyzer.GetWrappedICallInfo(method);
        return (wrappedName, injectedParams);
    }

    private static WrapperInfo? GetWrapperInfo(MethodDef? method, bool isICall)
    {
        if (method is null || isICall)
            return null;

        return ICallAnalyzer.AnalyzeWrapperChain(method);
    }

    private static string GetSimpleTypeName(TypeSig? typeSig)
    {
        if (typeSig is null)
            return "void";

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
        if (backtick > 0)
            typeName = typeName[..backtick];

        return typeName;
    }

    public static bool IsPublic(PropertyDef property) =>
        (property.GetMethod?.IsPublic ?? false) || (property.SetMethod?.IsPublic ?? false);
}