#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GradientAlphaKey", namespace = "UnityEngine", value_type)]
pub struct GradientAlphaKey {
    pub alpha: f32,
    pub time: f32,
}
