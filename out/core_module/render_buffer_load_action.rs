#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderBufferLoadAction {
    #[default]
    Load = 0,
    Clear = 1,
    DontCare = 2,
}
