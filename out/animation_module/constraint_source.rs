#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::core_module::Transform;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "ConstraintSource", namespace = "UnityEngine.Animations", value_type)]
pub struct ConstraintSource {
    pub m_source_transform: Option<Transform>,
    pub m_weight: f32,
}

#[unity_impl]
impl ConstraintSource {
    #[unity_method(name = "get_sourceTransform")]
    pub fn get_source_transform(&self) -> Option<Transform> {}

    #[unity_method(name = "set_sourceTransform")]
    pub fn set_source_transform(&self, value: Option<Transform>) {}

    #[unity_method(name = "get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_method(name = "set_weight")]
    pub fn set_weight(&self, value: f32) {}

}
