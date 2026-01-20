#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioHighPassFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioHighPassFilter(pub *mut c_void);

#[unity_impl]
impl AudioHighPassFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioHighPassFilter::get_cutoffFrequency")]
    pub fn get_cutoff_frequency(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioHighPassFilter::set_cutoffFrequency(System.Single)")]
    pub fn set_cutoff_frequency(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioHighPassFilter::get_highpassResonanceQ")]
    pub fn get_highpass_resonance_q(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioHighPassFilter::set_highpassResonanceQ(System.Single)")]
    pub fn set_highpass_resonance_q(&self, value: f32) {}

}
