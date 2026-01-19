using dnlib.DotNet;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;

namespace IL2Resolver.Analyzers;

public static class EnumAnalyzer
{
    public static Il2CppEnum Analyze(TypeDef typeDef)
    {
        if (!typeDef.IsEnum)
            throw new ArgumentException($"Type {typeDef.FullName} is not an enum", nameof(typeDef));

        var il2CppEnum = new Il2CppEnum
        {
            Name = typeDef.Name.String,
            Namespace = GetEffectiveNamespace(typeDef),
            UnderlyingType = GetUnderlyingRustType(typeDef),
            IsNested = typeDef.IsNested,
            ParentTypeName = typeDef.DeclaringType?.Name.String
        };

        foreach (var field in typeDef.Fields.Where(f => f.HasConstant))
            il2CppEnum.Values.Add(new Il2CppEnumValue
            {
                Name = field.Name.String,
                Value = Convert.ToInt64(field.Constant.Value)
            });

        return il2CppEnum;
    }

    private static string GetUnderlyingRustType(TypeDef typeDef)
    {
        var underlyingField = typeDef.Fields.FirstOrDefault(f => f.Name.String == "value__");
        return underlyingField is null
            ? "i32"
            : TypeMappings.Primitives.GetValueOrDefault(underlyingField.FieldType.FullName, "i32");
    }

    private static string GetEffectiveNamespace(TypeDef typeDef)
    {
        if (!string.IsNullOrEmpty(typeDef.Namespace?.String))
            return typeDef.Namespace.String;

        if (typeDef.IsNested && typeDef.DeclaringType is not null)
            return GetEffectiveNamespace(typeDef.DeclaringType);

        return "";
    }
}