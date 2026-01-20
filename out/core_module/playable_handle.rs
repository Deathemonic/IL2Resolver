#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemType};
use super::director_wrap_mode::DirectorWrapMode;
use super::playable_graph::PlayableGraph;
use super::playable_traversal_mode::PlayableTraversalMode;
use super::play_state::PlayState;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PlayableHandle", namespace = "UnityEngine.Playables", value_type)]
pub struct PlayableHandle {
    pub m_handle: isize,
    pub m_version: u32,
}

#[unity_impl]
impl PlayableHandle {
    #[unity_method(name = "get_Null", static)]
    pub fn get_null() -> PlayableHandle {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, p: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: PlayableHandle) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPlayableType")]
    pub fn get_playable_type(&self) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetJobType")]
    pub fn get_job_type(&self) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetScriptInstance(System.Object)")]
    pub fn set_script_instance(&self, script_instance: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanChangeInputs")]
    pub fn can_change_inputs(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanSetWeights")]
    pub fn can_set_weights(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanDestroy")]
    pub fn can_destroy(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPlayState")]
    pub fn get_play_state(&self) -> PlayState {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::Play")]
    pub fn play(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::Pause")]
    pub fn pause(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetSpeed")]
    pub fn get_speed(&self) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetSpeed(System.Double)")]
    pub fn set_speed(&self, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTime")]
    pub fn get_time(&self) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTime(System.Double)")]
    pub fn set_time(&self, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::IsDone")]
    pub fn is_done(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDone(System.Boolean)")]
    pub fn set_done(&self, value: bool) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetDuration")]
    pub fn get_duration(&self) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDuration(System.Double)")]
    pub fn set_duration(&self, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPropagateSetTime")]
    pub fn get_propagate_set_time(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetPropagateSetTime(System.Boolean)")]
    pub fn set_propagate_set_time(&self, value: bool) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetGraph")]
    pub fn get_graph(&self) -> PlayableGraph {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputCount")]
    pub fn get_input_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputCount(System.Int32)")]
    pub fn set_input_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetOutputCount")]
    pub fn get_output_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetOutputCount(System.Int32)")]
    pub fn set_output_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputWeight(PlayableHandle,System.Single)")]
    pub fn set_input_weight(&self, input: PlayableHandle, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDelay(System.Double)")]
    pub fn set_delay(&self, delay: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetDelay")]
    pub fn get_delay(&self) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::IsDelayed")]
    pub fn is_delayed(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPreviousTime")]
    pub fn get_previous_time(&self) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetLeadTime(System.Single)")]
    pub fn set_lead_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetLeadTime")]
    pub fn get_lead_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTraversalMode")]
    pub fn get_traversal_mode(&self) -> PlayableTraversalMode {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTraversalMode(PlayableTraversalMode)")]
    pub fn set_traversal_mode(&self, mode: PlayableTraversalMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetJobData")]
    pub fn get_job_data(&self) -> isize {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTimeWrapMode")]
    pub fn get_time_wrap_mode(&self) -> DirectorWrapMode {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTimeWrapMode(DirectorWrapMode)")]
    pub fn set_time_wrap_mode(&self, mode: DirectorWrapMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetScriptInstance")]
    pub fn get_script_instance(&self) -> Option<SystemObject> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputHandle(System.Int32)")]
    pub fn get_input_handle(&self, index: i32) -> PlayableHandle {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetOutputHandle(System.Int32)")]
    pub fn get_output_handle(&self, index: i32) -> PlayableHandle {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputWeightFromIndex(System.Int32,System.Single)")]
    pub fn set_input_weight_from_index(&self, index: i32, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputWeightFromIndex(System.Int32)")]
    pub fn get_input_weight_from_index(&self, index: i32) -> f32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::IsNull_Injected(PlayableHandle&)")]
    pub fn is_null(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPlayableType_Injected(PlayableHandle&)")]
    pub fn get_playable_type_1(_unity_self: &mut PlayableHandle) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetJobType_Injected(PlayableHandle&)")]
    pub fn get_job_type_1(_unity_self: &mut PlayableHandle) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetScriptInstance_Injected(PlayableHandle&,System.Object)")]
    pub fn set_script_instance_1(_unity_self: &mut PlayableHandle, script_instance: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanChangeInputs_Injected(PlayableHandle&)")]
    pub fn can_change_inputs_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanSetWeights_Injected(PlayableHandle&)")]
    pub fn can_set_weights_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::CanDestroy_Injected(PlayableHandle&)")]
    pub fn can_destroy_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPlayState_Injected(PlayableHandle&)")]
    pub fn get_play_state_1(_unity_self: &mut PlayableHandle) -> PlayState {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::Play_Injected(PlayableHandle&)")]
    pub fn play_1(_unity_self: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::Pause_Injected(PlayableHandle&)")]
    pub fn pause_1(_unity_self: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetSpeed_Injected(PlayableHandle&)")]
    pub fn get_speed_1(_unity_self: &mut PlayableHandle) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetSpeed_Injected(PlayableHandle&,System.Double)")]
    pub fn set_speed_1(_unity_self: &mut PlayableHandle, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTime_Injected(PlayableHandle&)")]
    pub fn get_time_1(_unity_self: &mut PlayableHandle) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTime_Injected(PlayableHandle&,System.Double)")]
    pub fn set_time_1(_unity_self: &mut PlayableHandle, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::IsDone_Injected(PlayableHandle&)")]
    pub fn is_done_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDone_Injected(PlayableHandle&,System.Boolean)")]
    pub fn set_done_1(_unity_self: &mut PlayableHandle, value: bool) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetDuration_Injected(PlayableHandle&)")]
    pub fn get_duration_1(_unity_self: &mut PlayableHandle) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDuration_Injected(PlayableHandle&,System.Double)")]
    pub fn set_duration_1(_unity_self: &mut PlayableHandle, value: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPropagateSetTime_Injected(PlayableHandle&)")]
    pub fn get_propagate_set_time_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetPropagateSetTime_Injected(PlayableHandle&,System.Boolean)")]
    pub fn set_propagate_set_time_1(_unity_self: &mut PlayableHandle, value: bool) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetGraph_Injected(PlayableHandle&,PlayableGraph&)")]
    pub fn get_graph_1(_unity_self: &mut PlayableHandle, ret: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputCount_Injected(PlayableHandle&)")]
    pub fn get_input_count_1(_unity_self: &mut PlayableHandle) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputCount_Injected(PlayableHandle&,System.Int32)")]
    pub fn set_input_count_1(_unity_self: &mut PlayableHandle, value: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetOutputCount_Injected(PlayableHandle&)")]
    pub fn get_output_count_1(_unity_self: &mut PlayableHandle) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetOutputCount_Injected(PlayableHandle&,System.Int32)")]
    pub fn set_output_count_1(_unity_self: &mut PlayableHandle, value: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputWeight_Injected(PlayableHandle&,PlayableHandle&,System.Single)")]
    pub fn set_input_weight_1(_unity_self: &mut PlayableHandle, input: &mut PlayableHandle, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetDelay_Injected(PlayableHandle&,System.Double)")]
    pub fn set_delay_1(_unity_self: &mut PlayableHandle, delay: f64) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetDelay_Injected(PlayableHandle&)")]
    pub fn get_delay_1(_unity_self: &mut PlayableHandle) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::IsDelayed_Injected(PlayableHandle&)")]
    pub fn is_delayed_1(_unity_self: &mut PlayableHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetPreviousTime_Injected(PlayableHandle&)")]
    pub fn get_previous_time_1(_unity_self: &mut PlayableHandle) -> f64 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetLeadTime_Injected(PlayableHandle&,System.Single)")]
    pub fn set_lead_time_1(_unity_self: &mut PlayableHandle, value: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetLeadTime_Injected(PlayableHandle&)")]
    pub fn get_lead_time_1(_unity_self: &mut PlayableHandle) -> f32 {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTraversalMode_Injected(PlayableHandle&)")]
    pub fn get_traversal_mode_1(_unity_self: &mut PlayableHandle) -> PlayableTraversalMode {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTraversalMode_Injected(PlayableHandle&,PlayableTraversalMode)")]
    pub fn set_traversal_mode_1(_unity_self: &mut PlayableHandle, mode: PlayableTraversalMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetJobData_Injected(PlayableHandle&)")]
    pub fn get_job_data_1(_unity_self: &mut PlayableHandle) -> isize {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetTimeWrapMode_Injected(PlayableHandle&)")]
    pub fn get_time_wrap_mode_1(_unity_self: &mut PlayableHandle) -> DirectorWrapMode {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetTimeWrapMode_Injected(PlayableHandle&,DirectorWrapMode)")]
    pub fn set_time_wrap_mode_1(_unity_self: &mut PlayableHandle, mode: DirectorWrapMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetScriptInstance_Injected(PlayableHandle&)")]
    pub fn get_script_instance_1(_unity_self: &mut PlayableHandle) -> Option<SystemObject> {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputHandle_Injected(PlayableHandle&,System.Int32,PlayableHandle&)")]
    pub fn get_input_handle_1(_unity_self: &mut PlayableHandle, index: i32, ret: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetOutputHandle_Injected(PlayableHandle&,System.Int32,PlayableHandle&)")]
    pub fn get_output_handle_1(_unity_self: &mut PlayableHandle, index: i32, ret: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::SetInputWeightFromIndex_Injected(PlayableHandle&,System.Int32,System.Single)")]
    pub fn set_input_weight_from_index_1(_unity_self: &mut PlayableHandle, index: i32, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableHandle::GetInputWeightFromIndex_Injected(PlayableHandle&,System.Int32)")]
    pub fn get_input_weight_from_index_1(_unity_self: &mut PlayableHandle, index: i32) -> f32 {}

}
