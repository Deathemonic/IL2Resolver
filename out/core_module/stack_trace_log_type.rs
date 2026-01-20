#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackTraceLogType {
    #[default]
    None = 0,
    ScriptOnly = 1,
    Full = 2,
}
