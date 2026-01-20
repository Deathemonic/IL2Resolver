#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionType {
    #[default]
    None = 0,
    Lzma = 1,
    Lz4 = 2,
    Lz4HC = 3,
}
