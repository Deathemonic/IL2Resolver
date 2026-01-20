#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScaleMode {
    #[default]
    StretchToFill = 0,
    ScaleAndCrop = 1,
    ScaleToFit = 2,
}
