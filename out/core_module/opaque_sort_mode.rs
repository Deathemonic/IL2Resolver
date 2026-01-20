#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpaqueSortMode {
    #[default]
    Default = 0,
    FrontToBack = 1,
    NoDistanceSort = 2,
}
