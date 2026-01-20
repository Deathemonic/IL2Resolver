#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "BoneWeight1", namespace = "UnityEngine", value_type)]
pub struct BoneWeight1 {
    pub m_weight: f32,
    pub m_bone_index: i32,
}

#[unity_impl]
impl BoneWeight1 {
    #[unity_method(name = "get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_method(name = "set_weight")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_method(name = "get_boneIndex")]
    pub fn get_bone_index(&self) -> i32 {}

    #[unity_method(name = "set_boneIndex")]
    pub fn set_bone_index(&self, value: i32) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: BoneWeight1) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
