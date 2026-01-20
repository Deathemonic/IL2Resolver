#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidbodyConstraints {
    #[default]
    None = 0,
    FreezePositionX = 2,
    FreezePositionY = 4,
    FreezePositionZ = 8,
    FreezeRotationX = 16,
    FreezeRotationY = 32,
    FreezeRotationZ = 64,
    FreezePosition = 14,
    FreezeRotation = 112,
    FreezeAll = 126,
}
