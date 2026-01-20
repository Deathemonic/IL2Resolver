#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::animation_clip::AnimationClip;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "RuntimeAnimatorController", namespace = "UnityEngine", inherit = "Object")]
pub struct RuntimeAnimatorController(pub *mut c_void);

#[unity_impl]
impl RuntimeAnimatorController {
    #[unity_icall("UnityEngine.RuntimeAnimatorController::get_animationClips")]
    pub fn get_animation_clips(&self) -> Array<AnimationClip> {}

}
