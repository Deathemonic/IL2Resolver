#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::YieldInstruction;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "AsyncOperation", namespace = "UnityEngine", inherit = "YieldInstruction")]
pub struct AsyncOperation(pub *mut c_void);

#[unity_impl]
impl AsyncOperation {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AsyncOperation::get_isDone")]
    pub fn get_is_done(&self) -> bool {}

    #[unity_icall("UnityEngine.AsyncOperation::get_progress")]
    pub fn get_progress(&self) -> f32 {}

    #[unity_icall("UnityEngine.AsyncOperation::get_priority")]
    pub fn get_priority(&self) -> i32 {}

    #[unity_icall("UnityEngine.AsyncOperation::set_priority(System.Int32)")]
    pub fn set_priority(&self, value: i32) {}

    #[unity_icall("UnityEngine.AsyncOperation::get_allowSceneActivation")]
    pub fn get_allow_scene_activation(&self) -> bool {}

    #[unity_icall("UnityEngine.AsyncOperation::set_allowSceneActivation(System.Boolean)")]
    pub fn set_allow_scene_activation(&self, value: bool) {}

    #[unity_method(name = "remove_completed")]
    pub fn remove_completed(&self, value: *mut c_void) {}

    #[unity_icall("UnityEngine.AsyncOperation::InternalDestroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

}
