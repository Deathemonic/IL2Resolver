#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderPropertyType {
    #[default]
    Color = 0,
    Vector = 1,
    Float = 2,
    Range = 3,
    Texture = 4,
    Int = 5,
}
