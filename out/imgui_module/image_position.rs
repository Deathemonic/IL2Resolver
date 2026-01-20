#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImagePosition {
    #[default]
    ImageLeft = 0,
    ImageAbove = 1,
    ImageOnly = 2,
    TextOnly = 3,
}
