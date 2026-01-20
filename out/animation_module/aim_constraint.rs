#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::collections::{List};
use super::axis::Axis;
use super::constraint_source::ConstraintSource;
use crate::core_module::Transform;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AimConstraint", namespace = "UnityEngine.Animations", inherit = "Behaviour,Component,Object")]
pub struct AimConstraint(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldUpType {
    #[default]
    SceneUp = 0,
    ObjectUp = 1,
    ObjectRotationUp = 2,
    Vector = 3,
    None = 4,
}

#[unity_impl]
impl AimConstraint {
    #[unity_icall("UnityEngine.Animations.AimConstraint::get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_weight(System.Single)")]
    pub fn set_weight(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_constraintActive")]
    pub fn get_constraint_active(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_constraintActive(System.Boolean)")]
    pub fn set_constraint_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_locked")]
    pub fn get_locked(&self) -> bool {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_locked(System.Boolean)")]
    pub fn set_locked(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_rotationAtRest_Injected(Vector3&)")]
    pub fn get_rotation_at_rest(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_rotationAtRest_Injected(Vector3&)")]
    pub fn set_rotation_at_rest(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_rotationOffset_Injected(Vector3&)")]
    pub fn get_rotation_offset(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_rotationOffset_Injected(Vector3&)")]
    pub fn set_rotation_offset(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_rotationAxis")]
    pub fn get_rotation_axis(&self) -> Axis {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_rotationAxis(Axis)")]
    pub fn set_rotation_axis(&self, value: Axis) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_aimVector_Injected(Vector3&)")]
    pub fn get_aim_vector(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_aimVector_Injected(Vector3&)")]
    pub fn set_aim_vector(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_upVector_Injected(Vector3&)")]
    pub fn get_up_vector(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_upVector_Injected(Vector3&)")]
    pub fn set_up_vector(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_worldUpVector_Injected(Vector3&)")]
    pub fn get_world_up_vector(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_worldUpVector_Injected(Vector3&)")]
    pub fn set_world_up_vector(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_worldUpObject")]
    pub fn get_world_up_object(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_worldUpObject(Transform)")]
    pub fn set_world_up_object(&self, value: Option<Transform>) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::get_worldUpType")]
    pub fn get_world_up_type(&self) -> WorldUpType {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::set_worldUpType(AimConstraint.WorldUpType)")]
    pub fn set_world_up_type(&self, value: WorldUpType) {}

    #[unity_method(name = "get_sourceCount")]
    pub fn get_source_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::Internal_Create(AimConstraint)")]
    pub fn internal_create(this: Option<AimConstraint>) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::GetSources(List<ConstraintSource>)")]
    pub fn get_sources(&self, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::SetSourcesInternal(AimConstraint,List<ConstraintSource>)")]
    pub fn set_sources_internal(this: Option<AimConstraint>, sources: List<ConstraintSource>) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::AddSource_Injected(ConstraintSource&)")]
    pub fn add_source(&self, source: &mut ConstraintSource) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::GetSourceCountInternal(AimConstraint)")]
    pub fn remove_source(this: Option<AimConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::RemoveSourceInternal(System.Int32)")]
    pub fn remove_source_internal(&self, index: i32) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::GetSourceCountInternal(AimConstraint)")]
    pub fn set_source(this: Option<AimConstraint>) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::GetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn get_source_internal(&self, index: i32, ret: &mut ConstraintSource) {}

    #[unity_icall("UnityEngine.Animations.AimConstraint::SetSourceInternal_Injected(System.Int32,ConstraintSource&)")]
    pub fn set_source_internal(&self, index: i32, source: &mut ConstraintSource) {}

}
