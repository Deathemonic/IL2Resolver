#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "Motion", namespace = "UnityEngine", inherit = "Object")]
pub struct Motion(pub *mut c_void);

#[unity_impl]
impl Motion {
    #[unity_icall("UnityEngine.Motion::get_averageDuration")]
    pub fn get_average_duration(&self) -> f32 {}

    #[unity_icall("UnityEngine.Motion::get_averageAngularSpeed")]
    pub fn get_average_angular_speed(&self) -> f32 {}

    #[unity_icall("UnityEngine.Motion::get_averageSpeed_Injected(Vector3&)")]
    pub fn get_average_speed(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Motion::get_apparentSpeed")]
    pub fn get_apparent_speed(&self) -> f32 {}

    #[unity_icall("UnityEngine.Motion::get_isLooping")]
    pub fn get_is_looping(&self) -> bool {}

    #[unity_icall("UnityEngine.Motion::get_legacy")]
    pub fn get_legacy(&self) -> bool {}

    #[unity_icall("UnityEngine.Motion::get_isHumanMotion")]
    pub fn get_is_human_motion(&self) -> bool {}

    #[unity_method(name = "get_isAnimatorMotion")]
    pub fn get_is_animator_motion(&self) -> bool {}

    #[unity_method(name = "ValidateIfRetargetable")]
    pub fn validate_if_retargetable(&self, val: bool) -> bool {}

}
