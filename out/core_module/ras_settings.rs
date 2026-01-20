#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::ray_tracing_acceleration_structure;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RASSettings", namespace = "UnityEngine.Experimental.Rendering", value_type)]
pub struct RASSettings {
    pub management_mode: ray_tracing_acceleration_structure::ManagementMode,
    pub ray_tracing_mode_mask: ray_tracing_acceleration_structure::RayTracingModeMask,
    pub layer_mask: i32,
}
