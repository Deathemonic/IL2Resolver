#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperatingSystemFamily {
    #[default]
    Other = 0,
    MacOSX = 1,
    Windows = 2,
    Linux = 3,
}
