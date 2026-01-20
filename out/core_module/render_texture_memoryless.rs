#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureMemoryless {
    #[default]
    None = 0,
    Color = 1,
    Depth = 2,
    MSAA = 4,
}
