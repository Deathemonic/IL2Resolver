#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SynchronisationStageFlags {
    #[default]
    VertexProcessing = 1,
    PixelProcessing = 2,
    ComputeProcessing = 4,
    AllGPUOperations = 7,
}
