#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::cull_state_changed_event::CullStateChangedEvent;
use crate::core_module::{Material, Rect};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Graphic, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "MaskableGraphic", namespace = "UnityEngine.UI", inherit = "Graphic,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct MaskableGraphic(pub *mut c_void);

#[unity_impl]
impl MaskableGraphic {
    #[unity_method(name = "get_onCullStateChanged")]
    pub fn get_on_cull_state_changed(&self) -> Option<CullStateChangedEvent> {}

    #[unity_method(name = "set_onCullStateChanged")]
    pub fn set_on_cull_state_changed(&self, value: Option<CullStateChangedEvent>) {}

    #[unity_method(name = "get_maskable")]
    pub fn get_maskable(&self) -> bool {}

    #[unity_method(name = "set_maskable")]
    pub fn set_maskable(&self, value: bool) {}

    #[unity_method(name = "get_isMaskingGraphic")]
    pub fn get_is_masking_graphic(&self) -> bool {}

    #[unity_method(name = "set_isMaskingGraphic")]
    pub fn set_is_masking_graphic(&self, value: bool) {}

    #[unity_method(name = "GetModifiedMaterial")]
    pub fn get_modified_material(&self, base_material: Option<Material>) -> Option<Material> {}

    #[unity_method(name = "Cull")]
    pub fn cull(&self, clip_rect: Rect, valid_rect: bool) {}

    #[unity_method(name = "SetClipRect")]
    pub fn set_clip_rect(&self, clip_rect: Rect, valid_rect: bool) {}

    #[unity_method(name = "SetClipSoftness")]
    pub fn set_clip_softness(&self, clip_softness: Vector2) {}

    #[unity_method(name = "ParentMaskStateChanged")]
    pub fn parent_mask_state_changed(&self) {}

    #[unity_method(name = "RecalculateClipping")]
    pub fn recalculate_clipping(&self) {}

    #[unity_method(name = "RecalculateMasking")]
    pub fn recalculate_masking(&self) {}

}
