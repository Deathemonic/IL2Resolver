#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollisionDetectionMode {
    #[default]
    Discrete = 0,
    Continuous = 1,
    ContinuousDynamic = 2,
    ContinuousSpeculative = 3,
}
