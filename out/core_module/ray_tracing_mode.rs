#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RayTracingMode {
    #[default]
    Off = 0,
    Static = 1,
    DynamicTransform = 2,
    DynamicGeometry = 3,
}
