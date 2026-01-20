#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::graphic::Graphic;
use crate::core_module::{Camera, Material, RectTransform};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Mask", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Mask(pub *mut c_void);

#[unity_impl]
impl Mask {
    #[unity_method(name = "get_rectTransform")]
    pub fn get_rect_transform(&self) -> Option<RectTransform> {}

    #[unity_method(name = "get_showMaskGraphic")]
    pub fn get_show_mask_graphic(&self) -> bool {}

    #[unity_method(name = "set_showMaskGraphic")]
    pub fn set_show_mask_graphic(&self, value: bool) {}

    #[unity_method(name = "get_graphic")]
    pub fn get_graphic(&self) -> Option<Graphic> {}

    #[unity_method(name = "MaskEnabled")]
    pub fn mask_enabled(&self) -> bool {}

    #[unity_method(name = "OnSiblingGraphicEnabledDisabled")]
    pub fn on_sibling_graphic_enabled_disabled(&self) {}

    #[unity_method(name = "IsRaycastLocationValid")]
    pub fn is_raycast_location_valid(&self, sp: Vector2, event_camera: Option<Camera>) -> bool {}

    #[unity_method(name = "GetModifiedMaterial")]
    pub fn get_modified_material(&self, base_material: Option<Material>) -> Option<Material> {}

}
