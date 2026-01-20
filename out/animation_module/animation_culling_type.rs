#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationCullingType {
    #[default]
    AlwaysAnimate = 0,
    BasedOnRenderers = 1,
    BasedOnClipBounds = 2,
    BasedOnUserBounds = 3,
}
