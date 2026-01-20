#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanLimit", namespace = "UnityEngine", value_type)]
pub struct HumanLimit {
    pub m_min: Vector3,
    pub m_max: Vector3,
    pub m_center: Vector3,
    pub m_axis_length: f32,
    pub m_use_default_values: i32,
}

#[unity_impl]
impl HumanLimit {
    #[unity_method(name = "get_useDefaultValues")]
    pub fn get_use_default_values(&self) -> bool {}

    #[unity_method(name = "set_useDefaultValues")]
    pub fn set_use_default_values(&self, value: bool) {}

    #[unity_method(name = "get_min")]
    pub fn get_min(&self) -> Vector3 {}

    #[unity_method(name = "set_min")]
    pub fn set_min(&self, value: Vector3) {}

    #[unity_method(name = "get_max")]
    pub fn get_max(&self) -> Vector3 {}

    #[unity_method(name = "set_max")]
    pub fn set_max(&self, value: Vector3) {}

    #[unity_method(name = "get_center")]
    pub fn get_center(&self) -> Vector3 {}

    #[unity_method(name = "set_center")]
    pub fn set_center(&self, value: Vector3) {}

    #[unity_method(name = "get_axisLength")]
    pub fn get_axis_length(&self) -> f32 {}

    #[unity_method(name = "set_axisLength")]
    pub fn set_axis_length(&self, value: f32) {}

}
