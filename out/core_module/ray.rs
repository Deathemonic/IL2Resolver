#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Ray", namespace = "UnityEngine", value_type)]
pub struct Ray {
    pub m_origin: Vector3,
    pub m_direction: Vector3,
}

#[unity_impl]
impl Ray {
    #[unity_method(name = "get_origin")]
    pub fn get_origin(&self) -> Vector3 {}

    #[unity_method(name = "set_origin")]
    pub fn set_origin(&self, value: Vector3) {}

    #[unity_method(name = "get_direction")]
    pub fn get_direction(&self) -> Vector3 {}

    #[unity_method(name = "set_direction")]
    pub fn set_direction(&self, value: Vector3) {}

    #[unity_method(name = "GetPoint")]
    pub fn get_point(&self, distance: f32) -> Vector3 {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

}
