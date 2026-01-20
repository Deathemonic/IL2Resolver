#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureWrapMode {
    #[default]
    Repeat = 0,
    Clamp = 1,
    Mirror = 2,
    MirrorOnce = 3,
}
