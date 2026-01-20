#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeType {
    #[default]
    Cube = 0,
    Card = 1,
}
