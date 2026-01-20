#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "Scope", namespace = "UnityEngine")]
pub struct Scope(pub *mut c_void);

#[unity_impl]
impl Scope {
    #[unity_method(name = "Dispose")]
    pub fn dispose(&self) {}

}
