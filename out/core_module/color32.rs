#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Color32", namespace = "UnityEngine", value_type)]
pub struct Color32 {
    pub rgba: i32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[unity_impl]
impl Color32 {
    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> u8 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: u8) {}

    #[unity_method(name = "Lerp", static)]
    pub fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {}

    #[unity_method(name = "LerpUnclamped", static)]
    pub fn lerp_unclamped(a: Color32, b: Color32, t: f32) -> Color32 {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

}
