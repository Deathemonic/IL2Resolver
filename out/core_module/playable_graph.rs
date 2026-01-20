#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemType};
use super::director_update_mode::DirectorUpdateMode;
use super::playable_handle::PlayableHandle;
use super::playable_output_handle::PlayableOutputHandle;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PlayableGraph", namespace = "UnityEngine.Playables", value_type)]
pub struct PlayableGraph {
    pub m_handle: isize,
    pub m_version: u32,
}

#[unity_impl]
impl PlayableGraph {
    #[unity_icall("UnityEngine.Playables.PlayableGraph::Evaluate(System.Single)")]
    pub fn evaluate(&self, delta_time: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Destroy")]
    pub fn destroy(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsPlaying")]
    pub fn is_playing(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsDone")]
    pub fn is_done(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Play")]
    pub fn play(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Stop")]
    pub fn stop(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetTimeUpdateMode")]
    pub fn get_time_update_mode(&self) -> DirectorUpdateMode {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SetTimeUpdateMode(DirectorUpdateMode)")]
    pub fn set_time_update_mode(&self, value: DirectorUpdateMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetResolver")]
    pub fn get_resolver(&self) -> *mut c_void {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SetResolver(IExposedPropertyTable)")]
    pub fn set_resolver(&self, value: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetPlayableCount")]
    pub fn get_playable_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetRootPlayableCount")]
    pub fn get_root_playable_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SynchronizeEvaluation(PlayableGraph)")]
    pub fn synchronize_evaluation(&self, playable: PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputCount")]
    pub fn get_output_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::CreatePlayableHandle")]
    pub fn create_playable_handle(&self) -> PlayableHandle {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::CreateScriptOutputInternal(System.String,PlayableOutputHandle&)")]
    pub fn create_script_output_internal(&self, name: &str, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetRootPlayableInternal(System.Int32)")]
    pub fn get_root_playable_internal(&self, index: i32) -> PlayableHandle {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroyOutputInternal(PlayableOutputHandle)")]
    pub fn destroy_output_internal(&self, handle: PlayableOutputHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsMatchFrameRateEnabled")]
    pub fn is_match_frame_rate_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::EnableMatchFrameRate(FrameRate)")]
    pub fn enable_match_frame_rate(&self, frame_rate: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DisableMatchFrameRate")]
    pub fn disable_match_frame_rate(&self) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputInternal(System.Int32,PlayableOutputHandle&)")]
    pub fn get_output_internal(&self, index: i32, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputCountByTypeInternal(System.Type)")]
    pub fn get_output_count_by_type_internal(&self, output_type: Option<SystemType>) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputByTypeInternal(System.Type,System.Int32,PlayableOutputHandle&)")]
    pub fn get_output_by_type_internal(&self, output_type: Option<SystemType>, index: i32, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::ConnectInternal(PlayableHandle,System.Int32,PlayableHandle,System.Int32)")]
    pub fn connect_internal(&self, source: PlayableHandle, source_output_port: i32, destination: PlayableHandle, destination_input_port: i32) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DisconnectInternal(PlayableHandle,System.Int32)")]
    pub fn disconnect_internal(&self, playable: PlayableHandle, input_port: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroyPlayableInternal(PlayableHandle)")]
    pub fn destroy_playable_internal(&self, playable: PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroySubgraphInternal(PlayableHandle)")]
    pub fn destroy_subgraph_internal(&self, playable: PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Create_Injected(System.String,PlayableGraph&)")]
    pub fn create(name: &str, ret: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Destroy_Injected(PlayableGraph&)")]
    pub fn destroy_1(_unity_self: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsPlaying_Injected(PlayableGraph&)")]
    pub fn is_playing_1(_unity_self: &mut PlayableGraph) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsDone_Injected(PlayableGraph&)")]
    pub fn is_done_1(_unity_self: &mut PlayableGraph) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Play_Injected(PlayableGraph&)")]
    pub fn play_1(_unity_self: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Stop_Injected(PlayableGraph&)")]
    pub fn stop_1(_unity_self: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::Evaluate_Injected(PlayableGraph&,System.Single)")]
    pub fn evaluate_1(_unity_self: &mut PlayableGraph, delta_time: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetTimeUpdateMode_Injected(PlayableGraph&)")]
    pub fn get_time_update_mode_1(_unity_self: &mut PlayableGraph) -> DirectorUpdateMode {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SetTimeUpdateMode_Injected(PlayableGraph&,DirectorUpdateMode)")]
    pub fn set_time_update_mode_1(_unity_self: &mut PlayableGraph, value: DirectorUpdateMode) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetResolver_Injected(PlayableGraph&)")]
    pub fn get_resolver_1(_unity_self: &mut PlayableGraph) -> *mut c_void {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SetResolver_Injected(PlayableGraph&,IExposedPropertyTable)")]
    pub fn set_resolver_1(_unity_self: &mut PlayableGraph, value: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetPlayableCount_Injected(PlayableGraph&)")]
    pub fn get_playable_count_1(_unity_self: &mut PlayableGraph) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetRootPlayableCount_Injected(PlayableGraph&)")]
    pub fn get_root_playable_count_1(_unity_self: &mut PlayableGraph) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::SynchronizeEvaluation_Injected(PlayableGraph&,PlayableGraph&)")]
    pub fn synchronize_evaluation_1(_unity_self: &mut PlayableGraph, playable: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputCount_Injected(PlayableGraph&)")]
    pub fn get_output_count_1(_unity_self: &mut PlayableGraph) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::CreatePlayableHandle_Injected(PlayableGraph&,PlayableHandle&)")]
    pub fn create_playable_handle_1(_unity_self: &mut PlayableGraph, ret: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::CreateScriptOutputInternal_Injected(PlayableGraph&,System.String,PlayableOutputHandle&)")]
    pub fn create_script_output_internal_1(_unity_self: &mut PlayableGraph, name: &str, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetRootPlayableInternal_Injected(PlayableGraph&,System.Int32,PlayableHandle&)")]
    pub fn get_root_playable_internal_1(_unity_self: &mut PlayableGraph, index: i32, ret: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroyOutputInternal_Injected(PlayableGraph&,PlayableOutputHandle&)")]
    pub fn destroy_output_internal_1(_unity_self: &mut PlayableGraph, handle: &mut PlayableOutputHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::IsMatchFrameRateEnabled_Injected(PlayableGraph&)")]
    pub fn is_match_frame_rate_enabled_1(_unity_self: &mut PlayableGraph) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::EnableMatchFrameRate_Injected(PlayableGraph&,FrameRate&)")]
    pub fn enable_match_frame_rate_1(_unity_self: &mut PlayableGraph, frame_rate: &mut *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DisableMatchFrameRate_Injected(PlayableGraph&)")]
    pub fn disable_match_frame_rate_1(_unity_self: &mut PlayableGraph) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetFrameRate_Injected(PlayableGraph&,FrameRate&)")]
    pub fn get_frame_rate(_unity_self: &mut PlayableGraph, ret: &mut *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputInternal_Injected(PlayableGraph&,System.Int32,PlayableOutputHandle&)")]
    pub fn get_output_internal_1(_unity_self: &mut PlayableGraph, index: i32, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputCountByTypeInternal_Injected(PlayableGraph&,System.Type)")]
    pub fn get_output_count_by_type_internal_1(_unity_self: &mut PlayableGraph, output_type: Option<SystemType>) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::GetOutputByTypeInternal_Injected(PlayableGraph&,System.Type,System.Int32,PlayableOutputHandle&)")]
    pub fn get_output_by_type_internal_1(_unity_self: &mut PlayableGraph, output_type: Option<SystemType>, index: i32, handle: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::ConnectInternal_Injected(PlayableGraph&,PlayableHandle&,System.Int32,PlayableHandle&,System.Int32)")]
    pub fn connect_internal_1(_unity_self: &mut PlayableGraph, source: &mut PlayableHandle, source_output_port: i32, destination: &mut PlayableHandle, destination_input_port: i32) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DisconnectInternal_Injected(PlayableGraph&,PlayableHandle&,System.Int32)")]
    pub fn disconnect_internal_1(_unity_self: &mut PlayableGraph, playable: &mut PlayableHandle, input_port: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroyPlayableInternal_Injected(PlayableGraph&,PlayableHandle&)")]
    pub fn destroy_playable_internal_1(_unity_self: &mut PlayableGraph, playable: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableGraph::DestroySubgraphInternal_Injected(PlayableGraph&,PlayableHandle&)")]
    pub fn destroy_subgraph_internal_1(_unity_self: &mut PlayableGraph, playable: &mut PlayableHandle) {}

}
