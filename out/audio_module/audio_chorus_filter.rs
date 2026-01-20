#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioChorusFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioChorusFilter(pub *mut c_void);

#[unity_impl]
impl AudioChorusFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_dryMix")]
    pub fn get_dry_mix(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_dryMix(System.Single)")]
    pub fn set_dry_mix(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_wetMix1")]
    pub fn get_wet_mix1(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_wetMix1(System.Single)")]
    pub fn set_wet_mix1(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_wetMix2")]
    pub fn get_wet_mix2(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_wetMix2(System.Single)")]
    pub fn set_wet_mix2(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_wetMix3")]
    pub fn get_wet_mix3(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_wetMix3(System.Single)")]
    pub fn set_wet_mix3(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_delay")]
    pub fn get_delay(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_delay(System.Single)")]
    pub fn set_delay(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_rate")]
    pub fn get_rate(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_rate(System.Single)")]
    pub fn set_rate(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioChorusFilter::get_depth")]
    pub fn get_depth(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioChorusFilter::set_depth(System.Single)")]
    pub fn set_depth(&self, value: f32) {}

    #[unity_method(name = "get_feedback")]
    pub fn get_feedback(&self) -> f32 {}

    #[unity_method(name = "set_feedback")]
    pub fn set_feedback(&self, value: f32) {}

}
