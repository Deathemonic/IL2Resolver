#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    #[default]
    None = 0,
    X = 1,
    Y = 2,
    Z = 4,
}
