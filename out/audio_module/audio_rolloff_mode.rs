#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioRolloffMode {
    #[default]
    Logarithmic = 0,
    Linear = 1,
    Custom = 2,
}
