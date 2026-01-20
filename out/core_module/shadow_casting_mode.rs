#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowCastingMode {
    #[default]
    Off = 0,
    On = 1,
    TwoSided = 2,
    ShadowsOnly = 3,
}
