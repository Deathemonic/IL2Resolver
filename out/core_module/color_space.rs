#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorSpace {
    Uninitialized = -1,
    #[default]
    Gamma = 0,
    Linear = 1,
}
