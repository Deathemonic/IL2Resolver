#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};
use crate::mscorlib::collections::{Array};
use super::allocator::Allocator;
use super::job_handle::JobHandle;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "NativeArray", namespace = "Unity.Collections", value_type)]
pub struct NativeArray {
    pub m_buffer: *mut (),
    pub m_length: i32,
    pub m_allocator_label: Allocator,
}

#[unity_impl]
impl NativeArray {
    #[unity_method(name = "get_Length")]
    pub fn get_length(&self) -> i32 {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> *mut c_void {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: *mut c_void) {}

    #[unity_method(name = "get_IsCreated")]
    pub fn get_is_created(&self) -> bool {}

    #[unity_method(name = "Dispose")]
    pub fn dispose(&self) {}

    #[unity_method(name = "Dispose")]
    pub fn dispose_1(&self, input_deps: JobHandle) -> JobHandle {}

    #[unity_method(name = "CopyFrom")]
    pub fn copy_from(&self, array: Array<*mut c_void>) {}

    #[unity_method(name = "CopyFrom")]
    pub fn copy_from_1(&self, array: *mut c_void) {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to(&self, array: Array<*mut c_void>) {}

    #[unity_method(name = "CopyTo")]
    pub fn copy_to_1(&self, array: *mut c_void) {}

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

    #[unity_method(name = "Copy", static)]
    pub fn copy(src: *mut c_void, dst: *mut c_void) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_1(src: *mut c_void, dst: *mut c_void) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_2(src: Array<*mut c_void>, dst: *mut c_void) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_3(src: *mut c_void, dst: Array<*mut c_void>) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_4(src: *mut c_void, dst: Array<*mut c_void>) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_5(src: *mut c_void, dst: *mut c_void, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_6(src: *mut c_void, dst: *mut c_void, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_7(src: Array<*mut c_void>, dst: *mut c_void, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_8(src: *mut c_void, dst: Array<*mut c_void>, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_9(src: *mut c_void, dst: Array<*mut c_void>, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_10(src: *mut c_void, src_index: i32, dst: *mut c_void, dst_index: i32, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_11(src: *mut c_void, src_index: i32, dst: *mut c_void, dst_index: i32, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_12(src: Array<*mut c_void>, src_index: i32, dst: *mut c_void, dst_index: i32, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_13(src: *mut c_void, src_index: i32, dst: Array<*mut c_void>, dst_index: i32, length: i32) {}

    #[unity_method(name = "Copy", static)]
    pub fn copy_14(src: *mut c_void, src_index: i32, dst: Array<*mut c_void>, dst_index: i32, length: i32) {}

    #[unity_method(name = "AsReadOnly")]
    pub fn as_read_only(&self) -> *mut c_void {}

}
