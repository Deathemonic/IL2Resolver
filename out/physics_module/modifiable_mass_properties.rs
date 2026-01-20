#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ModifiableMassProperties", namespace = "UnityEngine", value_type)]
pub struct ModifiableMassProperties {
    pub inverse_mass_scale: f32,
    pub inverse_inertia_scale: f32,
    pub other_inverse_mass_scale: f32,
    pub other_inverse_inertia_scale: f32,
}
