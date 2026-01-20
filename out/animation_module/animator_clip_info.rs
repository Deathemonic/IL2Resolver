#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::animation_clip::AnimationClip;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorClipInfo", namespace = "UnityEngine", value_type)]
pub struct AnimatorClipInfo {
    pub m_clip_instance_id: i32,
    pub m_weight: f32,
}

#[unity_impl]
impl AnimatorClipInfo {
    #[unity_method(name = "get_clip")]
    pub fn get_clip(&self) -> Option<AnimationClip> {}

    #[unity_method(name = "get_weight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.AnimatorClipInfo::InstanceIDToAnimationClipPPtr(System.Int32)")]
    pub fn instance_id_to_animation_clip_p_ptr(instance_id: i32) -> Option<AnimationClip> {}

}
