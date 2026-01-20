#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RayTracingSubMeshFlags {
    #[default]
    Disabled = 0,
    Enabled = 1,
    ClosestHitOnly = 2,
    UniqueAnyHitCalls = 4,
}
