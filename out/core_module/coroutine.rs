#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::YieldInstruction;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Coroutine", namespace = "UnityEngine", inherit = "YieldInstruction")]
pub struct Coroutine(pub *mut c_void);

#[unity_impl]
impl Coroutine {
    #[unity_icall("UnityEngine.Coroutine::ReleaseCoroutine(System.IntPtr)")]
    pub fn release_coroutine(ptr: isize) {}

}
