#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "CanvasScaler", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct CanvasScaler(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScaleMode {
    #[default]
    ConstantPixelSize = 0,
    ScaleWithScreenSize = 1,
    ConstantPhysicalSize = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenMatchMode {
    #[default]
    MatchWidthOrHeight = 0,
    Expand = 1,
    Shrink = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Unit {
    #[default]
    Centimeters = 0,
    Millimeters = 1,
    Inches = 2,
    Points = 3,
    Picas = 4,
}

#[unity_impl]
impl CanvasScaler {
    #[unity_method(name = "get_uiScaleMode")]
    pub fn get_ui_scale_mode(&self) -> ScaleMode {}

    #[unity_method(name = "set_uiScaleMode")]
    pub fn set_ui_scale_mode(&self, value: ScaleMode) {}

    #[unity_method(name = "get_referencePixelsPerUnit")]
    pub fn get_reference_pixels_per_unit(&self) -> f32 {}

    #[unity_method(name = "set_referencePixelsPerUnit")]
    pub fn set_reference_pixels_per_unit(&self, value: f32) {}

    #[unity_method(name = "get_scaleFactor")]
    pub fn get_scale_factor(&self) -> f32 {}

    #[unity_method(name = "set_scaleFactor")]
    pub fn set_scale_factor(&self, value: f32) {}

    #[unity_method(name = "get_referenceResolution")]
    pub fn get_reference_resolution(&self) -> Vector2 {}

    #[unity_method(name = "set_referenceResolution")]
    pub fn set_reference_resolution(&self, value: Vector2) {}

    #[unity_method(name = "get_screenMatchMode")]
    pub fn get_screen_match_mode(&self) -> ScreenMatchMode {}

    #[unity_method(name = "set_screenMatchMode")]
    pub fn set_screen_match_mode(&self, value: ScreenMatchMode) {}

    #[unity_method(name = "get_matchWidthOrHeight")]
    pub fn get_match_width_or_height(&self) -> f32 {}

    #[unity_method(name = "set_matchWidthOrHeight")]
    pub fn set_match_width_or_height(&self, value: f32) {}

    #[unity_method(name = "get_physicalUnit")]
    pub fn get_physical_unit(&self) -> Unit {}

    #[unity_method(name = "set_physicalUnit")]
    pub fn set_physical_unit(&self, value: Unit) {}

    #[unity_method(name = "get_fallbackScreenDPI")]
    pub fn get_fallback_screen_dpi(&self) -> f32 {}

    #[unity_method(name = "set_fallbackScreenDPI")]
    pub fn set_fallback_screen_dpi(&self, value: f32) {}

    #[unity_method(name = "get_defaultSpriteDPI")]
    pub fn get_default_sprite_dpi(&self) -> f32 {}

    #[unity_method(name = "set_defaultSpriteDPI")]
    pub fn set_default_sprite_dpi(&self, value: f32) {}

    #[unity_method(name = "get_dynamicPixelsPerUnit")]
    pub fn get_dynamic_pixels_per_unit(&self) -> f32 {}

    #[unity_method(name = "set_dynamicPixelsPerUnit")]
    pub fn set_dynamic_pixels_per_unit(&self, value: f32) {}

}
