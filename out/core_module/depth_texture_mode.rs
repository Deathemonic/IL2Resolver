#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DepthTextureMode {
    #[default]
    None = 0,
    Depth = 1,
    DepthNormals = 2,
    MotionVectors = 4,
}
