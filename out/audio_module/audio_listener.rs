#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::audio_velocity_update_mode::AudioVelocityUpdateMode;
use super::fft_window::FFTWindow;
use crate::audio_module::AudioBehaviour;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioListener", namespace = "UnityEngine", inherit = "AudioBehaviour,Behaviour,Component,Object")]
pub struct AudioListener(pub *mut c_void);

#[unity_impl]
impl AudioListener {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioListener::get_volume")]
    pub fn get_volume() -> f32 {}

    #[unity_icall("UnityEngine.AudioListener::set_volume(System.Single)")]
    pub fn set_volume(value: f32) {}

    #[unity_icall("UnityEngine.AudioListener::get_pause")]
    pub fn get_pause() -> bool {}

    #[unity_icall("UnityEngine.AudioListener::set_pause(System.Boolean)")]
    pub fn set_pause(value: bool) {}

    #[unity_icall("UnityEngine.AudioListener::get_velocityUpdateMode")]
    pub fn get_velocity_update_mode(&self) -> AudioVelocityUpdateMode {}

    #[unity_icall("UnityEngine.AudioListener::set_velocityUpdateMode(AudioVelocityUpdateMode)")]
    pub fn set_velocity_update_mode(&self, value: AudioVelocityUpdateMode) {}

    #[unity_icall("UnityEngine.AudioListener::GetOutputDataHelper(System.Single[],System.Int32)")]
    pub fn get_output_data(samples: &mut Array<f32>, channel: i32) {}

    #[unity_icall("UnityEngine.AudioListener::GetSpectrumDataHelper(System.Single[],System.Int32,FFTWindow)")]
    pub fn get_spectrum_data(samples: &mut Array<f32>, channel: i32, window: FFTWindow) {}

}
