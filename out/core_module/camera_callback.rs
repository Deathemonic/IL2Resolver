#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};
use super::camera::Camera;
use crate::mscorlib::{Delegate, MulticastDelegate};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CameraCallback", namespace = "UnityEngine", inherit = "MulticastDelegate,Delegate")]
pub struct CameraCallback(pub *mut c_void);

#[unity_impl]
impl CameraCallback {
    #[unity_ctor]
    pub fn new(object: Option<SystemObject>, method: isize) -> Option<Self> {}

    #[unity_method(name = "Invoke")]
    pub fn invoke(&self, cam: Option<Camera>) {}

    #[unity_method(name = "BeginInvoke")]
    pub fn begin_invoke(&self, cam: Option<Camera>, callback: *mut c_void, object: Option<SystemObject>) -> *mut c_void {}

    #[unity_method(name = "EndInvoke")]
    pub fn end_invoke(&self, result: *mut c_void) {}

}
