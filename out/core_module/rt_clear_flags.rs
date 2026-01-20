#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RTClearFlags {
    #[default]
    None = 0,
    Color = 1,
    Depth = 2,
    Stencil = 4,
    All = 7,
    DepthStencil = 6,
    ColorDepth = 3,
    ColorStencil = 5,
}
