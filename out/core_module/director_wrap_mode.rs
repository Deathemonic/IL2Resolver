#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DirectorWrapMode {
    #[default]
    Hold = 0,
    Loop = 1,
    None = 2,
}
