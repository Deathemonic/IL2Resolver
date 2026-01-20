#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HDRDisplaySupportFlags {
    #[default]
    None = 0,
    Supported = 1,
    RuntimeSwitchable = 2,
    AutomaticTonemapping = 4,
}
