#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusType {
    #[default]
    Native = 0,
    Keyboard = 1,
    Passive = 2,
}
