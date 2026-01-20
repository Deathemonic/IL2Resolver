#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::audio_mixer_group::AudioMixerGroup;
use super::audio_mixer_snapshot::AudioMixerSnapshot;
use super::audio_mixer_update_mode::AudioMixerUpdateMode;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioMixer", namespace = "UnityEngine.Audio", inherit = "Object")]
pub struct AudioMixer(pub *mut c_void);

#[unity_impl]
impl AudioMixer {
    #[unity_icall("UnityEngine.Audio.AudioMixer::get_outputAudioMixerGroup")]
    pub fn get_output_audio_mixer_group(&self) -> Option<AudioMixerGroup> {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::set_outputAudioMixerGroup(AudioMixerGroup)")]
    pub fn set_output_audio_mixer_group(&self, value: Option<AudioMixerGroup>) {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::get_updateMode")]
    pub fn get_update_mode(&self) -> AudioMixerUpdateMode {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::set_updateMode(AudioMixerUpdateMode)")]
    pub fn set_update_mode(&self, value: AudioMixerUpdateMode) {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::FindSnapshot(System.String)")]
    pub fn find_snapshot(&self, name: &str) -> Option<AudioMixerSnapshot> {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::FindMatchingGroups(System.String)")]
    pub fn find_matching_groups(&self, sub_path: &str) -> Array<AudioMixerGroup> {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::TransitionToSnapshotInternal(AudioMixerSnapshot,System.Single)")]
    pub fn transition_to_snapshot_internal(&self, snapshot: Option<AudioMixerSnapshot>, time_to_reach: f32) {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::TransitionToSnapshots(AudioMixerSnapshot[],System.Single[],System.Single)")]
    pub fn transition_to_snapshots(&self, snapshots: Array<AudioMixerSnapshot>, weights: Array<f32>, time_to_reach: f32) {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::SetFloat(System.String,System.Single)")]
    pub fn set_float(&self, name: &str, value: f32) -> bool {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::ClearFloat(System.String)")]
    pub fn clear_float(&self, name: &str) -> bool {}

    #[unity_icall("UnityEngine.Audio.AudioMixer::GetFloat(System.String,System.Single&)")]
    pub fn get_float(&self, name: &str, value: &mut f32) -> bool {}

}
