#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeUsage {
    #[default]
    Off = 0,
    BlendProbes = 1,
    BlendProbesAndSkybox = 2,
    Simple = 3,
}
