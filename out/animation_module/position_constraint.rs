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
#[unity(assembly = "UnityEngine.AnimationModule", class = "PositionConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct PositionConstraint(pub *mut c_void);

#[unity_impl]
impl PositionConstraint {
    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_translationAtRest_Injected(Vector3&)")]
    pub fn get_translation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_translationAtRest_Injected(Vector3&)")]
    pub fn set_translation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_translationOffset_Injected(Vector3&)")]
    pub fn get_translation_offset(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_translationOffset_Injected(Vector3&)")]
    pub fn set_translation_offset(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_translationAxis")]
    pub fn get_translation_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_translationAxis(Axis)")]
    pub fn set_translation_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::Internal_Create(PositionConstraint)")]
    pub fn internal_create(this: Option<PositionConstraint>) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::SetSourcesInternal(PositionConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<PositionConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::GetSourceCountInternal(PositionConstraint)")]
    pub fn remove_source(this: Option<PositionConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::GetSourceCountInternal(PositionConstraint)")]
    pub fn set_source(this: Option<PositionConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.PositionConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
