#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::collections::{Array, List};
use super::axis::Axis;
use super::constraint_source::ConstraintSource;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "ParentConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct ParentConstraint(pub *mut c_void);

#[unity_impl]
impl ParentConstraint {
    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_translationAtRest_Injected(Vector3&)")]
    pub fn get_translation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_translationAtRest_Injected(Vector3&)")]
    pub fn set_translation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_rotationAtRest_Injected(Vector3&)")]
    pub fn get_rotation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_rotationAtRest_Injected(Vector3&)")]
    pub fn set_rotation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_translationOffsets")]
    pub fn get_translation_offsets(&self) -> Array<Vector3> {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_translationOffsets(Vector3[])")]
    pub fn set_translation_offsets(&self, value: Array<Vector3>) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_rotationOffsets")]
    pub fn get_rotation_offsets(&self) -> Array<Vector3> {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_rotationOffsets(Vector3[])")]
    pub fn set_rotation_offsets(&self, value: Array<Vector3>) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_translationAxis")]
    pub fn get_translation_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_translationAxis(Axis)")]
    pub fn set_translation_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::get_rotationAxis")]
    pub fn get_rotation_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::set_rotationAxis(Axis)")]
    pub fn set_rotation_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::Internal_Create(ParentConstraint)")]
    pub fn internal_create(this: Option<ParentConstraint>) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSourceCountInternal(ParentConstraint)")]
    pub fn set_translation_offset(this: Option<ParentConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSourceCountInternal(ParentConstraint)")]
    pub fn set_rotation_offset(this: Option<ParentConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::SetSourcesInternal(ParentConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<ParentConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSourceCountInternal(ParentConstraint)")]
    pub fn remove_source(this: Option<ParentConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSourceCountInternal(ParentConstraint)")]
    pub fn set_source(this: Option<ParentConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetTranslationOffsetInternal_Injected(System.Int32,Vector3&)")]
    pub fn get_translation_offset_internal(&self, index: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::SetTranslationOffsetInternal_Injected(System.Int32,Vector3&)")]
    pub fn set_translation_offset_internal(&self, index: i32, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetRotationOffsetInternal_Injected(System.Int32,Vector3&)")]
    pub fn get_rotation_offset_internal(&self, index: i32, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::SetRotationOffsetInternal_Injected(System.Int32,Vector3&)")]
    pub fn set_rotation_offset_internal(&self, index: i32, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.ParentConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
