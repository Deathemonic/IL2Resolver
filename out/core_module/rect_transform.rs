#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector3};
use super::reapply_driven_properties::ReapplyDrivenProperties;
use super::rect::Rect;
use crate::core_module::{Component, Object, Transform};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RectTransform", namespace = "UnityEngine", inherit = "Transform,Component,Object")]
pub struct RectTransform(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Edge {
    #[default]
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    #[default]
    Horizontal = 0,
    Vertical = 1,
}

#[unity_impl]
impl RectTransform {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.RectTransform::get_rect_Injected(Rect&)")]
    pub fn get_rect(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.RectTransform::get_anchorMin_Injected(Vector2&)")]
    pub fn get_anchor_min(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::set_anchorMin_Injected(Vector2&)")]
    pub fn set_anchor_min(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::get_anchorMax_Injected(Vector2&)")]
    pub fn get_anchor_max(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::set_anchorMax_Injected(Vector2&)")]
    pub fn set_anchor_max(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::get_anchoredPosition_Injected(Vector2&)")]
    pub fn get_anchored_position(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::set_anchoredPosition_Injected(Vector2&)")]
    pub fn set_anchored_position(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::get_sizeDelta_Injected(Vector2&)")]
    pub fn get_size_delta(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::set_sizeDelta_Injected(Vector2&)")]
    pub fn set_size_delta(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::get_pivot_Injected(Vector2&)")]
    pub fn get_pivot(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::set_pivot_Injected(Vector2&)")]
    pub fn set_pivot(&self, value: &mut Vector2) {}

    #[unity_method(name = "get_anchoredPosition3D")]
    pub fn get_anchored_position3d(&self) -> Vector3 {}

    #[unity_method(name = "set_anchoredPosition3D")]
    pub fn set_anchored_position3d(&self, value: Vector3) {}

    #[unity_method(name = "get_offsetMin")]
    pub fn get_offset_min(&self) -> Vector2 {}

    #[unity_method(name = "set_offsetMin")]
    pub fn set_offset_min(&self, value: Vector2) {}

    #[unity_method(name = "get_offsetMax")]
    pub fn get_offset_max(&self) -> Vector2 {}

    #[unity_method(name = "set_offsetMax")]
    pub fn set_offset_max(&self, value: Vector2) {}

    #[unity_icall("UnityEngine.RectTransform::get_drivenByObject")]
    pub fn get_driven_by_object(&self) -> Option<Object> {}

    #[unity_icall("UnityEngine.RectTransform::set_drivenByObject(Object)")]
    pub fn set_driven_by_object(&self, value: Option<Object>) {}

    #[unity_method(name = "add_reapplyDrivenProperties", static)]
    pub fn add_reapply_driven_properties(value: Option<ReapplyDrivenProperties>) {}

    #[unity_method(name = "remove_reapplyDrivenProperties", static)]
    pub fn remove_reapply_driven_properties(value: Option<ReapplyDrivenProperties>) {}

    #[unity_icall("UnityEngine.RectTransform::ForceUpdateRectTransforms")]
    pub fn force_update_rect_transforms(&self) {}

}
