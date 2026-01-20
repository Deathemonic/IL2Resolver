#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "LayoutElement", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct LayoutElement(pub *mut c_void);

#[unity_impl]
impl LayoutElement {
    #[unity_method(name = "get_ignoreLayout")]
    pub fn get_ignore_layout(&self) -> bool {}

    #[unity_method(name = "set_ignoreLayout")]
    pub fn set_ignore_layout(&self, value: bool) {}

    #[unity_method(name = "get_minWidth")]
    pub fn get_min_width(&self) -> f32 {}

    #[unity_method(name = "set_minWidth")]
    pub fn set_min_width(&self, value: f32) {}

    #[unity_method(name = "get_minHeight")]
    pub fn get_min_height(&self) -> f32 {}

    #[unity_method(name = "set_minHeight")]
    pub fn set_min_height(&self, value: f32) {}

    #[unity_method(name = "get_preferredWidth")]
    pub fn get_preferred_width(&self) -> f32 {}

    #[unity_method(name = "set_preferredWidth")]
    pub fn set_preferred_width(&self, value: f32) {}

    #[unity_method(name = "get_preferredHeight")]
    pub fn get_preferred_height(&self) -> f32 {}

    #[unity_method(name = "set_preferredHeight")]
    pub fn set_preferred_height(&self, value: f32) {}

    #[unity_method(name = "get_flexibleWidth")]
    pub fn get_flexible_width(&self) -> f32 {}

    #[unity_method(name = "set_flexibleWidth")]
    pub fn set_flexible_width(&self, value: f32) {}

    #[unity_method(name = "get_flexibleHeight")]
    pub fn get_flexible_height(&self) -> f32 {}

    #[unity_method(name = "set_flexibleHeight")]
    pub fn set_flexible_height(&self, value: f32) {}

    #[unity_method(name = "get_layoutPriority")]
    pub fn get_layout_priority(&self) -> i32 {}

    #[unity_method(name = "set_layoutPriority")]
    pub fn set_layout_priority(&self, value: i32) {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

}
