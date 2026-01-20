#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::reflection_probe::ReflectionProbe;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ReflectionProbeBlendInfo", namespace = "UnityEngine.Rendering", value_type)]
pub struct ReflectionProbeBlendInfo {
    pub probe: Option<ReflectionProbe>,
    pub weight: f32,
}
