#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::audio_mixer::AudioMixer;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioMixerGroup", namespace = "UnityEngine.Audio", inherit = "Object")]
pub struct AudioMixerGroup(pub *mut c_void);

#[unity_impl]
impl AudioMixerGroup {
    #[unity_icall("UnityEngine.Audio.AudioMixerGroup::get_audioMixer")]
    pub fn get_audio_mixer(&self) -> Option<AudioMixer> {}

}
