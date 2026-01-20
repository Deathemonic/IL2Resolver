#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureSubElement {
    #[default]
    Color = 0,
    Depth = 1,
    Stencil = 2,
    Default = 3,
}
