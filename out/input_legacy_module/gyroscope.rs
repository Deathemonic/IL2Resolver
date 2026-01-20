#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "Gyroscope", namespace = "UnityEngine")]
pub struct Gyroscope(pub *mut c_void);

#[unity_impl]
impl Gyroscope {
    #[unity_method(name = "get_rotationRate")]
    pub fn get_rotation_rate(&self) -> Vector3 {}

    #[unity_method(name = "get_rotationRateUnbiased")]
    pub fn get_rotation_rate_unbiased(&self) -> Vector3 {}

    #[unity_method(name = "get_gravity")]
    pub fn get_gravity(&self) -> Vector3 {}

    #[unity_method(name = "get_userAcceleration")]
    pub fn get_user_acceleration(&self) -> Vector3 {}

    #[unity_method(name = "get_attitude")]
    pub fn get_attitude(&self) -> Quaternion {}

    #[unity_method(name = "get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_method(name = "set_enabled")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_method(name = "get_updateInterval")]
    pub fn get_update_interval(&self) -> f32 {}

    #[unity_method(name = "set_updateInterval")]
    pub fn set_update_interval(&self, value: f32) {}

    #[unity_icall("UnityEngine.Gyroscope::rotationRate_Internal(System.Int32)")]
    pub fn rotation_rate_internal(idx: i32) -> Vector3 {}

    #[unity_icall("UnityEngine.Gyroscope::rotationRateUnbiased_Internal(System.Int32)")]
    pub fn rotation_rate_unbiased_internal(idx: i32) -> Vector3 {}

    #[unity_icall("UnityEngine.Gyroscope::gravity_Internal(System.Int32)")]
    pub fn gravity_internal(idx: i32) -> Vector3 {}

    #[unity_icall("UnityEngine.Gyroscope::userAcceleration_Internal(System.Int32)")]
    pub fn user_acceleration_internal(idx: i32) -> Vector3 {}

    #[unity_icall("UnityEngine.Gyroscope::attitude_Internal(System.Int32)")]
    pub fn attitude_internal(idx: i32) -> Quaternion {}

    #[unity_icall("UnityEngine.Gyroscope::getEnabled_Internal(System.Int32)")]
    pub fn get_enabled_internal(idx: i32) -> bool {}

    #[unity_icall("UnityEngine.Gyroscope::setEnabled_Internal(System.Int32,System.Boolean)")]
    pub fn set_enabled_internal(idx: i32, enabled: bool) {}

    #[unity_icall("UnityEngine.Gyroscope::getUpdateInterval_Internal(System.Int32)")]
    pub fn get_update_interval_internal(idx: i32) -> f32 {}

    #[unity_icall("UnityEngine.Gyroscope::setUpdateInterval_Internal(System.Int32,System.Single)")]
    pub fn set_update_interval_internal(idx: i32, interval: f32) {}

    #[unity_icall("UnityEngine.Gyroscope::rotationRate_Internal_Injected(System.Int32,Vector3&)")]
    pub fn rotation_rate_internal_1(idx: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gyroscope::rotationRateUnbiased_Internal_Injected(System.Int32,Vector3&)")]
    pub fn rotation_rate_unbiased_internal_1(idx: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gyroscope::gravity_Internal_Injected(System.Int32,Vector3&)")]
    pub fn gravity_internal_1(idx: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gyroscope::userAcceleration_Internal_Injected(System.Int32,Vector3&)")]
    pub fn user_acceleration_internal_1(idx: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gyroscope::attitude_Internal_Injected(System.Int32,Quaternion&)")]
    pub fn attitude_internal_1(idx: i32, ret: &mut Quaternion) {}

}
