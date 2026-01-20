#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ForceMode {
    #[default]
    Force = 0,
    Acceleration = 5,
    Impulse = 1,
    VelocityChange = 2,
}
