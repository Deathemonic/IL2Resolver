#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::audio_clip_load_type::AudioClipLoadType;
use super::audio_data_load_state::AudioDataLoadState;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioClip", namespace = "UnityEngine", inherit = "Object")]
pub struct AudioClip(pub *mut c_void);

#[unity_impl]
impl AudioClip {
    #[unity_icall("UnityEngine.AudioClip::get_length")]
    pub fn get_length(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioClip::get_samples")]
    pub fn get_samples(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioClip::get_channels")]
    pub fn get_channels(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioClip::get_frequency")]
    pub fn get_frequency(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioClip::get_isReadyToPlay")]
    pub fn get_is_ready_to_play(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::get_loadType")]
    pub fn get_load_type(&self) -> AudioClipLoadType {}

    #[unity_icall("UnityEngine.AudioClip::get_preloadAudioData")]
    pub fn get_preload_audio_data(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::get_ambisonic")]
    pub fn get_ambisonic(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::get_loadInBackground")]
    pub fn get_load_in_background(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::get_loadState")]
    pub fn get_load_state(&self) -> AudioDataLoadState {}

    #[unity_icall("UnityEngine.AudioClip::GetData(AudioClip,System.Single[],System.Int32,System.Int32)")]
    pub fn get_data(clip: Option<AudioClip>, data: &mut Array<f32>, num_samples: i32, samples_offset: i32) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::SetData(AudioClip,System.Single[],System.Int32,System.Int32)")]
    pub fn set_data(clip: Option<AudioClip>, data: Array<f32>, numsamples: i32, samples_offset: i32) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::Construct_Internal")]
    pub fn construct_internal() -> Option<AudioClip> {}

    #[unity_icall("UnityEngine.AudioClip::GetName")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AudioClip::CreateUserSound(System.String,System.Int32,System.Int32,System.Int32,System.Boolean)")]
    pub fn create_user_sound(&self, name: &str, length_samples: i32, channels: i32, frequency: i32, stream: bool) {}

    #[unity_icall("UnityEngine.AudioClip::LoadAudioData")]
    pub fn load_audio_data(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioClip::UnloadAudioData")]
    pub fn unload_audio_data(&self) -> bool {}

}
