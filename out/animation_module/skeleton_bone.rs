#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "SkeletonBone", namespace = "UnityEngine", value_type)]
pub struct SkeletonBone {
    pub name: Option<SystemString>,
    pub parent_name: Option<SystemString>,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

#[unity_impl]
impl SkeletonBone {
    #[unity_method(name = "get_transformModified")]
    pub fn get_transform_modified(&self) -> i32 {}

    #[unity_method(name = "set_transformModified")]
    pub fn set_transform_modified(&self, value: i32) {}

}
