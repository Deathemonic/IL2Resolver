#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasUpdate {
    #[default]
    Prelayout = 0,
    Layout = 1,
    PostLayout = 2,
    PreRender = 3,
    LatePreRender = 4,
    MaxUpdateValue = 5,
}
