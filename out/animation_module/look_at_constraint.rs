#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::collections::{List};
use super::constraint_source::ConstraintSource;
use crate::core_module::Transform;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "LookAtConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct LookAtConstraint(pub *mut c_void);

#[unity_impl]
impl LookAtConstraint {
    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_roll")]
    pub fn get_roll(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_roll(System.Single)")]
    pub fn set_roll(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_rotationAtRest_Injected(Vector3&)")]
    pub fn get_rotation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_rotationAtRest_Injected(Vector3&)")]
    pub fn set_rotation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_rotationOffset_Injected(Vector3&)")]
    pub fn get_rotation_offset(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_rotationOffset_Injected(Vector3&)")]
    pub fn set_rotation_offset(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_worldUpObject")]
    pub fn get_world_up_object(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_worldUpObject(Transform)")]
    pub fn set_world_up_object(&self, value: Option<Transform>) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::get_useUpObject")]
    pub fn get_use_up_object(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::set_useUpObject(System.Boolean)")]
    pub fn set_use_up_object(&self, value: bool) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::Internal_Create(LookAtConstraint)")]
    pub fn internal_create(this: Option<LookAtConstraint>) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::SetSourcesInternal(LookAtConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<LookAtConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::GetSourceCountInternal(LookAtConstraint)")]
    pub fn remove_source(this: Option<LookAtConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::GetSourceCountInternal(LookAtConstraint)")]
    pub fn set_source(this: Option<LookAtConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.LookAtConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
