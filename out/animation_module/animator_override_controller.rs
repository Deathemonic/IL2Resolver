#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::animation_clip::AnimationClip;
use super::animation_clip_pair::AnimationClipPair;
use crate::animation_module::RuntimeAnimatorController;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorOverrideController", namespace = "UnityEngine", inherit = "RuntimeAnimatorController,Object")]
pub struct AnimatorOverrideController(pub *mut c_void);

#[unity_impl]
impl AnimatorOverrideController {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(controller: Option<RuntimeAnimatorController>) -> Option<Self> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::get_runtimeAnimatorController")]
    pub fn get_runtime_animator_controller(&self) -> Option<RuntimeAnimatorController> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::set_runtimeAnimatorController(RuntimeAnimatorController)")]
    pub fn set_runtime_animator_controller(&self, value: Option<RuntimeAnimatorController>) {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> Option<AnimationClip> {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: Option<AnimationClip>) {}

    #[unity_method(name = "get_Item")]
    pub fn get_item_1(&self) -> Option<AnimationClip> {}

    #[unity_method(name = "set_Item")]
    pub fn set_item_1(&self, value: Option<AnimationClip>) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::get_overridesCount")]
    pub fn get_overrides_count(&self) -> i32 {}

    #[unity_method(name = "get_clips")]
    pub fn get_clips(&self) -> Array<AnimationClipPair> {}

    #[unity_method(name = "set_clips")]
    pub fn set_clips(&self, value: Array<AnimationClipPair>) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::Internal_Create(AnimatorOverrideController,RuntimeAnimatorController)")]
    pub fn internal_create(this: Option<AnimatorOverrideController>, controller: Option<RuntimeAnimatorController>) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::Internal_GetClipByName(System.String,System.Boolean)")]
    pub fn internal_get_clip_by_name(&self, name: &str, return_effective_clip: bool) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::Internal_SetClipByName(System.String,AnimationClip)")]
    pub fn internal_set_clip_by_name(&self, name: &str, clip: Option<AnimationClip>) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::GetClip(AnimationClip,System.Boolean)")]
    pub fn get_clip(&self, original_clip: Option<AnimationClip>, return_effective_clip: bool) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::SetClip(AnimationClip,AnimationClip,System.Boolean)")]
    pub fn set_clip(&self, original_clip: Option<AnimationClip>, override_clip: Option<AnimationClip>, notify: bool) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::SendNotification")]
    pub fn send_notification(&self) {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::GetOriginalClip(System.Int32)")]
    pub fn get_original_clip(&self, index: i32) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::GetOverrideClip(AnimationClip)")]
    pub fn get_override_clip(&self, original_clip: Option<AnimationClip>) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.AnimatorOverrideController::PerformOverrideClipListCleanup")]
    pub fn perform_override_clip_list_cleanup(&self) {}

}
