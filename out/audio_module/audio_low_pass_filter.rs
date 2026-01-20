#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::AnimationCurve;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioLowPassFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioLowPassFilter(pub *mut c_void);

#[unity_impl]
impl AudioLowPassFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_customCutoffCurve")]
    pub fn get_custom_cutoff_curve(&self) -> Option<AnimationCurve> {}

    #[unity_method(name = "set_customCutoffCurve")]
    pub fn set_custom_cutoff_curve(&self, value: Option<AnimationCurve>) {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::get_cutoffFrequency")]
    pub fn get_cutoff_frequency(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::set_cutoffFrequency(System.Single)")]
    pub fn set_cutoff_frequency(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::get_lowpassResonanceQ")]
    pub fn get_lowpass_resonance_q(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::set_lowpassResonanceQ(System.Single)")]
    pub fn set_lowpass_resonance_q(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::GetCustomLowpassLevelCurveCopy")]
    pub fn get_custom_lowpass_level_curve_copy(&self) -> Option<AnimationCurve> {}

    #[unity_icall("UnityEngine.AudioLowPassFilter::SetCustomLowpassLevelCurveHelper(AudioLowPassFilter,AnimationCurve)")]
    pub fn set_custom_lowpass_level_curve_helper(source: Option<AudioLowPassFilter>, curve: Option<AnimationCurve>) {}

}
