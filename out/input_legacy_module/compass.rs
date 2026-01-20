#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "Compass", namespace = "UnityEngine")]
pub struct Compass(pub *mut c_void);

#[unity_impl]
impl Compass {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_magneticHeading")]
    pub fn get_magnetic_heading(&self) -> f32 {}

    #[unity_method(name = "get_trueHeading")]
    pub fn get_true_heading(&self) -> f32 {}

    #[unity_method(name = "get_headingAccuracy")]
    pub fn get_heading_accuracy(&self) -> f32 {}

    #[unity_method(name = "get_rawVector")]
    pub fn get_raw_vector(&self) -> Vector3 {}

    #[unity_method(name = "get_timestamp")]
    pub fn get_timestamp(&self) -> f64 {}

    #[unity_method(name = "get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_method(name = "set_enabled")]
    pub fn set_enabled(&self, value: bool) {}

}
