using System.Text;
using CaseConverter;
using IL2Resolver.Context;
using IL2Resolver.Mapping;
using IL2Resolver.Schema;
using IL2Resolver.Utils;

namespace IL2Resolver.Writers;

public static class ImportWriter
{
    public static void Write(StringBuilder sb, Il2CppClass cls, Il2CppSchema schema, WriterContext context)
    {
        var usedRustTypes = CollectUsedTypes(cls);
        var needsCVoid = !cls.IsValueType || usedRustTypes.Any(t => t.Contains("c_void"));

        var mathTypes = new HashSet<string>();
        var systemTypes = new HashSet<string>();
        var localTypes = new HashSet<string>();

        foreach (var rustType in usedRustTypes)
        {
            CollectTypeFromRustType(rustType, mathTypes, systemTypes, localTypes, cls.Name, context);
            if (rustType.Contains("c_void"))
                needsCVoid = true;
        }

        localTypes.ExceptWith(context.NestedEnumNames);

        if (needsCVoid)
            sb.AppendLine("use std::ffi::c_void;");

        sb.AppendLine("use unity_derive::*;");

        if (mathTypes.Count > 0)
            sb.AppendLine($"use crate::math::{{{string.Join(", ", mathTypes.OrderBy(t => t))}}};");

        var inheritanceTypes = cls.InheritanceChain.Select(b => b.Name).ToHashSet();
        localTypes.ExceptWith(inheritanceTypes);

        WriteSystemTypeImports(sb, systemTypes);
        WriteLocalTypeImports(sb, localTypes, cls);
        WriteInheritanceImports(sb, cls, schema);
    }

    private static HashSet<string> CollectUsedTypes(Il2CppClass cls)
    {
        var usedRustTypes = new HashSet<string>();

        foreach (var field in cls.Fields.Where(f => !f.IsStatic))
            usedRustTypes.Add(field.Type);

        foreach (var prop in cls.Properties)
            usedRustTypes.Add(prop.Type);

        foreach (var method in cls.Methods.Where(m => m.GenericParameters.Count == 0 && !m.RequiresTodo))
        {
            usedRustTypes.Add(method.ReturnType);
            foreach (var param in method.Parameters)
            {
                var paramType = param.Type;
                if (param.IsOut || param.IsRef)
                    if (paramType.StartsWith("&mut "))
                        paramType = paramType[5..].Trim();
                usedRustTypes.Add(paramType);
            }
        }

        foreach (var ctor in cls.Constructors)
        foreach (var param in ctor.Parameters)
            usedRustTypes.Add(param.Type);

        return usedRustTypes;
    }

    private static void CollectTypeFromRustType(
        string rustType,
        HashSet<string> mathTypes,
        HashSet<string> systemTypes,
        HashSet<string> localTypes,
        string selfName,
        WriterContext context)
    {
        if (string.IsNullOrEmpty(rustType))
            return;

        if (rustType.StartsWith("Array<") || rustType.StartsWith("List<") ||
            rustType.StartsWith("Dictionary<") || rustType.StartsWith("Nullable<"))
        {
            var collectionName = rustType[..rustType.IndexOf('<')];
            systemTypes.Add(collectionName);

            var start = rustType.IndexOf('<') + 1;
            var end = rustType.LastIndexOf('>');
            if (end <= start) return;
            var innerArgs = TemplateUtils.SplitTemplateArgs(rustType[start..end]);
            foreach (var arg in innerArgs)
                CollectTypeFromRustType(arg.Trim(), mathTypes, systemTypes, localTypes, selfName, context);
            return;
        }

        if (rustType.StartsWith("ValueTuple"))
        {
            var angleBracket = rustType.IndexOf('<');
            if (angleBracket <= 0) return;
            var tupleName = rustType[..angleBracket];
            systemTypes.Add(tupleName);

            var end = rustType.LastIndexOf('>');
            if (end <= angleBracket) return;
            var innerArgs = TemplateUtils.SplitTemplateArgs(rustType[(angleBracket + 1)..end]);
            foreach (var arg in innerArgs)
                CollectTypeFromRustType(arg.Trim(), mathTypes, systemTypes, localTypes, selfName, context);
            return;
        }

        var baseName = TypeNameUtils.ExtractBaseName(rustType);

        if (string.IsNullOrEmpty(baseName) || baseName == selfName)
            return;

        if (TypeCategories.RustPrimitives.Contains(baseName) || baseName is "c_void" or "str" or "String")
            return;

        if (baseName.Contains("::"))
        {
            var parentModule = TypeNameUtils.GetNestedTypeParent(baseName);
            if (parentModule == context.CurrentModuleName)
                return;

            localTypes.Add(baseName);
            return;
        }

        if (TypeCategories.MathTypes.Contains(baseName))
        {
            mathTypes.Add(baseName);
            return;
        }

        if (TypeCategories.SystemWrappers.Contains(baseName) ||
            TypeCategories.ReflectionTypes.Contains(baseName) ||
            TypeCategories.IoTypes.Contains(baseName))
        {
            systemTypes.Add(baseName);
            return;
        }

        if (!char.IsUpper(baseName[0]) || !baseName.All(c => char.IsLetterOrDigit(c) || c == '_')) return;
        if (context.ValidSchemaTypes.Contains(baseName) || context.ExternalTypeNames.Contains(baseName))
            localTypes.Add(baseName);
    }

