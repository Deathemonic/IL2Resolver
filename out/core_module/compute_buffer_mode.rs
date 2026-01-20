#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComputeBufferMode {
    #[default]
    Immutable = 0,
    Dynamic = 1,
    Circular = 2,
    StreamOut = 3,
    SubUpdates = 4,
}
