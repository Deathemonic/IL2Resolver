#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector3};
use crate::core_module::{Component, Object};
use crate::physics_module::Collider;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "CapsuleCollider", namespace = "UnityEngine", inherit = "Collider,Component,Object")]
pub struct CapsuleCollider(pub *mut c_void);

#[unity_impl]
impl CapsuleCollider {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.CapsuleCollider::get_center_Injected(Vector3&)")]
    pub fn get_center(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.CapsuleCollider::set_center_Injected(Vector3&)")]
    pub fn set_center(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.CapsuleCollider::get_radius")]
    pub fn get_radius(&self) -> f32 {}

    #[unity_icall("UnityEngine.CapsuleCollider::set_radius(System.Single)")]
    pub fn set_radius(&self, value: f32) {}

    #[unity_icall("UnityEngine.CapsuleCollider::get_height")]
    pub fn get_height(&self) -> f32 {}

    #[unity_icall("UnityEngine.CapsuleCollider::set_height(System.Single)")]
    pub fn set_height(&self, value: f32) {}

    #[unity_icall("UnityEngine.CapsuleCollider::get_direction")]
    pub fn get_direction(&self) -> i32 {}

    #[unity_icall("UnityEngine.CapsuleCollider::set_direction(System.Int32)")]
    pub fn set_direction(&self, value: i32) {}

    #[unity_icall("UnityEngine.CapsuleCollider::GetGlobalExtents_Injected(Vector2&)")]
    pub fn get_global_extents(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.CapsuleCollider::CalculateTransform_Injected(Matrix4x4&)")]
    pub fn calculate_transform(&self, ret: &mut Matrix4x4) {}

}
