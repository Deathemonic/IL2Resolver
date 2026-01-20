#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::RectTransform;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "LayoutUtility", namespace = "UnityEngine.UI")]
pub struct LayoutUtility(pub *mut c_void);

#[unity_impl]
impl LayoutUtility {
    #[unity_method(name = "GetMinSize", static)]
    pub fn get_min_size(rect: Option<RectTransform>, axis: i32) -> f32 {}

    #[unity_method(name = "GetPreferredSize", static)]
    pub fn get_preferred_size(rect: Option<RectTransform>, axis: i32) -> f32 {}

    #[unity_method(name = "GetFlexibleSize", static)]
    pub fn get_flexible_size(rect: Option<RectTransform>, axis: i32) -> f32 {}

    #[unity_method(name = "GetMinWidth", static)]
    pub fn get_min_width(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetPreferredWidth", static)]
    pub fn get_preferred_width(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetFlexibleWidth", static)]
    pub fn get_flexible_width(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetMinHeight", static)]
    pub fn get_min_height(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetPreferredHeight", static)]
    pub fn get_preferred_height(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetFlexibleHeight", static)]
    pub fn get_flexible_height(rect: Option<RectTransform>) -> f32 {}

    #[unity_method(name = "GetLayoutProperty", static)]
    pub fn get_layout_property(rect: Option<RectTransform>, property: *mut c_void, default_value: f32) -> f32 {}

    #[unity_method(name = "GetLayoutProperty", static)]
    pub fn get_layout_property_1(rect: Option<RectTransform>, property: *mut c_void, default_value: f32, source: &mut *mut c_void) -> f32 {}

}