    private static void WriteSystemTypeImports(StringBuilder sb, HashSet<string> systemTypes)
    {
        if (systemTypes.Count == 0)
            return;

        var collectionTypes = systemTypes.Where(TypeCategories.CollectionTypes.Contains).OrderBy(t => t).ToList();
        var tupleTypes = systemTypes.Where(t => t.StartsWith("ValueTuple")).OrderBy(t => t).ToList();
        var reflectionTypes = systemTypes.Where(TypeCategories.ReflectionTypes.Contains).OrderBy(t => t).ToList();
        var ioTypes = systemTypes.Where(TypeCategories.IoTypes.Contains).OrderBy(t => t).ToList();
        var otherSystemTypes = systemTypes.Where(t =>
                !TypeCategories.CollectionTypes.Contains(t) &&
                !t.StartsWith("ValueTuple") &&
                !TypeCategories.ReflectionTypes.Contains(t) &&
                !TypeCategories.IoTypes.Contains(t))
            .OrderBy(t => t)
            .ToList();

        if (otherSystemTypes.Count > 0)
            sb.AppendLine($"use crate::mscorlib::{{{string.Join(", ", otherSystemTypes)}}};");
        if (collectionTypes.Count > 0)
            sb.AppendLine($"use crate::mscorlib::collections::{{{string.Join(", ", collectionTypes)}}};");
        if (tupleTypes.Count > 0)
            sb.AppendLine($"use crate::mscorlib::tuples::{{{string.Join(", ", tupleTypes)}}};");
        if (reflectionTypes.Count > 0)
            sb.AppendLine($"use crate::mscorlib::reflection::{{{string.Join(", ", reflectionTypes)}}};");
        if (ioTypes.Count > 0)
            sb.AppendLine($"use crate::mscorlib::io::{{{string.Join(", ", ioTypes)}}};");
    }

    private static void WriteLocalTypeImports(StringBuilder sb, HashSet<string> localTypes, Il2CppClass cls)
    {
        var nestedTypeParents = new HashSet<string>();
        var sameModuleTypes = new HashSet<string>();
        var externalTypesByModule = new Dictionary<string, HashSet<string>>();
        var inheritanceTypes = cls.InheritanceChain.Select(b => b.Name).ToHashSet();

        foreach (var typeName in localTypes.Where(typeName =>
                     typeName != cls.Name && !inheritanceTypes.Contains(typeName)))
            if (typeName.Contains("::"))
            {
                var parentModule = TypeNameUtils.GetNestedTypeParent(typeName);
                if (parentModule is not null)
                    nestedTypeParents.Add(parentModule);
            }
            else
            {
                if (inheritanceTypes.Contains(typeName))
                    continue;

                if (cls.ExternalTypes.TryGetValue(typeName, out var externalModule))
                {
                    var rustModuleName = GetRustModuleName(externalModule);
                    if (!externalTypesByModule.ContainsKey(rustModuleName))
                        externalTypesByModule[rustModuleName] = [];
                    externalTypesByModule[rustModuleName].Add(typeName);
                }
                else
                {
                    sameModuleTypes.Add(typeName);
                }
            }

        foreach (var parentModule in nestedTypeParents.OrderBy(t => t))
            sb.AppendLine($"use super::{parentModule};");

        foreach (var typeName in sameModuleTypes.OrderBy(t => t))
        {
            var modName = typeName.ToSnakeCase();
            sb.AppendLine($"use super::{modName}::{typeName};");
        }

        foreach (var moduleName in externalTypesByModule.Keys.OrderBy(m => m))
        {
            var types = externalTypesByModule[moduleName].OrderBy(t => t).ToList();
            if (types.Count == 1)
                sb.AppendLine($"use crate::{moduleName}::{types[0]};");
            else
                sb.AppendLine($"use crate::{moduleName}::{{{string.Join(", ", types)}}};");
        }
    }

    private static string GetRustModuleName(string dllName)
    {
        if (string.IsNullOrEmpty(dllName))
            return "unknown";

        var name = Path.GetFileNameWithoutExtension(dllName);

        if (name is "System.Private.CoreLib" or "mscorlib")
            return "mscorlib";

        if (name.StartsWith("UnityEngine."))
            name = name["UnityEngine.".Length..];

        return name.ToSnakeCase();
    }

    private static void WriteInheritanceImports(StringBuilder sb, Il2CppClass cls, Il2CppSchema schema)
    {
        if (cls.InheritanceChain.Count == 0)
            return;

        var currentModule = schema.DllName;
        var typesByModule = new Dictionary<string, HashSet<string>>();

        foreach (var baseType in cls.InheritanceChain)
        {
            var module = baseType.Module ?? currentModule;
            var rustModule = GetRustModuleName(module);

            if (rustModule == "unknown")
                continue;

            if (!typesByModule.ContainsKey(rustModule))
                typesByModule[rustModule] = [];
            typesByModule[rustModule].Add(baseType.Name);
        }

        foreach (var moduleName in typesByModule.Keys.OrderBy(m => m))
        {
            var types = typesByModule[moduleName].OrderBy(t => t).ToList();
            switch (types.Count)
            {
                case 0:
                    continue;
                case 1:
                    sb.AppendLine($"use crate::{moduleName}::{types[0]};");
                    break;
                default:
                    sb.AppendLine($"use crate::{moduleName}::{{{string.Join(", ", types)}}};");
                    break;
            }
        }
    }
}