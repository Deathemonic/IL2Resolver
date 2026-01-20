#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingThreadingMode {
    #[default]
    Direct = 0,
    SingleThreaded = 1,
    MultiThreaded = 2,
    LegacyJobified = 3,
    NativeGraphicsJobs = 4,
    NativeGraphicsJobsWithoutRenderThread = 5,
}
