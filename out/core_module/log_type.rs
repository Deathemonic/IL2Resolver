#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogType {
    #[default]
    Error = 0,
    Assert = 1,
    Warning = 2,
    Log = 3,
    Exception = 4,
}
