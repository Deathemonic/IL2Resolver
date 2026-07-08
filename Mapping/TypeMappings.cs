using System.Collections.Frozen;

namespace IL2Resolver.Mapping;

public static class TypeMappings
{
    public static readonly FrozenDictionary<string, string> Primitives = new Dictionary<string, string>
    {
        ["System.Void"] = "()",
        ["System.Boolean"] = "bool",
        ["System.Byte"] = "u8",
        ["System.SByte"] = "i8",
        ["System.Int16"] = "i16",
        ["System.UInt16"] = "u16",
        ["System.Int32"] = "i32",
        ["System.UInt32"] = "u32",
        ["System.Int64"] = "i64",
        ["System.UInt64"] = "u64",
        ["System.Single"] = "f32",
        ["System.Double"] = "f64",
        ["System.Char"] = "u16",
        ["System.IntPtr"] = "*mut c_void",
        ["System.UIntPtr"] = "*mut c_void"
    }.ToFrozenDictionary();

    public static readonly FrozenDictionary<string, string> UnityMath = new Dictionary<string, string>
    {
        ["UnityEngine.Vector2"] = "Vector2",
        ["UnityEngine.Vector3"] = "Vector3",
        ["UnityEngine.Vector4"] = "Vector4",
        ["UnityEngine.Quaternion"] = "Quaternion",
        ["UnityEngine.Matrix4x4"] = "Matrix4x4"
    }.ToFrozenDictionary();

    public static readonly FrozenDictionary<string, string> SystemTypes = new Dictionary<string, string>
    {
        ["System.String"] = "SystemString",
        ["System.Object"] = "SystemObject",
        ["System.Type"] = "SystemType",
        ["System.Array"] = "SystemArray",
        ["System.IO.Stream"] = "Stream"
    }.ToFrozenDictionary();

    public static readonly FrozenSet<string> SystemInterfaces = new HashSet<string>
    {
        "IAsyncResult",
        "IDisposable",
        "IEnumerable",
        "IEnumerator",
        "IComparable",
        "ICloneable",
        "IList",
        "IFormattable",
        "IFormatProvider",
        "IServiceProvider",
        "IConvertible",
        "AsyncCallback"
    }.ToFrozenSet();

    public static readonly FrozenDictionary<string, string> EnumUnderlyingTypes = new Dictionary<string, string>
    {
        ["int32_t"] = "i32",
        ["int"] = "i32",
        ["i32"] = "i32",
        ["uint32_t"] = "u32",
        ["uint"] = "u32",
        ["u32"] = "u32",
        ["int64_t"] = "i64",
        ["long"] = "i64",
        ["i64"] = "i64",
        ["uint64_t"] = "u64",
        ["ulong"] = "u64",
        ["u64"] = "u64",
        ["int16_t"] = "i16",
        ["short"] = "i16",
        ["i16"] = "i16",
        ["uint16_t"] = "u16",
        ["ushort"] = "u16",
        ["u16"] = "u16",
        ["int8_t"] = "i8",
        ["sbyte"] = "i8",
        ["i8"] = "i8",
        ["uint8_t"] = "u8",
        ["byte"] = "u8",
        ["u8"] = "u8"
    }.ToFrozenDictionary();

    public static string GetEnumUnderlyingType(string cppType) => EnumUnderlyingTypes.GetValueOrDefault(cppType, "i32");

    public static readonly FrozenDictionary<(string Type, string Method), string> DefaultMethods =
        new Dictionary<(string, string), string>
        {
            [("Vector2", "get_zero")] = "Vector2::ZERO",
            [("Vector3", "get_zero")] = "Vector3::ZERO",
            [("Vector4", "get_zero")] = "Vector4::ZERO",
            [("Quaternion", "get_identity")] = "Quaternion::IDENTITY"
        }.ToFrozenDictionary();

    public static readonly FrozenDictionary<(string Type, string Field), string> DefaultFields =
        new Dictionary<(string, string), string>
        {
            [("Vector2", "zero")] = "Vector2::ZERO",
            [("Vector3", "zero")] = "Vector3::ZERO",
            [("Vector4", "zero")] = "Vector4::ZERO"
        }.ToFrozenDictionary();

    public static string? GetDefaultMethod(string typeName, string methodName) =>
        DefaultMethods.GetValueOrDefault((typeName, methodName));

    public static string? GetDefaultField(string typeName, string fieldName) =>
        DefaultFields.GetValueOrDefault((typeName, fieldName));

    public static string FormatFloat(float value) =>
        float.IsNaN(value) ? "f32::NAN" :
        float.IsPositiveInfinity(value) ? "f32::INFINITY" :
        float.IsNegativeInfinity(value) ? "f32::NEG_INFINITY" :
        $"{value}f32";

    public static string FormatDouble(double value) =>
        double.IsNaN(value) ? "f64::NAN" :
        double.IsPositiveInfinity(value) ? "f64::INFINITY" :
        double.IsNegativeInfinity(value) ? "f64::NEG_INFINITY" :
        $"{value}f64";

    public static string GetRustPrimitive(string csharpType, IReadOnlySet<string>? valueTypes = null, IReadOnlySet<string>? enumTypes = null)
    {
        if (csharpType.EndsWith("*"))
        {
            var innerType = csharpType[..^1];
            var mappedInner = GetRustPrimitive(innerType, valueTypes, enumTypes);
            if (mappedInner == "()")
                return "*mut c_void";
            return $"*mut {mappedInner}";
        }

        var shortName = csharpType.Split('.').Last();

        if (valueTypes is not null)
        {
            if (valueTypes.Contains(csharpType) || valueTypes.Any(v => v.EndsWith($".{shortName}")))
                return shortName;
        }

        if (enumTypes is not null)
        {
            if (enumTypes.Contains(csharpType) || enumTypes.Any(v => v.EndsWith($".{shortName}")))
                return shortName;
        }

        if (UnityMath.ContainsKey(csharpType))
            return UnityMath[csharpType];

        return csharpType switch
        {
            "System.Void" => "()",
            "System.Boolean" => "bool",
            "System.Byte" => "u8",
            "System.SByte" => "i8",
            "System.Int16" => "i16",
            "System.UInt16" => "u16",
            "System.Int32" => "i32",
            "System.UInt32" => "u32",
            "System.Int64" => "i64",
            "System.UInt64" => "u64",
            "System.Single" => "f32",
            "System.Double" => "f64",
            "System.String" => "&str",
            "System.Object" => "Option<SystemObject>",
            "System.Type" => "Option<SystemType>",
            "System.Array" => "Option<SystemArray>",
            "System.IntPtr" => "*mut c_void",
            "System.UIntPtr" => "*mut c_void",
            _ => $"Option<{csharpType.Split('.').Last()}>"
        };
    }
}