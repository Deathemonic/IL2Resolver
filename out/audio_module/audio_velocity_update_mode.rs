#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioVelocityUpdateMode {
    #[default]
    Auto = 0,
    Fixed = 1,
    Dynamic = 2,
}
