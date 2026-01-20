#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureDimension {
    Unknown = -1,
    #[default]
    None = 0,
    Any = 1,
    Tex2D = 2,
    Tex3D = 3,
    Cube = 4,
    Tex2DArray = 5,
    CubeArray = 6,
}
