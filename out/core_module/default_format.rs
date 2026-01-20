#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DefaultFormat {
    #[default]
    LDR = 0,
    HDR = 1,
    DepthStencil = 2,
    Shadow = 3,
    Video = 4,
}
