#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::animation_state::AnimationState;
use super::animator_clip_info::AnimatorClipInfo;
use super::animator_state_info::AnimatorStateInfo;
use crate::core_module::{Object, SendMessageOptions};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimationEvent", namespace = "UnityEngine")]
pub struct AnimationEvent(pub *mut c_void);

#[unity_impl]
impl AnimationEvent {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_data")]
    pub fn get_data(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_data")]
    pub fn set_data(&self, value: &str) {}

    #[unity_method(name = "get_stringParameter")]
    pub fn get_string_parameter(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_stringParameter")]
    pub fn set_string_parameter(&self, value: &str) {}

    #[unity_method(name = "get_floatParameter")]
    pub fn get_float_parameter(&self) -> f32 {}

    #[unity_method(name = "set_floatParameter")]
    pub fn set_float_parameter(&self, value: f32) {}

    #[unity_method(name = "get_intParameter")]
    pub fn get_int_parameter(&self) -> i32 {}

    #[unity_method(name = "set_intParameter")]
    pub fn set_int_parameter(&self, value: i32) {}

    #[unity_method(name = "get_objectReferenceParameter")]
    pub fn get_object_reference_parameter(&self) -> Option<Object> {}

    #[unity_method(name = "set_objectReferenceParameter")]
    pub fn set_object_reference_parameter(&self, value: Option<Object>) {}

    #[unity_method(name = "get_functionName")]
    pub fn get_function_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_functionName")]
    pub fn set_function_name(&self, value: &str) {}

    #[unity_method(name = "get_time")]
    pub fn get_time(&self) -> f32 {}

    #[unity_method(name = "set_time")]
    pub fn set_time(&self, value: f32) {}

    #[unity_method(name = "get_messageOptions")]
    pub fn get_message_options(&self) -> SendMessageOptions {}

    #[unity_method(name = "set_messageOptions")]
    pub fn set_message_options(&self, value: SendMessageOptions) {}

    #[unity_method(name = "get_isFiredByLegacy")]
    pub fn get_is_fired_by_legacy(&self) -> bool {}

    #[unity_method(name = "get_isFiredByAnimator")]
    pub fn get_is_fired_by_animator(&self) -> bool {}

    #[unity_method(name = "get_animationState")]
    pub fn get_animation_state(&self) -> Option<AnimationState> {}

    #[unity_method(name = "get_animatorStateInfo")]
    pub fn get_animator_state_info(&self) -> AnimatorStateInfo {}

    #[unity_method(name = "get_animatorClipInfo")]
    pub fn get_animator_clip_info(&self) -> AnimatorClipInfo {}

}
