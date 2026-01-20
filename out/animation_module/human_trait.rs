#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanTrait", namespace = "UnityEngine")]
pub struct HumanTrait(pub *mut c_void);

#[unity_impl]
impl HumanTrait {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.HumanTrait::get_MuscleCount")]
    pub fn get_muscle_count() -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::get_MuscleName")]
    pub fn get_muscle_name() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.HumanTrait::get_BoneCount")]
    pub fn get_bone_count() -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::get_BoneName")]
    pub fn get_bone_name() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.HumanTrait::get_RequiredBoneCount")]
    pub fn get_required_bone_count() -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::GetBoneIndexFromMono(System.Int32)")]
    pub fn get_bone_index_from_mono(human_id: i32) -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::GetBoneIndexToMono(System.Int32)")]
    pub fn get_bone_index_to_mono(bone_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::Internal_MuscleFromBone(System.Int32,System.Int32)")]
    pub fn internal_muscle_from_bone(i: i32, dof_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::Internal_BoneFromMuscle(System.Int32)")]
    pub fn internal_bone_from_muscle(i: i32) -> i32 {}

    #[unity_icall("UnityEngine.HumanTrait::Internal_RequiredBone(System.Int32)")]
    pub fn internal_required_bone(i: i32) -> bool {}

    #[unity_icall("UnityEngine.HumanTrait::GetMuscleDefaultMin(System.Int32)")]
    pub fn get_muscle_default_min(i: i32) -> f32 {}

    #[unity_icall("UnityEngine.HumanTrait::GetMuscleDefaultMax(System.Int32)")]
    pub fn get_muscle_default_max(i: i32) -> f32 {}

    #[unity_icall("UnityEngine.HumanTrait::Internal_GetBoneHierarchyMass(System.Int32)")]
    pub fn internal_get_bone_hierarchy_mass(i: i32) -> f32 {}

    #[unity_icall("UnityEngine.HumanTrait::Internal_GetParent(System.Int32)")]
    pub fn internal_get_parent(i: i32) -> i32 {}

}
