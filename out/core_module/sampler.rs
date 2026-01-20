#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{List};
use super::recorder::Recorder;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Sampler", namespace = "UnityEngine.Profiling")]
pub struct Sampler(pub *mut c_void);

#[unity_impl]
impl Sampler {
    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "GetRecorder")]
    pub fn get_recorder(&self) -> Option<Recorder> {}

    #[unity_method(name = "Get", static)]
    pub fn get(name: &str) -> Option<Sampler> {}

    #[unity_method(name = "GetNames", static)]
    pub fn get_names(names: List<SystemString>) -> i32 {}

}
