#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "BoneWeight", namespace = "UnityEngine", value_type)]
pub struct BoneWeight {
    pub m_weight0: f32,
    pub m_weight1: f32,
    pub m_weight2: f32,
    pub m_weight3: f32,
    pub m_bone_index0: i32,
    pub m_bone_index1: i32,
    pub m_bone_index2: i32,
    pub m_bone_index3: i32,
}

#[unity_impl]
impl BoneWeight {
    #[unity_method(name = "get_weight0")]
    pub fn get_weight0(&self) -> f32 {}

    #[unity_method(name = "set_weight0")]
    pub fn set_weight0(&self, value: f32) {}

    #[unity_method(name = "get_weight1")]
    pub fn get_weight1(&self) -> f32 {}

    #[unity_method(name = "set_weight1")]
    pub fn set_weight1(&self, value: f32) {}

    #[unity_method(name = "get_weight2")]
    pub fn get_weight2(&self) -> f32 {}

    #[unity_method(name = "set_weight2")]
    pub fn set_weight2(&self, value: f32) {}

    #[unity_method(name = "get_weight3")]
    pub fn get_weight3(&self) -> f32 {}

    #[unity_method(name = "set_weight3")]
    pub fn set_weight3(&self, value: f32) {}

    #[unity_method(name = "get_boneIndex0")]
    pub fn get_bone_index0(&self) -> i32 {}

    #[unity_method(name = "set_boneIndex0")]
    pub fn set_bone_index0(&self, value: i32) {}

    #[unity_method(name = "get_boneIndex1")]
    pub fn get_bone_index1(&self) -> i32 {}

    #[unity_method(name = "set_boneIndex1")]
    pub fn set_bone_index1(&self, value: i32) {}

    #[unity_method(name = "get_boneIndex2")]
    pub fn get_bone_index2(&self) -> i32 {}

    #[unity_method(name = "set_boneIndex2")]
    pub fn set_bone_index2(&self, value: i32) {}

    #[unity_method(name = "get_boneIndex3")]
    pub fn get_bone_index3(&self) -> i32 {}

    #[unity_method(name = "set_boneIndex3")]
    pub fn set_bone_index3(&self, value: i32) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: BoneWeight) -> bool {}

}
