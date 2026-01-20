#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "AccelerationEvent", namespace = "UnityEngine", value_type)]
pub struct AccelerationEvent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub m_time_delta: f32,
}

#[unity_impl]
impl AccelerationEvent {
    #[unity_method(name = "get_acceleration")]
    pub fn get_acceleration(&self) -> Vector3 {}

    #[unity_method(name = "get_deltaTime")]
    pub fn get_delta_time(&self) -> f32 {}

}
