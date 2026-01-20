#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioDistortionFilter", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct AudioDistortionFilter(pub *mut c_void);

#[unity_impl]
impl AudioDistortionFilter {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioDistortionFilter::get_distortionLevel")]
    pub fn get_distortion_level(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioDistortionFilter::set_distortionLevel(System.Single)")]
    pub fn set_distortion_level(&self, value: f32) {}

}
