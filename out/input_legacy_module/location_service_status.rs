#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LocationServiceStatus {
    #[default]
    Stopped = 0,
    Initializing = 1,
    Running = 2,
    Failed = 3,
}
