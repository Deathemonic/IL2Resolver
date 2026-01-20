#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UIModule", class = "UISystemProfilerApi", namespace = "UnityEngine")]
pub struct UISystemProfilerApi(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SampleType {
    #[default]
    Layout = 0,
    Render = 1,
}

#[unity_impl]
impl UISystemProfilerApi {
    #[unity_icall("UnityEngine.UISystemProfilerApi::BeginSample(UISystemProfilerApi.SampleType)")]
    pub fn begin_sample(type_ref: SampleType) {}

    #[unity_icall("UnityEngine.UISystemProfilerApi::EndSample(UISystemProfilerApi.SampleType)")]
    pub fn end_sample(type_ref: SampleType) {}

    #[unity_icall("UnityEngine.UISystemProfilerApi::AddMarker(System.String,Object)")]
    pub fn add_marker(name: &str, obj: Option<Object>) {}

}
