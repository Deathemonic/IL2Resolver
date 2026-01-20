#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotionVectorGenerationMode {
    #[default]
    Camera = 0,
    Object = 1,
    ForceNoMotion = 2,
}
