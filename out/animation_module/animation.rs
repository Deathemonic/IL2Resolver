#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::animation_clip::AnimationClip;
use super::animation_culling_type::AnimationCullingType;
use super::animation_state::AnimationState;
use super::play_mode::PlayMode;
use super::queue_mode::QueueMode;
use crate::core_module::{Bounds, WrapMode};
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "Animation", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct Animation(pub *mut c_void);

#[unity_impl]
impl Animation {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Animation::get_clip")]
    pub fn get_clip(&self) -> Option<AnimationClip> {}

    #[unity_icall("UnityEngine.Animation::set_clip(AnimationClip)")]
    pub fn set_clip(&self, value: Option<AnimationClip>) {}

    #[unity_icall("UnityEngine.Animation::get_playAutomatically")]
    pub fn get_play_automatically(&self) -> bool {}

    #[unity_icall("UnityEngine.Animation::set_playAutomatically(System.Boolean)")]
    pub fn set_play_automatically(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animation::get_wrapMode")]
    pub fn get_wrap_mode(&self) -> WrapMode {}

    #[unity_icall("UnityEngine.Animation::set_wrapMode(WrapMode)")]
    pub fn set_wrap_mode(&self, value: WrapMode) {}

    #[unity_icall("UnityEngine.Animation::get_isPlaying")]
    pub fn get_is_playing(&self) -> bool {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> Option<AnimationState> {}

    #[unity_icall("UnityEngine.Animation::get_animatePhysics")]
    pub fn get_animate_physics(&self) -> bool {}

    #[unity_icall("UnityEngine.Animation::set_animatePhysics(System.Boolean)")]
    pub fn set_animate_physics(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animation::get_animateOnlyIfVisible")]
    pub fn get_animate_only_if_visible(&self) -> bool {}

    #[unity_icall("UnityEngine.Animation::set_animateOnlyIfVisible(System.Boolean)")]
    pub fn set_animate_only_if_visible(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animation::get_cullingType")]
    pub fn get_culling_type(&self) -> AnimationCullingType {}

    #[unity_icall("UnityEngine.Animation::set_cullingType(AnimationCullingType)")]
    pub fn set_culling_type(&self, value: AnimationCullingType) {}

    #[unity_icall("UnityEngine.Animation::get_localBounds_Injected(Bounds&)")]
    pub fn get_local_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Animation::set_localBounds_Injected(Bounds&)")]
    pub fn set_local_bounds(&self, value: &mut Bounds) {}

    #[unity_icall("UnityEngine.Animation::Stop")]
    pub fn stop(&self) {}

    #[unity_icall("UnityEngine.Animation::StopNamed(System.String)")]
    pub fn stop_1(&self, name: &str) {}

    #[unity_icall("UnityEngine.Animation::Rewind")]
    pub fn rewind(&self) {}

    #[unity_icall("UnityEngine.Animation::RewindNamed(System.String)")]
    pub fn rewind_1(&self, name: &str) {}

    #[unity_icall("UnityEngine.Animation::Sample")]
    pub fn sample(&self) {}

    #[unity_icall("UnityEngine.Animation::IsPlaying(System.String)")]
    pub fn is_playing(&self, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Animation::PlayDefaultAnimation(PlayMode)")]
    pub fn play_default_animation(&self, mode: PlayMode) -> bool {}

    #[unity_icall("UnityEngine.Animation::Play(System.String,PlayMode)")]
    pub fn play(&self, animation: &str, mode: PlayMode) -> bool {}

    #[unity_icall("UnityEngine.Animation::CrossFade(System.String,System.Single,PlayMode)")]
    pub fn cross_fade(&self, animation: &str, fade_length: f32, mode: PlayMode) {}

    #[unity_icall("UnityEngine.Animation::CrossFade(System.String,System.Single,PlayMode)")]
    pub fn cross_fade_1(&self, animation: &str, fade_length: f32, mode: PlayMode) {}

    #[unity_icall("UnityEngine.Animation::Blend(System.String,System.Single,System.Single)")]
    pub fn blend(&self, animation: &str, target_weight: f32, fade_length: f32) {}

    #[unity_icall("UnityEngine.Animation::Blend(System.String,System.Single,System.Single)")]
    pub fn blend_1(&self, animation: &str, target_weight: f32, fade_length: f32) {}

    #[unity_icall("UnityEngine.Animation::CrossFadeQueued(System.String,System.Single,QueueMode,PlayMode)")]
    pub fn cross_fade_queued(&self, animation: &str, fade_length: f32, queue: QueueMode, mode: PlayMode) -> Option<AnimationState> {}

    #[unity_icall("UnityEngine.Animation::PlayQueued(System.String,QueueMode,PlayMode)")]
    pub fn play_queued(&self, animation: &str, queue: QueueMode, mode: PlayMode) -> Option<AnimationState> {}

    #[unity_icall("UnityEngine.Animation::AddClip(AnimationClip,System.String,System.Int32,System.Int32,System.Boolean)")]
    pub fn add_clip(&self, clip: Option<AnimationClip>, new_name: &str, first_frame: i32, last_frame: i32, add_loop_frame: bool) {}

    #[unity_icall("UnityEngine.Animation::AddClip(AnimationClip,System.String,System.Int32,System.Int32,System.Boolean)")]
    pub fn add_clip_1(&self, clip: Option<AnimationClip>, new_name: &str, first_frame: i32, last_frame: i32, add_loop_frame: bool) {}

    #[unity_icall("UnityEngine.Animation::RemoveClip(AnimationClip)")]
    pub fn remove_clip(&self, clip: Option<AnimationClip>) {}

    #[unity_icall("UnityEngine.Animation::RemoveClipNamed(System.String)")]
    pub fn remove_clip_1(&self, clip_name: &str) {}

    #[unity_icall("UnityEngine.Animation::GetClipCount")]
    pub fn get_clip_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animation::SyncLayer(System.Int32)")]
    pub fn sync_layer(&self, layer: i32) {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> *mut c_void {}

    #[unity_icall("UnityEngine.Animation::GetState(System.String)")]
    pub fn get_state(&self, name: &str) -> Option<AnimationState> {}

    #[unity_icall("UnityEngine.Animation::GetStateAtIndex(System.Int32)")]
    pub fn get_state_at_index(&self, index: i32) -> Option<AnimationState> {}

    #[unity_icall("UnityEngine.Animation::GetStateCount")]
    pub fn get_state_count(&self) -> i32 {}

}
