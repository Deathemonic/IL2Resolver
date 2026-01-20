#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrimitiveType {
    #[default]
    Sphere = 0,
    Capsule = 1,
    Cylinder = 2,
    Cube = 3,
    Plane = 4,
    Quad = 5,
}
