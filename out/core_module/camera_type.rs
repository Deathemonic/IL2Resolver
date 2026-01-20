#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraType {
    #[default]
    Game = 1,
    SceneView = 2,
    Preview = 4,
    VR = 8,
    Reflection = 16,
}
