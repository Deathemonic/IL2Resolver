#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CullingOptions {
    #[default]
    None = 0,
    ForceEvenIfCameraIsNotActive = 1,
    OcclusionCull = 2,
    NeedsLighting = 4,
    NeedsReflectionProbes = 8,
    Stereo = 16,
    DisablePerObjectCulling = 32,
    ShadowCasters = 64,
}
