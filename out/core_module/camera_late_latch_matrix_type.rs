#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraLateLatchMatrixType {
    #[default]
    View = 0,
    InverseView = 1,
    ViewProjection = 2,
    InverseViewProjection = 3,
}
