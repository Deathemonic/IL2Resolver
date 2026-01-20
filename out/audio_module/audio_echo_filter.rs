#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioEchoFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioEchoFilter(pub *mut c_void);

#[unity_impl]
impl AudioEchoFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioEchoFilter::get_delay")]
    pub fn get_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioEchoFilter::set_delay(System.Single)")]
    pub fn set_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioEchoFilter::get_decayRatio")]
    pub fn get_decay_ratio(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioEchoFilter::set_decayRatio(System.Single)")]
    pub fn set_decay_ratio(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioEchoFilter::get_dryMix")]
    pub fn get_dry_mix(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioEchoFilter::set_dryMix(System.Single)")]
    pub fn set_dry_mix(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioEchoFilter::get_wetMix")]
    pub fn get_wet_mix(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioEchoFilter::set_wetMix(System.Single)")]
    pub fn set_wet_mix(&self, value: f32) {}

}
