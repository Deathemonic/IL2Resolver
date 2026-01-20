#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshTopology {
    #[default]
    Triangles = 0,
    Quads = 2,
    Lines = 3,
    LineStrip = 4,
    Points = 5,
}
