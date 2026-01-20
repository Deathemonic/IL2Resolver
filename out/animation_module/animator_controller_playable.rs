#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::animator_clip_info::AnimatorClipInfo;
use super::animator_controller_parameter::AnimatorControllerParameter;
use super::animator_state_info::AnimatorStateInfo;
use super::animator_transition_info::AnimatorTransitionInfo;
use super::runtime_animator_controller::RuntimeAnimatorController;
use crate::core_module::{PlayableGraph, PlayableHandle};

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimatorControllerPlayable", namespace = "UnityEngine.Animations", value_type)]
pub struct AnimatorControllerPlayable {
    pub m_handle: PlayableHandle,
}

#[unity_impl]
impl AnimatorControllerPlayable {
    #[unity_method(name = "get_Null", static)]
    pub fn get_null() -> AnimatorControllerPlayable {}

    #[unity_method(name = "GetHandle")]
    pub fn get_handle(&self) -> PlayableHandle {}

    #[unity_method(name = "SetHandle")]
    pub fn set_handle(&self, handle: PlayableHandle) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: AnimatorControllerPlayable) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetFloatString(PlayableHandle&,System.String,System.Single)")]
    pub fn set_float(handle: &mut PlayableHandle, name: &str, value: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetFloatID(PlayableHandle&,System.Int32,System.Single)")]
    pub fn set_float_1(handle: &mut PlayableHandle, id: i32, value: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetBoolString(PlayableHandle&,System.String,System.Boolean)")]
    pub fn set_bool(handle: &mut PlayableHandle, name: &str, value: bool) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetBoolID(PlayableHandle&,System.Int32,System.Boolean)")]
    pub fn set_bool_1(handle: &mut PlayableHandle, id: i32, value: bool) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetIntegerString(PlayableHandle&,System.String,System.Int32)")]
    pub fn set_integer(handle: &mut PlayableHandle, name: &str, value: i32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetIntegerID(PlayableHandle&,System.Int32,System.Int32)")]
    pub fn set_integer_1(handle: &mut PlayableHandle, id: i32, value: i32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetTriggerString(PlayableHandle&,System.String)")]
    pub fn set_trigger(handle: &mut PlayableHandle, name: &str) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetTriggerID(PlayableHandle&,System.Int32)")]
    pub fn set_trigger_1(handle: &mut PlayableHandle, id: i32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::ResetTriggerString(PlayableHandle&,System.String)")]
    pub fn reset_trigger(handle: &mut PlayableHandle, name: &str) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::ResetTriggerID(PlayableHandle&,System.Int32)")]
    pub fn reset_trigger_1(handle: &mut PlayableHandle, id: i32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::SetLayerWeightInternal(PlayableHandle&,System.Int32,System.Single)")]
    pub fn set_layer_weight(handle: &mut PlayableHandle, layer_index: i32, weight: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetAnimatorClipInfoInternal(PlayableHandle&,System.Int32,System.Boolean,System.Object)")]
    pub fn get_animator_clip_info_internal(handle: &mut PlayableHandle, layer_index: i32, is_current: bool, clips: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade_in_fixed_time(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade_in_fixed_time_1(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade_in_fixed_time_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInFixedTimeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_in_fixed_time_3(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInFixedTimeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_in_fixed_time_4(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInFixedTimeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_in_fixed_time_5(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade_1(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn cross_fade_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_3(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_4(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CrossFadeInternal(PlayableHandle&,System.Int32,System.Single,System.Int32,System.Single)")]
    pub fn cross_fade_5(handle: &mut PlayableHandle, state_name_hash: i32, transition_duration: f32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play_in_fixed_time(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play_in_fixed_time_1(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play_in_fixed_time_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInFixedTimeInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_in_fixed_time_3(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInFixedTimeInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_in_fixed_time_4(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInFixedTimeInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_in_fixed_time_5(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play_1(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::StringToHash(System.String)")]
    pub fn play_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_3(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_4(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::PlayInternal(PlayableHandle&,System.Int32,System.Int32,System.Single)")]
    pub fn play_5(handle: &mut PlayableHandle, state_name_hash: i32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetAnimatorControllerInternal(PlayableHandle&)")]
    pub fn get_animator_controller_internal(handle: &mut PlayableHandle) -> Option<RuntimeAnimatorController> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetLayerCountInternal(PlayableHandle&)")]
    pub fn get_layer_count_internal(handle: &mut PlayableHandle) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetLayerNameInternal(PlayableHandle&,System.Int32)")]
    pub fn get_layer_name_internal(handle: &mut PlayableHandle, layer_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetLayerIndexInternal(PlayableHandle&,System.String)")]
    pub fn get_layer_index_internal(handle: &mut PlayableHandle, layer_name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetLayerWeightInternal(PlayableHandle&,System.Int32)")]
    pub fn get_layer_weight_internal(handle: &mut PlayableHandle, layer_index: i32) -> f32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetCurrentAnimatorClipInfoInternal(PlayableHandle&,System.Int32)")]
    pub fn get_current_animator_clip_info_internal(handle: &mut PlayableHandle, layer_index: i32) -> Array<AnimatorClipInfo> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetAnimatorClipInfoCountInternal(PlayableHandle&,System.Int32,System.Boolean)")]
    pub fn get_animator_clip_info_count_internal(handle: &mut PlayableHandle, layer_index: i32, current: bool) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetNextAnimatorClipInfoInternal(PlayableHandle&,System.Int32)")]
    pub fn get_next_animator_clip_info_internal(handle: &mut PlayableHandle, layer_index: i32) -> Array<AnimatorClipInfo> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::ResolveHashInternal(PlayableHandle&,System.Int32)")]
    pub fn resolve_hash_internal(handle: &mut PlayableHandle, hash: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::IsInTransitionInternal(PlayableHandle&,System.Int32)")]
    pub fn is_in_transition_internal(handle: &mut PlayableHandle, layer_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetParametersArrayInternal(PlayableHandle&)")]
    pub fn get_parameters_array_internal(handle: &mut PlayableHandle) -> Array<AnimatorControllerParameter> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetParameterInternal(PlayableHandle&,System.Int32)")]
    pub fn get_parameter_internal(handle: &mut PlayableHandle, index: i32) -> Option<AnimatorControllerParameter> {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetParameterCountInternal(PlayableHandle&)")]
    pub fn get_parameter_count_internal(handle: &mut PlayableHandle) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::HasStateInternal(PlayableHandle&,System.Int32,System.Int32)")]
    pub fn has_state_internal(handle: &mut PlayableHandle, layer_index: i32, state_id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetFloatString(PlayableHandle&,System.String)")]
    pub fn get_float_string(handle: &mut PlayableHandle, name: &str) -> f32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetFloatID(PlayableHandle&,System.Int32)")]
    pub fn get_float_id(handle: &mut PlayableHandle, id: i32) -> f32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetBoolString(PlayableHandle&,System.String)")]
    pub fn get_bool_string(handle: &mut PlayableHandle, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetBoolID(PlayableHandle&,System.Int32)")]
    pub fn get_bool_id(handle: &mut PlayableHandle, id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetIntegerString(PlayableHandle&,System.String)")]
    pub fn get_integer_string(handle: &mut PlayableHandle, name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetIntegerID(PlayableHandle&,System.Int32)")]
    pub fn get_integer_id(handle: &mut PlayableHandle, id: i32) -> i32 {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::IsParameterControlledByCurveString(PlayableHandle&,System.String)")]
    pub fn is_parameter_controlled_by_curve_string(handle: &mut PlayableHandle, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::IsParameterControlledByCurveID(PlayableHandle&,System.Int32)")]
    pub fn is_parameter_controlled_by_curve_id(handle: &mut PlayableHandle, id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::CreateHandleInternal_Injected(PlayableGraph&,RuntimeAnimatorController,PlayableHandle&)")]
    pub fn create_handle_internal(graph: &mut PlayableGraph, controller: Option<RuntimeAnimatorController>, handle: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetCurrentAnimatorStateInfoInternal_Injected(PlayableHandle&,System.Int32,AnimatorStateInfo&)")]
    pub fn get_current_animator_state_info_internal(handle: &mut PlayableHandle, layer_index: i32, ret: &mut AnimatorStateInfo) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetNextAnimatorStateInfoInternal_Injected(PlayableHandle&,System.Int32,AnimatorStateInfo&)")]
    pub fn get_next_animator_state_info_internal(handle: &mut PlayableHandle, layer_index: i32, ret: &mut AnimatorStateInfo) {}

    #[unity_icall("UnityEngine.Animations.AnimatorControllerPlayable::GetAnimatorTransitionInfoInternal_Injected(PlayableHandle&,System.Int32,AnimatorTransitionInfo&)")]
    pub fn get_animator_transition_info_internal(handle: &mut PlayableHandle, layer_index: i32, ret: &mut AnimatorTransitionInfo) {}

}
