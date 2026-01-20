#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::avatar_mask_body_part::AvatarMaskBodyPart;
use crate::core_module::Transform;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AvatarMask", namespace = "UnityEngine", inherit = "Object")]
pub struct AvatarMask(pub *mut c_void);

#[unity_impl]
impl AvatarMask {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_humanoidBodyPartCount")]
    pub fn get_humanoid_body_part_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.AvatarMask::get_transformCount")]
    pub fn get_transform_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.AvatarMask::set_transformCount(System.Int32)")]
    pub fn set_transform_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.AvatarMask::Internal_Create(AvatarMask)")]
    pub fn internal_create(this: Option<AvatarMask>) {}

    #[unity_icall("UnityEngine.AvatarMask::GetHumanoidBodyPartActive(AvatarMaskBodyPart)")]
    pub fn get_humanoid_body_part_active(&self, index: AvatarMaskBodyPart) -> bool {}

    #[unity_icall("UnityEngine.AvatarMask::SetHumanoidBodyPartActive(AvatarMaskBodyPart,System.Boolean)")]
    pub fn set_humanoid_body_part_active(&self, index: AvatarMaskBodyPart, value: bool) {}

    #[unity_icall("UnityEngine.AvatarMask::AddTransformPath(Transform,System.Boolean)")]
    pub fn add_transform_path(&self, transform: Option<Transform>, recursive: bool) {}

    #[unity_icall("UnityEngine.AvatarMask::RemoveTransformPath(Transform,System.Boolean)")]
    pub fn remove_transform_path(&self, transform: Option<Transform>, recursive: bool) {}

    #[unity_icall("UnityEngine.AvatarMask::GetTransformPath(System.Int32)")]
    pub fn get_transform_path(&self, index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AvatarMask::SetTransformPath(System.Int32,System.String)")]
    pub fn set_transform_path(&self, index: i32, path: &str) {}

    #[unity_icall("UnityEngine.AvatarMask::GetTransformWeight(System.Int32)")]
    pub fn get_transform_weight(&self, index: i32) -> f32 {}

    #[unity_icall("UnityEngine.AvatarMask::SetTransformWeight(System.Int32,System.Single)")]
    pub fn set_transform_weight(&self, index: i32, weight: f32) {}

}
