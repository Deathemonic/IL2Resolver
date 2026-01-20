#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ReadOnly", namespace = "Unity.Collections", value_type)]
pub struct ReadOnly {
    pub m_buffer: *mut (),
    pub m_length: i32,
}

#[unity_impl]
impl ReadOnly {
    #[unity_method(name = "get_Length")]
    pub fn get_length(&self) -> i32 {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> *mut c_void {}

    #[unity_method(name = "get_IsCreated")]
    pub fn get_is_created(&self) -> bool {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to(&self, array: Array<*mut c_void>) {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to_1(&self, array: *mut c_void) {}

    #[unity_method(name = "ToArray")]
    pub fn to_array(&self) -> Array<*mut c_void> {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> *mut c_void {}

}
