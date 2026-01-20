#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "AspectRatioFitter", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct AspectRatioFitter(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AspectMode {
    #[default]
    None = 0,
    WidthControlsHeight = 1,
    HeightControlsWidth = 2,
    FitInParent = 3,
    EnvelopeParent = 4,
}

#[unity_impl]
impl AspectRatioFitter {
    #[unity_method(name = "get_aspectMode")]
    pub fn get_aspect_mode(&self) -> AspectMode {}

    #[unity_method(name = "set_aspectMode")]
    pub fn set_aspect_mode(&self, value: AspectMode) {}

    #[unity_method(name = "get_aspectRatio")]
    pub fn get_aspect_ratio(&self) -> f32 {}

    #[unity_method(name = "set_aspectRatio")]
    pub fn set_aspect_ratio(&self, value: f32) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

    #[unity_method(name = "IsComponentValidOnObject")]
    pub fn is_component_valid_on_object(&self) -> bool {}

    #[unity_method(name = "IsAspectModeValid")]
    pub fn is_aspect_mode_valid(&self) -> bool {}

}
