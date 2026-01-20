#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::RectOffset;
use crate::text_rendering_module::TextAnchor;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "LayoutGroup", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct LayoutGroup(pub *mut c_void);

#[unity_impl]
impl LayoutGroup {
    #[unity_method(name = "get_padding")]
    pub fn get_padding(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_padding")]
    pub fn set_padding(&self, value: Option<RectOffset>) {}

    #[unity_method(name = "get_childAlignment")]
    pub fn get_child_alignment(&self) -> TextAnchor {}

    #[unity_method(name = "set_childAlignment")]
    pub fn set_child_alignment(&self, value: TextAnchor) {}

    #[unity_method(name = "get_minWidth")]
    pub fn get_min_width(&self) -> f32 {}

    #[unity_method(name = "get_preferredWidth")]
    pub fn get_preferred_width(&self) -> f32 {}

    #[unity_method(name = "get_flexibleWidth")]
    pub fn get_flexible_width(&self) -> f32 {}

    #[unity_method(name = "get_minHeight")]
    pub fn get_min_height(&self) -> f32 {}

    #[unity_method(name = "get_preferredHeight")]
    pub fn get_preferred_height(&self) -> f32 {}

    #[unity_method(name = "get_flexibleHeight")]
    pub fn get_flexible_height(&self) -> f32 {}

    #[unity_method(name = "get_layoutPriority")]
    pub fn get_layout_priority(&self) -> i32 {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

}
