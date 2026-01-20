#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::audio_reverb_preset::AudioReverbPreset;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioReverbZone", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioReverbZone(pub *mut c_void);

#[unity_impl]
impl AudioReverbZone {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_minDistance")]
    pub fn get_min_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_minDistance(System.Single)")]
    pub fn set_min_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_maxDistance")]
    pub fn get_max_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_maxDistance(System.Single)")]
    pub fn set_max_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_reverbPreset")]
    pub fn get_reverb_preset(&self) -> AudioReverbPreset {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_reverbPreset(AudioReverbPreset)")]
    pub fn set_reverb_preset(&self, value: AudioReverbPreset) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_room")]
    pub fn get_room(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_room(System.Int32)")]
    pub fn set_room(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_roomHF")]
    pub fn get_room_hf(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_roomHF(System.Int32)")]
    pub fn set_room_hf(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_roomLF")]
    pub fn get_room_lf(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_roomLF(System.Int32)")]
    pub fn set_room_lf(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_decayTime")]
    pub fn get_decay_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_decayTime(System.Single)")]
    pub fn set_decay_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_decayHFRatio")]
    pub fn get_decay_hf_ratio(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_decayHFRatio(System.Single)")]
    pub fn set_decay_hf_ratio(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_reflections")]
    pub fn get_reflections(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_reflections(System.Int32)")]
    pub fn set_reflections(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_reflectionsDelay")]
    pub fn get_reflections_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_reflectionsDelay(System.Single)")]
    pub fn set_reflections_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_reverb")]
    pub fn get_reverb(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_reverb(System.Int32)")]
    pub fn set_reverb(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_reverbDelay")]
    pub fn get_reverb_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_reverbDelay(System.Single)")]
    pub fn set_reverb_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_HFReference")]
    pub fn get_hf_reference(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_HFReference(System.Single)")]
    pub fn set_hf_reference(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_LFReference")]
    pub fn get_lf_reference(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_LFReference(System.Single)")]
    pub fn set_lf_reference(&self, value: f32) {}

    #[unity_method(name = "get_roomRolloffFactor")]
    pub fn get_room_rolloff_factor(&self) -> f32 {}

    #[unity_method(name = "set_roomRolloffFactor")]
    pub fn set_room_rolloff_factor(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_diffusion")]
    pub fn get_diffusion(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_diffusion(System.Single)")]
    pub fn set_diffusion(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioReverbZone::get_density")]
    pub fn get_density(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioReverbZone::set_density(System.Single)")]
    pub fn set_density(&self, value: f32) {}

}
