using System.Collections.Frozen;

namespace IL2Resolver.Mapping;

public static class TypeCategories
{
    public static readonly FrozenSet<string> RustPrimitives = new HashSet<string>
    {
        "()", "bool", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64", "isize", "usize"
    }.ToFrozenSet();

    public static readonly FrozenSet<string> MathTypes =
        TypeMappings.UnityMath.Values.ToFrozenSet();

    public static readonly FrozenSet<string> Copyable = RustPrimitives
        .Concat(MathTypes)
        .ToFrozenSet();

    public static readonly FrozenSet<string> CollectionTypes = new HashSet<string>
    {
        "Array", "List", "Dictionary"
    }.ToFrozenSet();

    public static readonly FrozenSet<string> TupleTypes = new HashSet<string>
    {
        "ValueTuple2", "ValueTuple3", "ValueTuple4"
    }.ToFrozenSet();

    public static readonly FrozenSet<string> ReflectionTypes = new HashSet<string>
    {
        "MethodInfo", "FieldInfo", "PropertyInfo", "Assembly"
    }.ToFrozenSet();

    public static readonly FrozenSet<string> IoTypes = new HashSet<string>
    {
        "Stream", "MemoryStream", "Path", "File", "Directory"
    }.ToFrozenSet();

    public static readonly FrozenSet<string> SystemWrappers = new HashSet<string>
    {
        "SystemString", "SystemObject", "SystemType", "SystemArray",
        "Exception", "Action", "Delegate", "MulticastDelegate"
    }.ToFrozenSet();
}