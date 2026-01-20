#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector4};
use crate::core_module::{Camera, Rect, RectTransform, Vector2Int};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "RectMask2D", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct RectMask2D(pub *mut c_void);

#[unity_impl]
impl RectMask2D {
    #[unity_method(name = "get_padding")]
    pub fn get_padding(&self) -> Vector4 {}

    #[unity_method(name = "set_padding")]
    pub fn set_padding(&self, value: Vector4) {}

    #[unity_method(name = "get_softness")]
    pub fn get_softness(&self) -> Vector2Int {}

    #[unity_method(name = "set_softness")]
    pub fn set_softness(&self, value: Vector2Int) {}

    #[unity_method(name = "get_canvasRect")]
    pub fn get_canvas_rect(&self) -> Rect {}

    #[unity_method(name = "get_rectTransform")]
    pub fn get_rect_transform(&self) -> Option<RectTransform> {}

    #[unity_method(name = "IsRaycastLocationValid")]
    pub fn is_raycast_location_valid(&self, sp: Vector2, event_camera: Option<Camera>) -> bool {}

    #[unity_method(name = "PerformClipping")]
    pub fn perform_clipping(&self) {}

    #[unity_method(name = "UpdateClipSoftness")]
    pub fn update_clip_softness(&self) {}

    #[unity_method(name = "AddClippable")]
    pub fn add_clippable(&self, clippable: *mut c_void) {}

    #[unity_method(name = "RemoveClippable")]
    pub fn remove_clippable(&self, clippable: *mut c_void) {}

}
