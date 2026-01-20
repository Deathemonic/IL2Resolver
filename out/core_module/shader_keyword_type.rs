#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderKeywordType {
    #[default]
    None = 0,
    BuiltinDefault = 2,
    BuiltinExtra = 6,
    BuiltinAutoStripped = 10,
    UserDefined = 16,
    Plugin = 32,
}
