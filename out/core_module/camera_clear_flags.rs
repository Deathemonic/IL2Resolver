#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraClearFlags {
    #[default]
    Skybox = 1,
    Color = 2,
    Depth = 3,
    Nothing = 4,
}
