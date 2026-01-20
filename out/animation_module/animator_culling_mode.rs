#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatorCullingMode {
    #[default]
    AlwaysAnimate = 0,
    CullUpdateTransforms = 1,
    CullCompletely = 2,
}
