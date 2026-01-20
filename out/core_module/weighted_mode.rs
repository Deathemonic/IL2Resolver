#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WeightedMode {
    #[default]
    None = 0,
    In = 1,
    Out = 2,
    Both = 3,
}
