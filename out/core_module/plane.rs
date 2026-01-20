#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemString};
use super::ray::Ray;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Plane", namespace = "UnityEngine", value_type)]
pub struct Plane {
    pub m_normal: Vector3,
    pub m_distance: f32,
}

#[unity_impl]
impl Plane {
    #[unity_method(name = "get_normal")]
    pub fn get_normal(&self) -> Vector3 {}

    #[unity_method(name = "set_normal")]
    pub fn set_normal(&self, value: Vector3) {}

    #[unity_method(name = "get_distance")]
    pub fn get_distance(&self) -> f32 {}

    #[unity_method(name = "set_distance")]
    pub fn set_distance(&self, value: f32) {}

    #[unity_method(name = "get_flipped")]
    pub fn get_flipped(&self) -> Plane {}

    #[unity_method(name = "SetNormalAndPosition")]
    pub fn set_normal_and_position(&self, in_normal: Vector3, in_point: Vector3) {}

    #[unity_method(name = "Set3Points")]
    pub fn set3points(&self, a: Vector3, b: Vector3, c: Vector3) {}

    #[unity_method(name = "Flip")]
    pub fn flip(&self) {}

    #[unity_method(name = "Translate")]
    pub fn translate(&self, translation: Vector3) {}

    #[unity_method(name = "Translate", static)]
    pub fn translate_1(plane: Plane, translation: Vector3) -> Plane {}

    #[unity_method(name = "ClosestPointOnPlane")]
    pub fn closest_point_on_plane(&self, point: Vector3) -> Vector3 {}

    #[unity_method(name = "GetDistanceToPoint")]
    pub fn get_distance_to_point(&self, point: Vector3) -> f32 {}

    #[unity_method(name = "GetSide")]
    pub fn get_side(&self, point: Vector3) -> bool {}

    #[unity_method(name = "SameSide")]
    pub fn same_side(&self, in_pt0: Vector3, in_pt1: Vector3) -> bool {}

    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, ray: Ray, enter: &mut f32) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

}
