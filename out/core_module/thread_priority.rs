#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThreadPriority {
    #[default]
    Low = 0,
    BelowNormal = 1,
    Normal = 2,
    High = 4,
}
