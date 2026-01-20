#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextClipping {
    #[default]
    Overflow = 0,
    Clip = 1,
}
