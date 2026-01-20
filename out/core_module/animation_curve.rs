#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::keyframe::Keyframe;
use super::wrap_mode::WrapMode;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "AnimationCurve", namespace = "UnityEngine")]
pub struct AnimationCurve(pub *mut c_void);

#[unity_impl]
impl AnimationCurve {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(keys: Array<Keyframe>) -> Option<Self> {}

    #[unity_method(name = "get_keys")]
    pub fn get_keys(&self) -> Array<Keyframe> {}

    #[unity_method(name = "set_keys")]
    pub fn set_keys(&self, value: Array<Keyframe>) {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> Keyframe {}

    #[unity_icall("UnityEngine.AnimationCurve::get_length")]
    pub fn get_length(&self) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::get_preWrapMode")]
    pub fn get_pre_wrap_mode(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.AnimationCurve::set_preWrapMode(WrapMode)")]
    pub fn set_pre_wrap_mode(&self, value: WrapMode) {}

    #[unity_icall("UnityEngine.AnimationCurve::get_postWrapMode")]
    pub fn get_post_wrap_mode(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.AnimationCurve::set_postWrapMode(WrapMode)")]
    pub fn set_post_wrap_mode(&self, value: WrapMode) {}

    #[unity_icall("UnityEngine.AnimationCurve::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.AnimationCurve::Internal_Create(Keyframe[])")]
    pub fn internal_create(keys: Array<Keyframe>) -> isize {}

    #[unity_icall("UnityEngine.AnimationCurve::Internal_Equals(System.IntPtr)")]
    pub fn internal_equals(&self, other: isize) -> bool {}

    #[unity_icall("UnityEngine.AnimationCurve::Evaluate(System.Single)")]
    pub fn evaluate(&self, time: f32) -> f32 {}

    #[unity_icall("UnityEngine.AnimationCurve::AddKey(System.Single,System.Single)")]
    pub fn add_key(&self, time: f32, value: f32) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::AddKey_Internal(Keyframe)")]
    pub fn add_key_internal(&self, key: Keyframe) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::MoveKey(System.Int32,Keyframe)")]
    pub fn move_key(&self, index: i32, key: Keyframe) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::RemoveKey(System.Int32)")]
    pub fn remove_key(&self, index: i32) {}

    #[unity_icall("UnityEngine.AnimationCurve::GetKey(System.Int32)")]
    pub fn get_key(&self, index: i32) -> Keyframe {}

    #[unity_icall("UnityEngine.AnimationCurve::SmoothTangents(System.Int32,System.Single)")]
    pub fn smooth_tangents(&self, index: i32, weight: f32) {}

    #[unity_method(name = "Constant", static)]
    pub fn constant(time_start: f32, time_end: f32, value: f32) -> Option<AnimationCurve> {}

    #[unity_method(name = "Linear", static)]
    pub fn linear(time_start: f32, value_start: f32, time_end: f32, value_end: f32) -> Option<AnimationCurve> {}

    #[unity_method(name = "EaseInOut", static)]
    pub fn ease_in_out(time_start: f32, value_start: f32, time_end: f32, value_end: f32) -> Option<AnimationCurve> {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::AddKey_Internal_Injected(Keyframe&)")]
    pub fn add_key_internal_1(&self, key: &mut Keyframe) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::MoveKey_Injected(System.Int32,Keyframe&)")]
    pub fn move_key_1(&self, index: i32, key: &mut Keyframe) -> i32 {}

    #[unity_icall("UnityEngine.AnimationCurve::GetKey_Injected(System.Int32,Keyframe&)")]
    pub fn get_key_1(&self, index: i32, ret: &mut Keyframe) {}

}
