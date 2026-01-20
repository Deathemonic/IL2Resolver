#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SinglePassStereoMode {
    #[default]
    None = 0,
    SideBySide = 1,
    Instancing = 2,
    Multiview = 3,
}
