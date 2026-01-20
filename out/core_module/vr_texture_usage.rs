#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VRTextureUsage {
    #[default]
    None = 0,
    OneEye = 1,
    TwoEyes = 2,
    DeviceSpecific = 3,
}
