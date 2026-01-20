#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::animator_controller_parameter_type::AnimatorControllerParameterType;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorControllerParameter", namespace = "UnityEngine")]
pub struct AnimatorControllerParameter(pub *mut c_void);

#[unity_impl]
impl AnimatorControllerParameter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_nameHash")]
    pub fn get_name_hash(&self) -> i32 {}

    #[unity_method(name = "get_type")]
    pub fn get_type(&self) -> AnimatorControllerParameterType {}

    #[unity_method(name = "set_type")]
    pub fn set_type(&self, value: AnimatorControllerParameterType) {}

    #[unity_method(name = "get_defaultFloat")]
    pub fn get_default_float(&self) -> f32 {}

    #[unity_method(name = "set_defaultFloat")]
    pub fn set_default_float(&self, value: f32) {}

    #[unity_method(name = "get_defaultInt")]
    pub fn get_default_int(&self) -> i32 {}

    #[unity_method(name = "set_defaultInt")]
    pub fn set_default_int(&self, value: i32) {}

    #[unity_method(name = "get_defaultBool")]
    pub fn get_default_bool(&self) -> bool {}

    #[unity_method(name = "set_defaultBool")]
    pub fn set_default_bool(&self, value: bool) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, o: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
