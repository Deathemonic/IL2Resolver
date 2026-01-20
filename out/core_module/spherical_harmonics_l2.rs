#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemObject};
use crate::mscorlib::collections::{Array};
use super::color::Color;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "SphericalHarmonicsL2", namespace = "UnityEngine.Rendering", value_type)]
pub struct SphericalHarmonicsL2 {
    pub shr0: f32,
    pub shr1: f32,
    pub shr2: f32,
    pub shr3: f32,
    pub shr4: f32,
    pub shr5: f32,
    pub shr6: f32,
    pub shr7: f32,
    pub shr8: f32,
    pub shg0: f32,
    pub shg1: f32,
    pub shg2: f32,
    pub shg3: f32,
    pub shg4: f32,
    pub shg5: f32,
    pub shg6: f32,
    pub shg7: f32,
    pub shg8: f32,
    pub shb0: f32,
    pub shb1: f32,
    pub shb2: f32,
    pub shb3: f32,
    pub shb4: f32,
    pub shb5: f32,
    pub shb6: f32,
    pub shb7: f32,
    pub shb8: f32,
}

#[unity_impl]
impl SphericalHarmonicsL2 {
    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> f32 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rendering.SphericalHarmonicsL2::SetZero_Injected(SphericalHarmonicsL2&)")]
    pub fn clear(_unity_self: &mut SphericalHarmonicsL2) {}

    #[unity_icall("UnityEngine.Rendering.SphericalHarmonicsL2::AddAmbientLight_Injected(SphericalHarmonicsL2&,Color&)")]
    pub fn add_ambient_light(_unity_self: &mut SphericalHarmonicsL2, color: &mut Color) {}

    #[unity_icall("UnityEngine.Rendering.SphericalHarmonicsL2::AddDirectionalLightInternal(SphericalHarmonicsL2&,Vector3,Color)")]
    pub fn add_directional_light_internal(sh: &mut SphericalHarmonicsL2, direction: Vector3, color: Color) {}

    #[unity_icall("UnityEngine.Rendering.SphericalHarmonicsL2::EvaluateInternal(SphericalHarmonicsL2&,Vector3[],Color[])")]
    pub fn evaluate_internal(sh: &mut SphericalHarmonicsL2, directions: Array<Vector3>, results: &mut Array<Color>) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: SphericalHarmonicsL2) -> bool {}

    #[unity_icall("UnityEngine.Rendering.SphericalHarmonicsL2::AddDirectionalLightInternal_Injected(SphericalHarmonicsL2&,Vector3&,Color&)")]
    pub fn add_directional_light_internal_1(sh: &mut SphericalHarmonicsL2, direction: &mut Vector3, color: &mut Color) {}

}
