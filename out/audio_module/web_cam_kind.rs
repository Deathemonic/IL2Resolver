#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebCamKind {
    #[default]
    WideAngle = 1,
    Telephoto = 2,
    ColorAndDepth = 3,
    UltraWideAngle = 4,
}
