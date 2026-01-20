#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::object::Object;
use crate::core_module::Sampler;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CustomSampler", namespace = "UnityEngine.Profiling", inherit = "Sampler")]
pub struct CustomSampler(pub *mut c_void);

#[unity_impl]
impl CustomSampler {
    #[unity_method(name = "Create", static)]
    pub fn create(name: &str, collect_gpu_data: bool) -> Option<CustomSampler> {}

    #[unity_method(name = "Begin")]
    pub fn begin(&self) {}

    #[unity_method(name = "Begin")]
    pub fn begin_1(&self, target_object: Option<Object>) {}

    #[unity_method(name = "End")]
    pub fn end(&self) {}

}
