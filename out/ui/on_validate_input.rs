#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};
use crate::mscorlib::{Delegate, MulticastDelegate};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "OnValidateInput", namespace = "UnityEngine.UI", inherit = "MulticastDelegate,Delegate")]
pub struct OnValidateInput(pub *mut c_void);

#[unity_impl]
impl OnValidateInput {
    #[unity_ctor]
    pub fn new(object: Option<SystemObject>, method: isize) -> Option<Self> {}

    #[unity_method(name = "Invoke")]
    pub fn invoke(&self, text: &str, char_index: i32, added_char: u16) -> u16 {}

    #[unity_method(name = "BeginInvoke")]
    pub fn begin_invoke(&self, text: &str, char_index: i32, added_char: u16, callback: *mut c_void, object: Option<SystemObject>) -> *mut c_void {}

    #[unity_method(name = "EndInvoke")]
    pub fn end_invoke(&self, result: *mut c_void) -> u16 {}

}
