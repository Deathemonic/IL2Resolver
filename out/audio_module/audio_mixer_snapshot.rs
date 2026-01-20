#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::audio_mixer::AudioMixer;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioMixerSnapshot", namespace = "UnityEngine.Audio", inherit = "Object")]
pub struct AudioMixerSnapshot(pub *mut c_void);

#[unity_impl]
impl AudioMixerSnapshot {
    #[unity_icall("UnityEngine.Audio.AudioMixerSnapshot::get_audioMixer")]
    pub fn get_audio_mixer(&self) -> Option<AudioMixer> {}

    #[unity_icall("UnityEngine.Audio.AudioMixerSnapshot::get_audioMixer")]
    pub fn transition_to(&self) -> Option<AudioMixer> {}

}
