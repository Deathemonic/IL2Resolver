#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::duration_unit::DurationUnit;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorTransitionInfo", namespace = "UnityEngine", value_type)]
pub struct AnimatorTransitionInfo {
    pub m_full_path: i32,
    pub m_user_name: i32,
    pub m_name: i32,
    pub m_has_fixed_duration: bool,
    pub m_duration: f32,
    pub m_normalized_time: f32,
    pub m_any_state: bool,
    pub m_transition_type: i32,
}

#[unity_impl]
impl AnimatorTransitionInfo {
    #[unity_method(name = "get_fullPathHash")]
    pub fn get_full_path_hash(&self) -> i32 {}

    #[unity_method(name = "get_nameHash")]
    pub fn get_name_hash(&self) -> i32 {}

    #[unity_method(name = "get_userNameHash")]
    pub fn get_user_name_hash(&self) -> i32 {}

    #[unity_method(name = "get_durationUnit")]
    pub fn get_duration_unit(&self) -> DurationUnit {}

    #[unity_method(name = "get_duration")]
    pub fn get_duration(&self) -> f32 {}

    #[unity_method(name = "get_normalizedTime")]
    pub fn get_normalized_time(&self) -> f32 {}

    #[unity_method(name = "get_anyState")]
    pub fn get_any_state(&self) -> bool {}

    #[unity_method(name = "IsName")]
    pub fn is_name(&self, name: &str) -> bool {}

    #[unity_method(name = "IsUserName")]
    pub fn is_user_name(&self, name: &str) -> bool {}

}
