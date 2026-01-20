#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorStateInfo", namespace = "UnityEngine", value_type)]
pub struct AnimatorStateInfo {
    pub m_name: i32,
    pub m_path: i32,
    pub m_full_path: i32,
    pub m_normalized_time: f32,
    pub m_length: f32,
    pub m_speed: f32,
    pub m_speed_multiplier: f32,
    pub m_tag: i32,
    pub m_loop: i32,
}

#[unity_impl]
impl AnimatorStateInfo {
    #[unity_method(name = "get_fullPathHash")]
    pub fn get_full_path_hash(&self) -> i32 {}

    #[unity_method(name = "get_nameHash")]
    pub fn get_name_hash(&self) -> i32 {}

    #[unity_method(name = "get_shortNameHash")]
    pub fn get_short_name_hash(&self) -> i32 {}

    #[unity_method(name = "get_normalizedTime")]
    pub fn get_normalized_time(&self) -> f32 {}

    #[unity_method(name = "get_length")]
    pub fn get_length(&self) -> f32 {}

    #[unity_method(name = "get_speed")]
    pub fn get_speed(&self) -> f32 {}

    #[unity_method(name = "get_speedMultiplier")]
    pub fn get_speed_multiplier(&self) -> f32 {}

    #[unity_method(name = "get_tagHash")]
    pub fn get_tag_hash(&self) -> i32 {}

    #[unity_method(name = "get_loop")]
    pub fn get_loop(&self) -> bool {}

    #[unity_method(name = "IsName")]
    pub fn is_name(&self, name: &str) -> bool {}

    #[unity_method(name = "IsTag")]
    pub fn is_tag(&self, tag: &str) -> bool {}

}
