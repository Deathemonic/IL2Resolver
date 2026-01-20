#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::camera;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GateFitParameters", namespace = "UnityEngine", value_type)]
pub struct GateFitParameters {
    pub mode: camera::GateFitMode,
    pub aspect: f32,
}

#[unity_impl]
impl GateFitParameters {
    #[unity_method(name = "get_mode")]
    pub fn get_mode(&self) -> camera::GateFitMode {}

    #[unity_method(name = "set_mode")]
    pub fn set_mode(&self, value: camera::GateFitMode) {}

    #[unity_method(name = "get_aspect")]
    pub fn get_aspect(&self) -> f32 {}

    #[unity_method(name = "set_aspect")]
    pub fn set_aspect(&self, value: f32) {}

}
