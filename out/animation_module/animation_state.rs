#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::animation_blend_mode::AnimationBlendMode;
use super::animation_clip::AnimationClip;
use crate::core_module::{Transform, WrapMode};
use crate::core_module::TrackedReference;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimationState", namespace = "UnityEngine", inherit = "TrackedReference")]
pub struct AnimationState(pub *mut c_void);

#[unity_impl]
impl AnimationState {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AnimationState::get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationState::set_enabled(System.Boolean)")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_icall("UnityEngine.AnimationState::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationState::get_wrapMode")]
    pub fn get_wrap_mode(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.AnimationState::set_wrapMode(WrapMode)")]
    pub fn set_wrap_mode(&self, value: WrapMode) {}

    #[unity_icall("UnityEngine.AnimationState::get_time")]
    pub fn get_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::set_time(System.Single)")]
    pub fn set_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationState::get_normalizedTime")]
    pub fn get_normalized_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::set_normalizedTime(System.Single)")]
    pub fn set_normalized_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationState::get_speed")]
    pub fn get_speed(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::set_speed(System.Single)")]
    pub fn set_speed(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationState::get_normalizedSpeed")]
    pub fn get_normalized_speed(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::set_normalizedSpeed(System.Single)")]
    pub fn set_normalized_speed(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationState::get_length")]
    pub fn get_length(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationState::get_layer")]
    pub fn get_layer(&self) -> i32 {}

    #[unity_icall("UnityEngine.AnimationState::set_layer(System.Int32)")]
    pub fn set_layer(&self, value: i32) {}

    #[unity_icall("UnityEngine.AnimationState::get_clip")]
    pub fn get_clip(&self) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.AnimationState::get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AnimationState::set_name(System.String)")]
    pub fn set_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.AnimationState::get_blendMode")]
    pub fn get_blend_mode(&self) -> AnimationBlendMode {}

    #[unity_icall("UnityEngine.AnimationState::set_blendMode(AnimationBlendMode)")]
    pub fn set_blend_mode(&self, value: AnimationBlendMode) {}

    #[unity_icall("UnityEngine.AnimationState::AddMixingTransform(Transform,System.Boolean)")]
    pub fn add_mixing_transform(&self, mix: Option<Transform>, recursive: bool) {}

    #[unity_icall("UnityEngine.AnimationState::RemoveMixingTransform(Transform)")]
    pub fn remove_mixing_transform(&self, mix: Option<Transform>) {}

}
