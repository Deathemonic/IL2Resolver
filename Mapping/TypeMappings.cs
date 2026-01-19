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
        ["System.IntPtr"] = "isize",
        ["System.UIntPtr"] = "usize",
    }.ToFrozenDictionary();

    public static readonly FrozenDictionary<string, string> UnityMath = new Dictionary<string, string>
    {
        ["UnityEngine.Vector2"] = "Vector2",
        ["UnityEngine.Vector3"] = "Vector3",
        ["UnityEngine.Vector4"] = "Vector4",
        ["UnityEngine.Quaternion"] = "Quaternion",
        ["UnityEngine.Matrix4x4"] = "Matrix4x4",
    }.ToFrozenDictionary();

    public static readonly FrozenDictionary<string, string> SystemTypes = new Dictionary<string, string>
    {
        ["System.String"] = "SystemString",
        ["System.Object"] = "SystemObject",
        ["System.Type"] = "SystemType",
        ["System.Array"] = "SystemArray",
        ["System.IO.Stream"] = "Stream",
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
        "AsyncCallback",
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
        ["u8"] = "u8",
    }.ToFrozenDictionary();

    public static string GetEnumUnderlyingType(string cppType) =>
        EnumUnderlyingTypes.GetValueOrDefault(cppType, "i32");
}
