#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemArray, SystemObject, SystemType};
use crate::mscorlib::collections::{Array};
use super::animation_event::AnimationEvent;
use crate::core_module::{AnimationCurve, Bounds, GameObject, WrapMode};
use crate::animation_module::Motion;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimationClip", namespace = "UnityEngine", inherit = "Motion,Object")]
pub struct AnimationClip(pub *mut c_void);

#[unity_impl]
impl AnimationClip {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AnimationClip::get_length")]
    pub fn get_length(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationClip::get_frameRate")]
    pub fn get_frame_rate(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimationClip::set_frameRate(System.Single)")]
    pub fn set_frame_rate(&self, value: f32) {}

    #[unity_icall("UnityEngine.AnimationClip::get_wrapMode")]
    pub fn get_wrap_mode(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.AnimationClip::set_wrapMode(WrapMode)")]
    pub fn set_wrap_mode(&self, value: WrapMode) {}

    #[unity_icall("UnityEngine.AnimationClip::get_localBounds_Injected(Bounds&)")]
    pub fn get_local_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.AnimationClip::set_localBounds_Injected(Bounds&)")]
    pub fn set_local_bounds(&self, value: &mut Bounds) {}

    #[unity_icall("UnityEngine.AnimationClip::get_legacy")]
    pub fn get_legacy(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::set_legacy(System.Boolean)")]
    pub fn set_legacy(&self, value: bool) {}

    #[unity_icall("UnityEngine.AnimationClip::get_humanMotion")]
    pub fn get_human_motion(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::get_empty")]
    pub fn get_empty(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::get_hasGenericRootTransform")]
    pub fn get_has_generic_root_transform(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::get_hasMotionFloatCurves")]
    pub fn get_has_motion_float_curves(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::get_hasMotionCurves")]
    pub fn get_has_motion_curves(&self) -> bool {}

    #[unity_icall("UnityEngine.AnimationClip::get_hasRootCurves")]
    pub fn get_has_root_curves(&self) -> bool {}

    #[unity_method(name = "get_events")]
    pub fn get_events(&self) -> Array<AnimationEvent> {}

    #[unity_method(name = "set_events")]
    pub fn set_events(&self, value: Array<AnimationEvent>) {}

    #[unity_icall("UnityEngine.AnimationClip::Internal_CreateAnimationClip(AnimationClip)")]
    pub fn internal_create_animation_clip(this: Option<AnimationClip>) {}

    #[unity_icall("UnityEngine.AnimationClip::get_wrapMode")]
    pub fn sample_animation(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.AnimationClip::SampleAnimation(GameObject,AnimationClip,System.Single,WrapMode)")]
    pub fn sample_animation_1(go: Option<GameObject>, clip: Option<AnimationClip>, in_time: f32, wrap_mode: WrapMode) {}

    #[unity_icall("UnityEngine.AnimationClip::SetCurve(System.String,System.Type,System.String,AnimationCurve)")]
    pub fn set_curve(&self, relative_path: &str, type_ref: Option<SystemType>, property_name: &str, curve: Option<AnimationCurve>) {}

    #[unity_icall("UnityEngine.AnimationClip::EnsureQuaternionContinuity")]
    pub fn ensure_quaternion_continuity(&self) {}

    #[unity_icall("UnityEngine.AnimationClip::ClearCurves")]
    pub fn clear_curves(&self) {}

    #[unity_icall("UnityEngine.AnimationClip::AddEventInternal(System.Object)")]
    pub fn add_event_internal(&self, evt: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.AnimationClip::SetEventsInternal(System.Array)")]
    pub fn set_events_internal(&self, value: Option<SystemArray>) {}

    #[unity_icall("UnityEngine.AnimationClip::GetEventsInternal")]
    pub fn get_events_internal(&self) -> Option<SystemArray> {}

}
