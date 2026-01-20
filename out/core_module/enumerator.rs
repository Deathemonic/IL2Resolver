#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Enumerator", namespace = "Unity.Collections", value_type)]
pub struct Enumerator {
    pub m_array: *mut c_void,
    pub m_index: i32,
}

#[unity_impl]
impl Enumerator {
    #[unity_method(name = "get_Current")]
    pub fn get_current(&self) -> *mut c_void {}

    #[unity_method(name = "Dispose")]
    pub fn dispose(&self) {}

    #[unity_method(name = "MoveNext")]
    pub fn move_next(&self) -> bool {}

    #[unity_method(name = "Reset")]
    pub fn reset(&self) {}

}
