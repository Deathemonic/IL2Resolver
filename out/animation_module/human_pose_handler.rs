#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::avatar::Avatar;
use crate::core_module::Transform;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanPoseHandler", namespace = "UnityEngine")]
pub struct HumanPoseHandler(pub *mut c_void);

#[unity_impl]
impl HumanPoseHandler {
    #[unity_ctor]
    pub fn new(avatar: Option<Avatar>, root: Option<Transform>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(avatar: Option<Avatar>, joint_paths: Array<SystemString>) -> Option<Self> {}

    #[unity_icall("UnityEngine.HumanPoseHandler::Internal_CreateFromRoot(Avatar,Transform)")]
    pub fn internal_create_from_root(avatar: Option<Avatar>, root: Option<Transform>) -> isize {}

    #[unity_icall("UnityEngine.HumanPoseHandler::Internal_CreateFromJointPaths(Avatar,System.String[])")]
    pub fn internal_create_from_joint_paths(avatar: Option<Avatar>, joint_paths: Array<SystemString>) -> isize {}

    #[unity_icall("UnityEngine.HumanPoseHandler::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::GetHumanPose(Vector3&,Quaternion&,System.Single[])")]
    pub fn get_human_pose(&self, body_position: &mut Vector3, body_rotation: &mut Quaternion, muscles: &mut Array<f32>) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::SetHumanPose(Vector3&,Quaternion&,System.Single[])")]
    pub fn set_human_pose(&self, body_position: &mut Vector3, body_rotation: &mut Quaternion, muscles: Array<f32>) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::GetInternalHumanPose(Vector3&,Quaternion&,System.Single[])")]
    pub fn get_internal_human_pose(&self, body_position: &mut Vector3, body_rotation: &mut Quaternion, muscles: &mut Array<f32>) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::SetInternalHumanPose(Vector3&,Quaternion&,System.Single[])")]
    pub fn set_internal_human_pose(&self, body_position: &mut Vector3, body_rotation: &mut Quaternion, muscles: Array<f32>) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::GetInternalAvatarPose(System.Void*,System.Int32)")]
    pub fn get_internal_avatar_pose(&self, avatar_pose: *mut (), avatar_pose_length: i32) {}

    #[unity_icall("UnityEngine.HumanPoseHandler::SetInternalAvatarPose(System.Void*,System.Int32)")]
    pub fn set_internal_avatar_pose(&self, avatar_pose: *mut (), avatar_pose_length: i32) {}

}
