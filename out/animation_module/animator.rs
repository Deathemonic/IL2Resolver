#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::{SystemObject, SystemString, SystemType};
use crate::mscorlib::collections::{Array};
use super::animator_clip_info::AnimatorClipInfo;
use super::animator_controller_parameter::AnimatorControllerParameter;
use super::animator_culling_mode::AnimatorCullingMode;
use super::animator_recorder_mode::AnimatorRecorderMode;
use super::animator_state_info::AnimatorStateInfo;
use super::animator_transition_info::AnimatorTransitionInfo;
use super::animator_update_mode::AnimatorUpdateMode;
use super::avatar::Avatar;
use super::avatar_ik_goal::AvatarIKGoal;
use super::avatar_ik_hint::AvatarIKHint;
use super::avatar_target::AvatarTarget;
use super::match_target_weight_mask::MatchTargetWeightMask;
use super::runtime_animator_controller::RuntimeAnimatorController;
use crate::core_module::{PlayableGraph, ScriptableObject, Transform};
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "Animator", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct Animator(pub *mut c_void);

#[unity_impl]
impl Animator {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Animator::get_isOptimizable")]
    pub fn get_is_optimizable(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_isHuman")]
    pub fn get_is_human(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_hasRootMotion")]
    pub fn get_has_root_motion(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_humanScale")]
    pub fn get_human_scale(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_isInitialized")]
    pub fn get_is_initialized(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_deltaPosition_Injected(Vector3&)")]
    pub fn get_delta_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_deltaRotation_Injected(Quaternion&)")]
    pub fn get_delta_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_velocity_Injected(Vector3&)")]
    pub fn get_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_angularVelocity_Injected(Vector3&)")]
    pub fn get_angular_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_rootPosition")]
    pub fn get_root_position(&self) -> Vector3 {}

    #[unity_icall("UnityEngine.Animator::set_rootPosition(Vector3)")]
    pub fn set_root_position(&self, value: Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_rootRotation")]
    pub fn get_root_rotation(&self) -> Quaternion {}

    #[unity_icall("UnityEngine.Animator::set_rootRotation(Quaternion)")]
    pub fn set_root_rotation(&self, value: Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_applyRootMotion")]
    pub fn get_apply_root_motion(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_applyRootMotion(System.Boolean)")]
    pub fn set_apply_root_motion(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_linearVelocityBlending")]
    pub fn get_linear_velocity_blending(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_linearVelocityBlending(System.Boolean)")]
    pub fn set_linear_velocity_blending(&self, value: bool) {}

    #[unity_method(name = "get_animatePhysics")]
    pub fn get_animate_physics(&self) -> bool {}

    #[unity_method(name = "set_animatePhysics")]
    pub fn set_animate_physics(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_updateMode")]
    pub fn get_update_mode(&self) -> AnimatorUpdateMode {}

    #[unity_icall("UnityEngine.Animator::set_updateMode(AnimatorUpdateMode)")]
    pub fn set_update_mode(&self, value: AnimatorUpdateMode) {}

    #[unity_icall("UnityEngine.Animator::get_hasTransformHierarchy")]
    pub fn get_has_transform_hierarchy(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_gravityWeight")]
    pub fn get_gravity_weight(&self) -> f32 {}

    #[unity_method(name = "get_bodyPosition")]
    pub fn get_body_position(&self) -> Vector3 {}

    #[unity_method(name = "set_bodyPosition")]
    pub fn set_body_position(&self, value: Vector3) {}

    #[unity_method(name = "get_bodyRotation")]
    pub fn get_body_rotation(&self) -> Quaternion {}

    #[unity_method(name = "set_bodyRotation")]
    pub fn set_body_rotation(&self, value: Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_stabilizeFeet")]
    pub fn get_stabilize_feet(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_stabilizeFeet(System.Boolean)")]
    pub fn set_stabilize_feet(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_layerCount")]
    pub fn get_layer_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animator::get_parameters")]
    pub fn get_parameters(&self) -> Array<AnimatorControllerParameter> {}

    #[unity_icall("UnityEngine.Animator::get_parameterCount")]
    pub fn get_parameter_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Animator::get_feetPivotActive")]
    pub fn get_feet_pivot_active(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::set_feetPivotActive(System.Single)")]
    pub fn set_feet_pivot_active(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animator::get_pivotWeight")]
    pub fn get_pivot_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_pivotPosition_Injected(Vector3&)")]
    pub fn get_pivot_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_isMatchingTarget")]
    pub fn get_is_matching_target(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_speed")]
    pub fn get_speed(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::set_speed(System.Single)")]
    pub fn set_speed(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animator::get_targetPosition_Injected(Vector3&)")]
    pub fn get_target_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_targetRotation_Injected(Quaternion&)")]
    pub fn get_target_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_cullingMode")]
    pub fn get_culling_mode(&self) -> AnimatorCullingMode {}

    #[unity_icall("UnityEngine.Animator::set_cullingMode(AnimatorCullingMode)")]
    pub fn set_culling_mode(&self, value: AnimatorCullingMode) {}

    #[unity_icall("UnityEngine.Animator::get_playbackTime")]
    pub fn get_playback_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::set_playbackTime(System.Single)")]
    pub fn set_playback_time(&self, value: f32) {}

    #[unity_method(name = "get_recorderStartTime")]
    pub fn get_recorder_start_time(&self) -> f32 {}

    #[unity_method(name = "set_recorderStartTime")]
    pub fn set_recorder_start_time(&self, value: f32) {}

    #[unity_method(name = "get_recorderStopTime")]
    pub fn get_recorder_stop_time(&self) -> f32 {}

    #[unity_method(name = "set_recorderStopTime")]
    pub fn set_recorder_stop_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.Animator::get_recorderMode")]
    pub fn get_recorder_mode(&self) -> AnimatorRecorderMode {}

    #[unity_icall("UnityEngine.Animator::get_runtimeAnimatorController")]
    pub fn get_runtime_animator_controller(&self) -> Option<RuntimeAnimatorController> {}

    #[unity_icall("UnityEngine.Animator::set_runtimeAnimatorController(RuntimeAnimatorController)")]
    pub fn set_runtime_animator_controller(&self, value: Option<RuntimeAnimatorController>) {}

    #[unity_icall("UnityEngine.Animator::get_hasBoundPlayables")]
    pub fn get_has_bound_playables(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_avatar")]
    pub fn get_avatar(&self) -> Option<Avatar> {}

    #[unity_icall("UnityEngine.Animator::set_avatar(Avatar)")]
    pub fn set_avatar(&self, value: Option<Avatar>) {}

    #[unity_method(name = "get_playableGraph")]
    pub fn get_playable_graph(&self) -> PlayableGraph {}

    #[unity_icall("UnityEngine.Animator::get_layersAffectMassCenter")]
    pub fn get_layers_affect_mass_center(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_layersAffectMassCenter(System.Boolean)")]
    pub fn set_layers_affect_mass_center(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_leftFeetBottomHeight")]
    pub fn get_left_feet_bottom_height(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_rightFeetBottomHeight")]
    pub fn get_right_feet_bottom_height(&self) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn get_log_warnings(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_logWarnings(System.Boolean)")]
    pub fn set_log_warnings(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_fireEvents")]
    pub fn get_fire_events(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_fireEvents(System.Boolean)")]
    pub fn set_fire_events(&self, value: bool) {}

    #[unity_method(name = "get_keepAnimatorControllerStateOnDisable")]
    pub fn get_keep_animator_controller_state_on_disable(&self) -> bool {}

    #[unity_method(name = "set_keepAnimatorControllerStateOnDisable")]
    pub fn set_keep_animator_controller_state_on_disable(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_keepAnimatorStateOnDisable")]
    pub fn get_keep_animator_state_on_disable(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_keepAnimatorStateOnDisable(System.Boolean)")]
    pub fn set_keep_animator_state_on_disable(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::get_writeDefaultValuesOnDisable")]
    pub fn get_write_default_values_on_disable(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::set_writeDefaultValuesOnDisable(System.Boolean)")]
    pub fn set_write_default_values_on_disable(&self, value: bool) {}

    #[unity_icall("UnityEngine.Animator::SetFloatString(System.String,System.Single)")]
    pub fn set_float(&self, name: &str, value: f32) {}

    #[unity_icall("UnityEngine.Animator::SetFloatStringDamp(System.String,System.Single,System.Single,System.Single)")]
    pub fn set_float_1(&self, name: &str, value: f32, damp_time: f32, delta_time: f32) {}

    #[unity_icall("UnityEngine.Animator::SetFloatID(System.Int32,System.Single)")]
    pub fn set_float_2(&self, id: i32, value: f32) {}

    #[unity_icall("UnityEngine.Animator::SetFloatIDDamp(System.Int32,System.Single,System.Single,System.Single)")]
    pub fn set_float_3(&self, id: i32, value: f32, damp_time: f32, delta_time: f32) {}

    #[unity_icall("UnityEngine.Animator::SetBoolString(System.String,System.Boolean)")]
    pub fn set_bool(&self, name: &str, value: bool) {}

    #[unity_icall("UnityEngine.Animator::SetBoolID(System.Int32,System.Boolean)")]
    pub fn set_bool_1(&self, id: i32, value: bool) {}

    #[unity_icall("UnityEngine.Animator::SetIntegerString(System.String,System.Int32)")]
    pub fn set_integer(&self, name: &str, value: i32) {}

    #[unity_icall("UnityEngine.Animator::SetIntegerID(System.Int32,System.Int32)")]
    pub fn set_integer_1(&self, id: i32, value: i32) {}

    #[unity_icall("UnityEngine.Animator::SetTriggerString(System.String)")]
    pub fn set_trigger(&self, name: &str) {}

    #[unity_icall("UnityEngine.Animator::SetTriggerID(System.Int32)")]
    pub fn set_trigger_1(&self, id: i32) {}

    #[unity_icall("UnityEngine.Animator::ResetTriggerString(System.String)")]
    pub fn reset_trigger(&self, name: &str) {}

    #[unity_icall("UnityEngine.Animator::ResetTriggerID(System.Int32)")]
    pub fn reset_trigger_1(&self, id: i32) {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_position(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_rotation(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetGoalWeightPosition(AvatarIKGoal)")]
    pub fn get_goal_weight_position(&self, goal: AvatarIKGoal) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_position_weight(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetGoalWeightPosition(AvatarIKGoal,System.Single)")]
    pub fn set_goal_weight_position(&self, goal: AvatarIKGoal, value: f32) {}

    #[unity_icall("UnityEngine.Animator::GetGoalWeightRotation(AvatarIKGoal)")]
    pub fn get_goal_weight_rotation(&self, goal: AvatarIKGoal) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_rotation_weight(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetGoalWeightRotation(AvatarIKGoal,System.Single)")]
    pub fn set_goal_weight_rotation(&self, goal: AvatarIKGoal, value: f32) {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_hint_position(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetHintWeightPosition(AvatarIKHint)")]
    pub fn get_hint_weight_position(&self, hint: AvatarIKHint) -> f32 {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_ik_hint_position_weight(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetHintWeightPosition(AvatarIKHint,System.Single)")]
    pub fn set_hint_weight_position(&self, hint: AvatarIKHint, value: f32) {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_position(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetLookAtPositionInternal(Vector3)")]
    pub fn set_look_at_position_internal(&self, look_at_position: Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_weight(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_weight_1(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_weight_2(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_weight_3(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_look_at_weight_4(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetLookAtWeightInternal(System.Single,System.Single,System.Single,System.Single,System.Single)")]
    pub fn set_look_at_weight_internal(&self, weight: f32, body_weight: f32, head_weight: f32, eyes_weight: f32, clamp_weight: f32) {}

    #[unity_icall("UnityEngine.Animator::get_logWarnings")]
    pub fn set_bone_local_rotation(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::SetBoneLocalRotationInternal(System.Int32,Quaternion)")]
    pub fn set_bone_local_rotation_internal(&self, human_bone_id: i32, rotation: Quaternion) {}

    #[unity_icall("UnityEngine.Animator::GetBehaviour(System.Type)")]
    pub fn get_behaviour(&self, type_ref: Option<SystemType>) -> Option<ScriptableObject> {}

    #[unity_icall("UnityEngine.Animator::InternalGetBehaviours(System.Type)")]
    pub fn internal_get_behaviours(&self, type_ref: Option<SystemType>) -> Array<ScriptableObject> {}

    #[unity_icall("UnityEngine.Animator::InternalGetBehavioursByKey(System.Int32,System.Int32,System.Type)")]
    pub fn internal_get_behaviours_by_key(&self, full_path_hash: i32, layer_index: i32, type_ref: Option<SystemType>) -> Array<ScriptableObject> {}

    #[unity_icall("UnityEngine.Animator::GetLayerName(System.Int32)")]
    pub fn get_layer_name(&self, layer_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Animator::GetLayerIndex(System.String)")]
    pub fn get_layer_index(&self, layer_name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::GetLayerWeight(System.Int32)")]
    pub fn get_layer_weight(&self, layer_index: i32) -> f32 {}

    #[unity_icall("UnityEngine.Animator::SetLayerWeight(System.Int32,System.Single)")]
    pub fn set_layer_weight(&self, layer_index: i32, weight: f32) {}

    #[unity_icall("UnityEngine.Animator::GetAnimatorStateInfo(System.Int32,StateInfoIndex,AnimatorStateInfo&)")]
    pub fn get_animator_state_info(&self, layer_index: i32, state_info_index: *mut c_void, info: &mut AnimatorStateInfo) {}

    #[unity_icall("UnityEngine.Animator::GetAnimatorTransitionInfo(System.Int32,AnimatorTransitionInfo&)")]
    pub fn get_animator_transition_info(&self, layer_index: i32, info: &mut AnimatorTransitionInfo) {}

    #[unity_icall("UnityEngine.Animator::GetAnimatorClipInfoCount(System.Int32,System.Boolean)")]
    pub fn get_animator_clip_info_count(&self, layer_index: i32, current: bool) -> i32 {}

    #[unity_icall("UnityEngine.Animator::GetCurrentAnimatorClipInfo(System.Int32)")]
    pub fn get_current_animator_clip_info(&self, layer_index: i32) -> Array<AnimatorClipInfo> {}

    #[unity_icall("UnityEngine.Animator::GetNextAnimatorClipInfo(System.Int32)")]
    pub fn get_next_animator_clip_info(&self, layer_index: i32) -> Array<AnimatorClipInfo> {}

    #[unity_icall("UnityEngine.Animator::GetAnimatorClipInfoInternal(System.Int32,System.Boolean,System.Object)")]
    pub fn get_animator_clip_info_internal(&self, layer_index: i32, is_current: bool, clips: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Animator::IsInTransition(System.Int32)")]
    pub fn is_in_transition(&self, layer_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetParameterInternal(System.Int32)")]
    pub fn get_parameter_internal(&self, index: i32) -> Option<AnimatorControllerParameter> {}

    #[unity_icall("UnityEngine.Animator::MatchTarget_Injected(Vector3&,Quaternion&,System.Int32,MatchTargetWeightMask&,System.Single,System.Single,System.Boolean)")]
    pub fn match_target(&self, match_position: &mut Vector3, match_rotation: &mut Quaternion, target_body_part: i32, weight_mask: &mut MatchTargetWeightMask, start_normalized_time: f32, target_normalized_time: f32, complete_match: bool) {}

    #[unity_icall("UnityEngine.Animator::MatchTarget_Injected(Vector3&,Quaternion&,System.Int32,MatchTargetWeightMask&,System.Single,System.Single,System.Boolean)")]
    pub fn match_target_1(&self, match_position: &mut Vector3, match_rotation: &mut Quaternion, target_body_part: i32, weight_mask: &mut MatchTargetWeightMask, start_normalized_time: f32, target_normalized_time: f32, complete_match: bool) {}

    #[unity_icall("UnityEngine.Animator::MatchTarget_Injected(Vector3&,Quaternion&,System.Int32,MatchTargetWeightMask&,System.Single,System.Single,System.Boolean)")]
    pub fn match_target_2(&self, match_position: &mut Vector3, match_rotation: &mut Quaternion, target_body_part: i32, weight_mask: &mut MatchTargetWeightMask, start_normalized_time: f32, target_normalized_time: f32, complete_match: bool) {}

    #[unity_icall("UnityEngine.Animator::InterruptMatchTarget(System.Boolean)")]
    pub fn interrupt_match_target(&self, complete_match: bool) {}

    #[unity_icall("UnityEngine.Animator::Play(System.Int32,System.Int32,System.Single)")]
    pub fn force_state_normalized_time(&self, state_name_hash: i32, layer: i32, normalized_time: f32) {}

    #[unity_icall("UnityEngine.Animator::StringToHash(System.String)")]
    pub fn cross_fade_in_fixed_time(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::CrossFadeInFixedTime(System.Int32,System.Single,System.Int32,System.Single,System.Single)")]
    pub fn cross_fade_in_fixed_time_1(&self, state_hash_name: i32, fixed_transition_duration: f32, layer: i32, fixed_time_offset: f32, normalized_transition_time: f32) {}

    #[unity_icall("UnityEngine.Animator::WriteDefaultValues")]
    pub fn write_default_values(&self) {}

    #[unity_icall("UnityEngine.Animator::StringToHash(System.String)")]
    pub fn cross_fade(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::CrossFade(System.Int32,System.Single,System.Int32,System.Single,System.Single)")]
    pub fn cross_fade_1(&self, state_hash_name: i32, normalized_transition_duration: f32, layer: i32, normalized_time_offset: f32, normalized_transition_time: f32) {}

    #[unity_icall("UnityEngine.Animator::StringToHash(System.String)")]
    pub fn play_in_fixed_time(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::PlayInFixedTime(System.Int32,System.Int32,System.Single)")]
    pub fn play_in_fixed_time_1(&self, state_name_hash: i32, layer: i32, fixed_time: f32) {}

    #[unity_icall("UnityEngine.Animator::StringToHash(System.String)")]
    pub fn play(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::SetTarget(AvatarTarget,System.Single)")]
    pub fn set_target(&self, target_index: AvatarTarget, target_normalized_time: f32) {}

    #[unity_method(name = "IsControlled")]
    pub fn is_controlled(&self, transform: Option<Transform>) -> bool {}

    #[unity_icall("UnityEngine.Animator::IsBoneTransform(Transform)")]
    pub fn is_bone_transform(&self, transform: Option<Transform>) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetBoneTransformInternal(System.Int32)")]
    pub fn get_bone_transform_internal(&self, human_bone_id: i32) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Animator::StartPlayback")]
    pub fn start_playback(&self) {}

    #[unity_icall("UnityEngine.Animator::StopPlayback")]
    pub fn stop_playback(&self) {}

    #[unity_icall("UnityEngine.Animator::StartRecording(System.Int32)")]
    pub fn start_recording(&self, frame_count: i32) {}

    #[unity_icall("UnityEngine.Animator::StopRecording")]
    pub fn stop_recording(&self) {}

    #[unity_icall("UnityEngine.Animator::ClearInternalControllerPlayable")]
    pub fn clear_internal_controller_playable(&self) {}

    #[unity_icall("UnityEngine.Animator::HasState(System.Int32,System.Int32)")]
    pub fn has_state(&self, layer_index: i32, state_id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetStats")]
    pub fn get_stats(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Animator::GetCurrentGraph(PlayableGraph&)")]
    pub fn get_current_graph(&self, graph: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Animator::IsInIKPass")]
    pub fn is_in_ik_pass(&self) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetFloatString(System.String)")]
    pub fn get_float_string(&self, name: &str) -> f32 {}

    #[unity_icall("UnityEngine.Animator::GetFloatID(System.Int32)")]
    pub fn get_float_id(&self, id: i32) -> f32 {}

    #[unity_icall("UnityEngine.Animator::GetBoolString(System.String)")]
    pub fn get_bool_string(&self, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetBoolID(System.Int32)")]
    pub fn get_bool_id(&self, id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animator::GetIntegerString(System.String)")]
    pub fn get_integer_string(&self, name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Animator::GetIntegerID(System.Int32)")]
    pub fn get_integer_id(&self, id: i32) -> i32 {}

    #[unity_icall("UnityEngine.Animator::IsParameterControlledByCurveString(System.String)")]
    pub fn is_parameter_controlled_by_curve_string(&self, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Animator::IsParameterControlledByCurveID(System.Int32)")]
    pub fn is_parameter_controlled_by_curve_id(&self, id: i32) -> bool {}

    #[unity_icall("UnityEngine.Animator::OnUpdateModeChanged")]
    pub fn on_update_mode_changed(&self) {}

    #[unity_icall("UnityEngine.Animator::OnCullingModeChanged")]
    pub fn on_culling_mode_changed(&self) {}

    #[unity_icall("UnityEngine.Animator::WriteDefaultPose")]
    pub fn write_default_pose(&self) {}

    #[unity_icall("UnityEngine.Animator::Update(System.Single)")]
    pub fn update(&self, delta_time: f32) {}

    #[unity_icall("UnityEngine.Animator::Rebind(System.Boolean)")]
    pub fn rebind(&self, write_default_values: bool) {}

    #[unity_icall("UnityEngine.Animator::ApplyBuiltinRootMotion")]
    pub fn apply_builtin_root_motion(&self) {}

    #[unity_icall("UnityEngine.Animator::EvaluateController(System.Single)")]
    pub fn evaluate_controller(&self, delta_time: f32) {}

    #[unity_icall("UnityEngine.Animator::GetAnimatorStateName(System.Int32,System.Boolean)")]
    pub fn get_animator_state_name(&self, layer_index: i32, current: bool) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Animator::ResolveHash(System.Int32)")]
    pub fn resolve_hash(&self, hash: i32) -> Option<SystemString> {}

    #[unity_method(name = "GetVector")]
    pub fn get_vector(&self, name: &str) -> Vector3 {}

    #[unity_method(name = "GetVector")]
    pub fn get_vector_1(&self, id: i32) -> Vector3 {}

    #[unity_method(name = "SetVector")]
    pub fn set_vector(&self, name: &str, value: Vector3) {}

    #[unity_method(name = "SetVector")]
    pub fn set_vector_1(&self, id: i32, value: Vector3) {}

    #[unity_method(name = "GetQuaternion")]
    pub fn get_quaternion(&self, name: &str) -> Quaternion {}

    #[unity_method(name = "GetQuaternion")]
    pub fn get_quaternion_1(&self, id: i32) -> Quaternion {}

    #[unity_method(name = "SetQuaternion")]
    pub fn set_quaternion(&self, name: &str, value: Quaternion) {}

    #[unity_method(name = "SetQuaternion")]
    pub fn set_quaternion_1(&self, id: i32, value: Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_rootPosition_Injected(Vector3&)")]
    pub fn get_root_position_1(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_rootRotation_Injected(Quaternion&)")]
    pub fn get_root_rotation_1(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::get_bodyPositionInternal_Injected(Vector3&)")]
    pub fn get_body_position_internal(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::set_bodyPositionInternal_Injected(Vector3&)")]
    pub fn set_body_position_internal(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::get_bodyRotationInternal_Injected(Quaternion&)")]
    pub fn get_body_rotation_internal(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::set_bodyRotationInternal_Injected(Quaternion&)")]
    pub fn set_body_rotation_internal(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::GetGoalPosition_Injected(AvatarIKGoal,Vector3&)")]
    pub fn get_goal_position(&self, goal: AvatarIKGoal, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::SetGoalPosition_Injected(AvatarIKGoal,Vector3&)")]
    pub fn set_goal_position(&self, goal: AvatarIKGoal, goal_position: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::GetGoalRotation_Injected(AvatarIKGoal,Quaternion&)")]
    pub fn get_goal_rotation(&self, goal: AvatarIKGoal, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::SetGoalRotation_Injected(AvatarIKGoal,Quaternion&)")]
    pub fn set_goal_rotation(&self, goal: AvatarIKGoal, goal_rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Animator::GetHintPosition_Injected(AvatarIKHint,Vector3&)")]
    pub fn get_hint_position(&self, hint: AvatarIKHint, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::SetHintPosition_Injected(AvatarIKHint,Vector3&)")]
    pub fn set_hint_position(&self, hint: AvatarIKHint, hint_position: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::SetLookAtPositionInternal_Injected(Vector3&)")]
    pub fn set_look_at_position_internal_1(&self, look_at_position: &mut Vector3) {}

    #[unity_icall("UnityEngine.Animator::SetBoneLocalRotationInternal_Injected(System.Int32,Quaternion&)")]
    pub fn set_bone_local_rotation_internal_1(&self, human_bone_id: i32, rotation: &mut Quaternion) {}

}
