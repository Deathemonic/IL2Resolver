#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::human_limit::HumanLimit;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanBone", namespace = "UnityEngine", value_type)]
pub struct HumanBone {
    pub m_bone_name: Option<SystemString>,
    pub m_human_name: Option<SystemString>,
    pub limit: HumanLimit,
}

#[unity_impl]
impl HumanBone {
    #[unity_method(name = "get_boneName")]
    pub fn get_bone_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_boneName")]
    pub fn set_bone_name(&self, value: &str) {}

    #[unity_method(name = "get_humanName")]
    pub fn get_human_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_humanName")]
    pub fn set_human_name(&self, value: &str) {}

}
