#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComputeBufferType {
    #[default]
    Default = 0,
    Raw = 1,
    Append = 2,
    Counter = 4,
    Constant = 8,
    Structured = 16,
    DrawIndirect = 256,
    GPUMemory = 512,
}
