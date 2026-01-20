#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionLevel {
    #[default]
    None = 0,
    Fastest = 1,
    Fast = 2,
    Normal = 3,
    High = 4,
    Maximum = 5,
}
