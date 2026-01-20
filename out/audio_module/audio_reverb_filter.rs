#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::audio_reverb_preset::AudioReverbPreset;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioReverbFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioReverbFilter(pub *mut c_void);

#[unity_impl]
impl AudioReverbFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_reverbPreset")]
    pub fn get_reverb_preset(&self) -> AudioReverbPreset {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_reverbPreset(AudioReverbPreset)")]
    pub fn set_reverb_preset(&self, value: AudioReverbPreset) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_dryLevel")]
    pub fn get_dry_level(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_dryLevel(System.Single)")]
    pub fn set_dry_level(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_room")]
    pub fn get_room(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_room(System.Single)")]
    pub fn set_room(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_roomHF")]
    pub fn get_room_hf(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_roomHF(System.Single)")]
    pub fn set_room_hf(&self, value: f32) {}

    #[unity_method(name = "get_roomRolloffFactor")]
    pub fn get_room_rolloff_factor(&self) -> f32 {}

    #[unity_method(name = "set_roomRolloffFactor")]
    pub fn set_room_rolloff_factor(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_decayTime")]
    pub fn get_decay_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_decayTime(System.Single)")]
    pub fn set_decay_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_decayHFRatio")]
    pub fn get_decay_hf_ratio(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_decayHFRatio(System.Single)")]
    pub fn set_decay_hf_ratio(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_reflectionsLevel")]
    pub fn get_reflections_level(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_reflectionsLevel(System.Single)")]
    pub fn set_reflections_level(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_reflectionsDelay")]
    pub fn get_reflections_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_reflectionsDelay(System.Single)")]
    pub fn set_reflections_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_reverbLevel")]
    pub fn get_reverb_level(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_reverbLevel(System.Single)")]
    pub fn set_reverb_level(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_reverbDelay")]
    pub fn get_reverb_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_reverbDelay(System.Single)")]
    pub fn set_reverb_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_diffusion")]
    pub fn get_diffusion(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_diffusion(System.Single)")]
    pub fn set_diffusion(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_density")]
    pub fn get_density(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_density(System.Single)")]
    pub fn set_density(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_hfReference")]
    pub fn get_hf_reference(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_hfReference(System.Single)")]
    pub fn set_hf_reference(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_roomLF")]
    pub fn get_room_lf(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_roomLF(System.Single)")]
    pub fn set_room_lf(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbFilter::get_lfReference")]
    pub fn get_lf_reference(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbFilter::set_lfReference(System.Single)")]
    pub fn set_lf_reference(&self, value: f32) {}

}
