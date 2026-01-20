#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingPath {
    UsePlayerSettings = -1,
    #[default]
    VertexLit = 0,
    Forward = 1,
    DeferredLighting = 2,
    DeferredShading = 3,
}
