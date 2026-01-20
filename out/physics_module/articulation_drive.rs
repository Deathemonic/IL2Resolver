#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ArticulationDrive", namespace = "UnityEngine", value_type)]
pub struct ArticulationDrive {
    pub lower_limit: f32,
    pub upper_limit: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub force_limit: f32,
    pub target: f32,
    pub target_velocity: f32,
}
