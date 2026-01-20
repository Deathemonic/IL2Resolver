#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTargetFlags {
    #[default]
    None = 0,
    ReadOnlyDepth = 1,
    ReadOnlyStencil = 2,
    ReadOnlyDepthStencil = 3,
}
