#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::weighted_mode::WeightedMode;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Keyframe", namespace = "UnityEngine", value_type)]
pub struct Keyframe {
    pub m_time: f32,
    pub m_value: f32,
    pub m_in_tangent: f32,
    pub m_out_tangent: f32,
    pub m_weighted_mode: i32,
    pub m_in_weight: f32,
    pub m_out_weight: f32,
}

#[unity_impl]
impl Keyframe {
    #[unity_method(name = "get_time")]
    pub fn get_time(&self) -> f32 {}

    #[unity_method(name = "set_time")]
    pub fn set_time(&self, value: f32) {}

    #[unity_method(name = "get_value")]
    pub fn get_value(&self) -> f32 {}

    #[unity_method(name = "set_value")]
    pub fn set_value(&self, value: f32) {}

    #[unity_method(name = "get_inTangent")]
    pub fn get_in_tangent(&self) -> f32 {}

    #[unity_method(name = "set_inTangent")]
    pub fn set_in_tangent(&self, value: f32) {}

    #[unity_method(name = "get_outTangent")]
    pub fn get_out_tangent(&self) -> f32 {}

    #[unity_method(name = "set_outTangent")]
    pub fn set_out_tangent(&self, value: f32) {}

    #[unity_method(name = "get_inWeight")]
    pub fn get_in_weight(&self) -> f32 {}

    #[unity_method(name = "set_inWeight")]
    pub fn set_in_weight(&self, value: f32) {}

    #[unity_method(name = "get_outWeight")]
    pub fn get_out_weight(&self) -> f32 {}

    #[unity_method(name = "set_outWeight")]
    pub fn set_out_weight(&self, value: f32) {}

    #[unity_method(name = "get_weightedMode")]
    pub fn get_weighted_mode(&self) -> WeightedMode {}

    #[unity_method(name = "set_weightedMode")]
    pub fn set_weighted_mode(&self, value: WeightedMode) {}

    #[unity_method(name = "get_tangentMode")]
    pub fn get_tangent_mode(&self) -> i32 {}

    #[unity_method(name = "set_tangentMode")]
    pub fn set_tangent_mode(&self, value: i32) {}

}
