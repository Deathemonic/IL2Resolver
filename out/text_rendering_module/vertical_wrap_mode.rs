#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalWrapMode {
    #[default]
    Truncate = 0,
    Overflow = 1,
}
