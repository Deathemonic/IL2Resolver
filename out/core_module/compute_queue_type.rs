#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComputeQueueType {
    #[default]
    Default = 0,
    Background = 1,
    Urgent = 2,
}
