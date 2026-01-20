#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpritePackingRotation {
    #[default]
    None = 0,
    FlipHorizontal = 1,
    FlipVertical = 2,
    Rotate180 = 3,
    Any = 15,
}
