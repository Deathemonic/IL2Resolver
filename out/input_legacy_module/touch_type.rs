#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TouchType {
    #[default]
    Direct = 0,
    Indirect = 1,
    Stylus = 2,
}
