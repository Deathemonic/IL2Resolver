#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkinQuality {
    #[default]
    Auto = 0,
    Bone1 = 1,
    Bone2 = 2,
    Bone4 = 4,
}
