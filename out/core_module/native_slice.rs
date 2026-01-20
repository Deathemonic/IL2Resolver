#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};
use crate::mscorlib::collections::{Array};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "NativeSlice", namespace = "Unity.Collections", value_type)]
pub struct NativeSlice {
    pub m_buffer: *mut u8,
    pub m_stride: i32,
    pub m_length: i32,
}

#[unity_impl]
impl NativeSlice {
    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> *mut c_void {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: *mut c_void) {}

    #[unity_method(name = "get_Stride")]
    pub fn get_stride(&self) -> i32 {}

    #[unity_method(name = "get_Length")]
    pub fn get_length(&self) -> i32 {}

    #[unity_method(name = "CopyFrom")]
    pub fn copy_from(&self, slice: *mut c_void) {}

    #[unity_method(name = "CopyFrom")]
    pub fn copy_from_1(&self, array: Array<*mut c_void>) {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to(&self, array: *mut c_void) {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to_1(&self, array: Array<*mut c_void>) {}

    #[unity_method(name = "ToArray")]
    pub fn to_array(&self) -> Array<*mut c_void> {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> *mut c_void {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: *mut c_void) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
