#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject};
use crate::core_module::Color;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "ColorBlock", namespace = "UnityEngine.UI", value_type)]
pub struct ColorBlock {
    pub m_normal_color: Color,
    pub m_highlighted_color: Color,
    pub m_pressed_color: Color,
    pub m_selected_color: Color,
    pub m_disabled_color: Color,
    pub m_color_multiplier: f32,
    pub m_fade_duration: f32,
}

#[unity_impl]
impl ColorBlock {
    #[unity_method(name = "get_normalColor")]
    pub fn get_normal_color(&self) -> Color {}

    #[unity_method(name = "set_normalColor")]
    pub fn set_normal_color(&self, value: Color) {}

    #[unity_method(name = "get_highlightedColor")]
    pub fn get_highlighted_color(&self) -> Color {}

    #[unity_method(name = "set_highlightedColor")]
    pub fn set_highlighted_color(&self, value: Color) {}

    #[unity_method(name = "get_pressedColor")]
    pub fn get_pressed_color(&self) -> Color {}

    #[unity_method(name = "set_pressedColor")]
    pub fn set_pressed_color(&self, value: Color) {}

    #[unity_method(name = "get_selectedColor")]
    pub fn get_selected_color(&self) -> Color {}

    #[unity_method(name = "set_selectedColor")]
    pub fn set_selected_color(&self, value: Color) {}

    #[unity_method(name = "get_disabledColor")]
    pub fn get_disabled_color(&self) -> Color {}

    #[unity_method(name = "set_disabledColor")]
    pub fn set_disabled_color(&self, value: Color) {}

    #[unity_method(name = "get_colorMultiplier")]
    pub fn get_color_multiplier(&self) -> f32 {}

    #[unity_method(name = "set_colorMultiplier")]
    pub fn set_color_multiplier(&self, value: f32) {}

    #[unity_method(name = "get_fadeDuration")]
    pub fn get_fade_duration(&self) -> f32 {}

    #[unity_method(name = "set_fadeDuration")]
    pub fn set_fade_duration(&self, value: f32) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: ColorBlock) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
