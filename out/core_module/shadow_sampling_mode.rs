#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowSamplingMode {
    #[default]
    CompareDepths = 0,
    RawDepth = 1,
    None = 2,
}
