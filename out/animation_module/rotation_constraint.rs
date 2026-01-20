#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::collections::{List};
use super::axis::Axis;
use super::constraint_source::ConstraintSource;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "RotationConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct RotationConstraint(pub *mut c_void);

#[unity_impl]
impl RotationConstraint {
    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_rotationAtRest_Injected(Vector3&)")]
    pub fn get_rotation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_rotationAtRest_Injected(Vector3&)")]
    pub fn set_rotation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_rotationOffset_Injected(Vector3&)")]
    pub fn get_rotation_offset(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_rotationOffset_Injected(Vector3&)")]
    pub fn set_rotation_offset(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_rotationAxis")]
    pub fn get_rotation_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_rotationAxis(Axis)")]
    pub fn set_rotation_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::Internal_Create(RotationConstraint)")]
    pub fn internal_create(this: Option<RotationConstraint>) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::SetSourcesInternal(RotationConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<RotationConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::GetSourceCountInternal(RotationConstraint)")]
    pub fn remove_source(this: Option<RotationConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::GetSourceCountInternal(RotationConstraint)")]
    pub fn set_source(this: Option<RotationConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.RotationConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
