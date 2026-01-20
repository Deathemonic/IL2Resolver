#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::UnityEventBase;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "UnityEvent", namespace = "UnityEngine.Events", inherit = "UnityEventBase")]
pub struct UnityEvent(pub *mut c_void);

#[unity_impl]
impl UnityEvent {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "AddListener")]
    pub fn add_listener(&self, call: *mut c_void) {}

    #[unity_method(name = "RemoveListener")]
    pub fn remove_listener(&self, call: *mut c_void) {}

    #[unity_method(name = "Invoke")]
    pub fn invoke(&self, arg0: *mut c_void) {}

}
