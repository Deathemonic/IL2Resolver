#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "AbstractEventData", namespace = "UnityEngine.EventSystems")]
pub struct AbstractEventData(pub *mut c_void);

#[unity_impl]
impl AbstractEventData {
    #[unity_method(name = "get_used")]
    pub fn get_used(&self) -> bool {}

    #[unity_method(name = "Reset")]
    pub fn reset(&self) {}

    #[unity_method(name = "Use")]
    pub fn use_ref(&self) {}

}
