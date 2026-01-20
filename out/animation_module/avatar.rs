#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use super::human_description::HumanDescription;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "Avatar", namespace = "UnityEngine", inherit = "Object")]
pub struct Avatar(pub *mut c_void);

#[unity_impl]
impl Avatar {
    #[unity_icall("UnityEngine.Avatar::get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_icall("UnityEngine.Avatar::get_isHuman")]
    pub fn get_is_human(&self) -> bool {}

    #[unity_icall("UnityEngine.Avatar::get_humanDescription_Injected(HumanDescription&)")]
    pub fn get_human_description(&self, ret: &mut HumanDescription) {}

    #[unity_icall("UnityEngine.Avatar::SetMuscleMinMax(System.Int32,System.Single,System.Single)")]
    pub fn set_muscle_min_max(&self, muscle_id: i32, min: f32, max: f32) {}

    #[unity_icall("UnityEngine.Avatar::SetParameter(System.Int32,System.Single)")]
    pub fn set_parameter(&self, parameter_id: i32, value: f32) {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetAxisLength(System.Int32)")]
    pub fn internal_get_axis_length(&self, human_id: i32) -> f32 {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetPreRotation(System.Int32)")]
    pub fn internal_get_pre_rotation(&self, human_id: i32) -> Quaternion {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetPostRotation(System.Int32)")]
    pub fn internal_get_post_rotation(&self, human_id: i32) -> Quaternion {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetZYPostQ(System.Int32,Quaternion,Quaternion)")]
    pub fn internal_get_zy_post_q(&self, human_id: i32, parent_q: Quaternion, q: Quaternion) -> Quaternion {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetZYRoll(System.Int32,Vector3)")]
    pub fn internal_get_zy_roll(&self, human_id: i32, uvw: Vector3) -> Quaternion {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetLimitSign(System.Int32)")]
    pub fn internal_get_limit_sign(&self, human_id: i32) -> Vector3 {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetPreRotation_Injected(System.Int32,Quaternion&)")]
    pub fn internal_get_pre_rotation_1(&self, human_id: i32, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetPostRotation_Injected(System.Int32,Quaternion&)")]
    pub fn internal_get_post_rotation_1(&self, human_id: i32, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetZYPostQ_Injected(System.Int32,Quaternion&,Quaternion&,Quaternion&)")]
    pub fn internal_get_zy_post_q_1(&self, human_id: i32, parent_q: &mut Quaternion, q: &mut Quaternion, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetZYRoll_Injected(System.Int32,Vector3&,Quaternion&)")]
    pub fn internal_get_zy_roll_1(&self, human_id: i32, uvw: &mut Vector3, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Avatar::Internal_GetLimitSign_Injected(System.Int32,Vector3&)")]
    pub fn internal_get_limit_sign_1(&self, human_id: i32, ret: &mut Vector3) {}

}
