#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalWrapMode {
    #[default]
    Wrap = 0,
    Overflow = 1,
}
