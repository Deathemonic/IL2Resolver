#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemObject, SystemString};
use super::ray::Ray;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Bounds", namespace = "UnityEngine", value_type)]
pub struct Bounds {
    pub m_center: Vector3,
    pub m_extents: Vector3,
}

#[unity_impl]
impl Bounds {
    #[unity_method(name = "get_center")]
    pub fn get_center(&self) -> Vector3 {}

    #[unity_method(name = "set_center")]
    pub fn set_center(&self, value: Vector3) {}

    #[unity_method(name = "get_size")]
    pub fn get_size(&self) -> Vector3 {}

    #[unity_method(name = "set_size")]
    pub fn set_size(&self, value: Vector3) {}

    #[unity_method(name = "get_extents")]
    pub fn get_extents(&self) -> Vector3 {}

    #[unity_method(name = "set_extents")]
    pub fn set_extents(&self, value: Vector3) {}

    #[unity_method(name = "get_min")]
    pub fn get_min(&self) -> Vector3 {}

    #[unity_method(name = "set_min")]
    pub fn set_min(&self, value: Vector3) {}

    #[unity_method(name = "get_max")]
    pub fn get_max(&self) -> Vector3 {}

    #[unity_method(name = "set_max")]
    pub fn set_max(&self, value: Vector3) {}

    #[unity_icall("UnityEngine.Bounds::Contains(Vector3)")]
    pub fn contains(&self, point: Vector3) -> bool {}

    #[unity_icall("UnityEngine.Bounds::SqrDistance(Vector3)")]
    pub fn sqr_distance(&self, point: Vector3) -> f32 {}

    #[unity_icall("UnityEngine.Bounds::IntersectRayAABB(Ray,Bounds,System.Single&)")]
    pub fn intersect_ray_aabb(ray: Ray, bounds: Bounds, dist: &mut f32) -> bool {}

    #[unity_icall("UnityEngine.Bounds::ClosestPoint(Vector3)")]
    pub fn closest_point(&self, point: Vector3) -> Vector3 {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: Bounds) -> bool {}

    #[unity_method(name = "SetMinMax")]
    pub fn set_min_max(&self, min: Vector3, max: Vector3) {}

    #[unity_method(name = "Encapsulate")]
    pub fn encapsulate(&self, point: Vector3) {}

    #[unity_method(name = "Encapsulate")]
    pub fn encapsulate_1(&self, bounds: Bounds) {}

    #[unity_method(name = "Expand")]
    pub fn expand(&self, amount: f32) {}

    #[unity_method(name = "Expand")]
    pub fn expand_1(&self, amount: Vector3) {}

    #[unity_method(name = "Intersects")]
    pub fn intersects(&self, bounds: Bounds) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Bounds::Contains_Injected(Bounds&,Vector3&)")]
    pub fn contains_1(_unity_self: &mut Bounds, point: &mut Vector3) -> bool {}

    #[unity_icall("UnityEngine.Bounds::SqrDistance_Injected(Bounds&,Vector3&)")]
    pub fn sqr_distance_1(_unity_self: &mut Bounds, point: &mut Vector3) -> f32 {}

    #[unity_icall("UnityEngine.Bounds::IntersectRayAABB_Injected(Ray&,Bounds&,System.Single&)")]
    pub fn intersect_ray_aabb_1(ray: &mut Ray, bounds: &mut Bounds, dist: &mut f32) -> bool {}

    #[unity_icall("UnityEngine.Bounds::ClosestPoint_Injected(Bounds&,Vector3&,Vector3&)")]
    pub fn closest_point_1(_unity_self: &mut Bounds, point: &mut Vector3, ret: &mut Vector3) {}

}
