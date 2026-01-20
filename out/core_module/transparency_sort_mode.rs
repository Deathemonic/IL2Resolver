#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransparencySortMode {
    #[default]
    Default = 0,
    Perspective = 1,
    Orthographic = 2,
    CustomAxis = 3,
}
