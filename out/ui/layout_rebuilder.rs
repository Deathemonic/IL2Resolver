#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::canvas_update::CanvasUpdate;
use crate::core_module::{RectTransform, Transform};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "LayoutRebuilder", namespace = "UnityEngine.UI")]
pub struct LayoutRebuilder(pub *mut c_void);

#[unity_impl]
impl LayoutRebuilder {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_transform")]
    pub fn get_transform(&self) -> Option<Transform> {}

    #[unity_method(name = "IsDestroyed")]
    pub fn is_destroyed(&self) -> bool {}

    #[unity_method(name = "ForceRebuildLayoutImmediate", static)]
    pub fn force_rebuild_layout_immediate(layout_root: Option<RectTransform>) {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, executing: CanvasUpdate) {}

    #[unity_method(name = "MarkLayoutForRebuild", static)]
    pub fn mark_layout_for_rebuild(rect: Option<RectTransform>) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
