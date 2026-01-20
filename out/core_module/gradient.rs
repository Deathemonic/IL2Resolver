#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::gradient_alpha_key::GradientAlphaKey;
use super::gradient_color_key::GradientColorKey;
use super::gradient_mode::GradientMode;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Gradient", namespace = "UnityEngine")]
pub struct Gradient(pub *mut c_void);

#[unity_impl]
impl Gradient {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Gradient::get_colorKeys")]
    pub fn get_color_keys(&self) -> Array<GradientColorKey> {}

    #[unity_icall("UnityEngine.Gradient::set_colorKeys(GradientColorKey[])")]
    pub fn set_color_keys(&self, value: Array<GradientColorKey>) {}

    #[unity_icall("UnityEngine.Gradient::get_alphaKeys")]
    pub fn get_alpha_keys(&self) -> Array<GradientAlphaKey> {}

    #[unity_icall("UnityEngine.Gradient::set_alphaKeys(GradientAlphaKey[])")]
    pub fn set_alpha_keys(&self, value: Array<GradientAlphaKey>) {}

    #[unity_icall("UnityEngine.Gradient::get_mode")]
    pub fn get_mode(&self) -> GradientMode {}

    #[unity_icall("UnityEngine.Gradient::set_mode(GradientMode)")]
    pub fn set_mode(&self, value: GradientMode) {}

    #[unity_icall("UnityEngine.Gradient::Init")]
    pub fn init() -> isize {}

    #[unity_icall("UnityEngine.Gradient::Cleanup")]
    pub fn cleanup(&self) {}

    #[unity_icall("UnityEngine.Gradient::Internal_Equals(System.IntPtr)")]
    pub fn internal_equals(&self, other: isize) -> bool {}

    #[unity_icall("UnityEngine.Gradient::Evaluate(System.Single)")]
    pub fn evaluate(&self, time: f32) -> Color {}

    #[unity_icall("UnityEngine.Gradient::SetKeys(GradientColorKey[],GradientAlphaKey[])")]
    pub fn set_keys(&self, color_keys: Array<GradientColorKey>, alpha_keys: Array<GradientAlphaKey>) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_icall("UnityEngine.Gradient::Evaluate_Injected(System.Single,Color&)")]
    pub fn evaluate_1(&self, time: f32, ret: &mut Color) {}

}
