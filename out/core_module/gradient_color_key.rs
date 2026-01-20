#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::color::Color;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GradientColorKey", namespace = "UnityEngine", value_type)]
pub struct GradientColorKey {
    pub color: Color,
    pub time: f32,
}
