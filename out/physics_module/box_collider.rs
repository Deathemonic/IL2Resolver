#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::core_module::{Component, Object};
use crate::physics_module::Collider;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "BoxCollider", namespace = "UnityEngine", inherit = "Collider,Component,Object")]
pub struct BoxCollider(pub *mut c_void);

#[unity_impl]
impl BoxCollider {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_extents")]
    pub fn get_extents(&self) -> Vector3 {}

    #[unity_method(name = "set_extents")]
    pub fn set_extents(&self, value: Vector3) {}

    #[unity_icall("UnityEngine.BoxCollider::get_center_Injected(Vector3&)")]
    pub fn get_center(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.BoxCollider::set_center_Injected(Vector3&)")]
    pub fn set_center(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.BoxCollider::get_size_Injected(Vector3&)")]
    pub fn get_size(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.BoxCollider::set_size_Injected(Vector3&)")]
    pub fn set_size(&self, value: &mut Vector3) {}

}
