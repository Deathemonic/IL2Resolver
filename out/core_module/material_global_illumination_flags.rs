#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaterialGlobalIlluminationFlags {
    #[default]
    None = 0,
    RealtimeEmissive = 1,
    BakedEmissive = 2,
    EmissiveIsBlack = 4,
    AnyEmissive = 3,
}
