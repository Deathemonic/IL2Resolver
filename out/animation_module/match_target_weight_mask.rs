#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "MatchTargetWeightMask", namespace = "UnityEngine", value_type)]
pub struct MatchTargetWeightMask {
    pub m_position_xyz_weight: Vector3,
    pub m_rotation_weight: f32,
}

#[unity_impl]
impl MatchTargetWeightMask {
    #[unity_method(name = "get_positionXYZWeight")]
    pub fn get_position_xyz_weight(&self) -> Vector3 {}

    #[unity_method(name = "set_positionXYZWeight")]
    pub fn set_position_xyz_weight(&self, value: Vector3) {}

    #[unity_method(name = "get_rotationWeight")]
    pub fn get_rotation_weight(&self) -> f32 {}

    #[unity_method(name = "set_rotationWeight")]
    pub fn set_rotation_weight(&self, value: f32) {}

}
