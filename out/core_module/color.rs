#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Color", namespace = "UnityEngine", value_type)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[unity_impl]
impl Color {
    #[unity_method(name = "get_red", static)]
    pub fn get_red() -> Color {}

    #[unity_method(name = "get_green", static)]
    pub fn get_green() -> Color {}

    #[unity_method(name = "get_blue", static)]
    pub fn get_blue() -> Color {}

    #[unity_method(name = "get_white", static)]
    pub fn get_white() -> Color {}

    #[unity_method(name = "get_black", static)]
    pub fn get_black() -> Color {}

    #[unity_method(name = "get_yellow", static)]
    pub fn get_yellow() -> Color {}

    #[unity_method(name = "get_cyan", static)]
    pub fn get_cyan() -> Color {}

    #[unity_method(name = "get_magenta", static)]
    pub fn get_magenta() -> Color {}

    #[unity_method(name = "get_gray", static)]
    pub fn get_gray() -> Color {}

    #[unity_method(name = "get_grey", static)]
    pub fn get_grey() -> Color {}

    #[unity_method(name = "get_clear", static)]
    pub fn get_clear() -> Color {}

    #[unity_method(name = "get_grayscale")]
    pub fn get_grayscale(&self) -> f32 {}

    #[unity_method(name = "get_linear")]
    pub fn get_linear(&self) -> Color {}

    #[unity_method(name = "get_gamma")]
    pub fn get_gamma(&self) -> Color {}

    #[unity_method(name = "get_maxColorComponent")]
    pub fn get_max_color_component(&self) -> f32 {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> f32 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: f32) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: Color) -> bool {}

    #[unity_method(name = "Lerp", static)]
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {}

    #[unity_method(name = "LerpUnclamped", static)]
    pub fn lerp_unclamped(a: Color, b: Color, t: f32) -> Color {}

    #[unity_method(name = "RGBToHSV", static)]
    pub fn rgb_to_hsv(rgb_color: Color, h: &mut f32, s: &mut f32, v: &mut f32) {}

    #[unity_method(name = "HSVToRGB", static)]
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {}

    #[unity_method(name = "HSVToRGB", static)]
    pub fn hsv_to_rgb_1(h: f32, s: f32, v: f32, hdr: bool) -> Color {}

}
