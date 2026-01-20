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
#[unity(assembly = "UnityEngine.AnimationModule", class = "ScaleConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct ScaleConstraint(pub *mut c_void);

#[unity_impl]
impl ScaleConstraint {
    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_scaleAtRest_Injected(Vector3&)")]
    pub fn get_scale_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_scaleAtRest_Injected(Vector3&)")]
    pub fn set_scale_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_scaleOffset_Injected(Vector3&)")]
    pub fn get_scale_offset(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_scaleOffset_Injected(Vector3&)")]
    pub fn set_scale_offset(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_scalingAxis")]
    pub fn get_scaling_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_scalingAxis(Axis)")]
    pub fn set_scaling_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::Internal_Create(ScaleConstraint)")]
    pub fn internal_create(this: Option<ScaleConstraint>) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::SetSourcesInternal(ScaleConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<ScaleConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::GetSourceCountInternal(ScaleConstraint)")]
    pub fn remove_source(this: Option<ScaleConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::GetSourceCountInternal(ScaleConstraint)")]
    pub fn set_source(this: Option<ScaleConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.ScaleConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
