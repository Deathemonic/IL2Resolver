#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use super::physic_material_combine::PhysicMaterialCombine;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "PhysicMaterial", namespace = "UnityEngine", inherit = "Object")]
pub struct PhysicMaterial(pub *mut c_void);

#[unity_impl]
impl PhysicMaterial {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(name: &str) -> Option<Self> {}

    #[unity_method(name = "get_bouncyness")]
    pub fn get_bouncyness(&self) -> f32 {}

    #[unity_method(name = "set_bouncyness")]
    pub fn set_bouncyness(&self, value: f32) {}

    #[unity_method(name = "get_frictionDirection2")]
    pub fn get_friction_direction2(&self) -> Vector3 {}

    #[unity_method(name = "set_frictionDirection2")]
    pub fn set_friction_direction2(&self, value: Vector3) {}

    #[unity_method(name = "get_dynamicFriction2")]
    pub fn get_dynamic_friction2(&self) -> f32 {}

    #[unity_method(name = "set_dynamicFriction2")]
    pub fn set_dynamic_friction2(&self, value: f32) {}

    #[unity_method(name = "get_staticFriction2")]
    pub fn get_static_friction2(&self) -> f32 {}

    #[unity_method(name = "set_staticFriction2")]
    pub fn set_static_friction2(&self, value: f32) {}

    #[unity_method(name = "get_frictionDirection")]
    pub fn get_friction_direction(&self) -> Vector3 {}

    #[unity_method(name = "set_frictionDirection")]
    pub fn set_friction_direction(&self, value: Vector3) {}

    #[unity_icall("UnityEngine.PhysicMaterial::get_bounciness")]
    pub fn get_bounciness(&self) -> f32 {}

    #[unity_icall("UnityEngine.PhysicMaterial::set_bounciness(System.Single)")]
    pub fn set_bounciness(&self, value: f32) {}

    #[unity_icall("UnityEngine.PhysicMaterial::get_dynamicFriction")]
    pub fn get_dynamic_friction(&self) -> f32 {}

    #[unity_icall("UnityEngine.PhysicMaterial::set_dynamicFriction(System.Single)")]
    pub fn set_dynamic_friction(&self, value: f32) {}

    #[unity_icall("UnityEngine.PhysicMaterial::get_staticFriction")]
    pub fn get_static_friction(&self) -> f32 {}

    #[unity_icall("UnityEngine.PhysicMaterial::set_staticFriction(System.Single)")]
    pub fn set_static_friction(&self, value: f32) {}

    #[unity_icall("UnityEngine.PhysicMaterial::get_frictionCombine")]
    pub fn get_friction_combine(&self) -> PhysicMaterialCombine {}

    #[unity_icall("UnityEngine.PhysicMaterial::set_frictionCombine(PhysicMaterialCombine)")]
    pub fn set_friction_combine(&self, value: PhysicMaterialCombine) {}

    #[unity_icall("UnityEngine.PhysicMaterial::get_bounceCombine")]
    pub fn get_bounce_combine(&self) -> PhysicMaterialCombine {}

    #[unity_icall("UnityEngine.PhysicMaterial::set_bounceCombine(PhysicMaterialCombine)")]
    pub fn set_bounce_combine(&self, value: PhysicMaterialCombine) {}

    #[unity_icall("UnityEngine.PhysicMaterial::Internal_CreateDynamicsMaterial(PhysicMaterial,System.String)")]
    pub fn internal_create_dynamics_material(mat: Option<PhysicMaterial>, name: &str) {}

}
