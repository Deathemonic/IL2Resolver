#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Ping", namespace = "UnityEngine")]
pub struct Ping(pub *mut c_void);

#[unity_impl]
impl Ping {
    #[unity_ctor]
    pub fn new(address: &str) -> Option<Self> {}

    #[unity_method(name = "get_isDone")]
    pub fn get_is_done(&self) -> bool {}

    #[unity_icall("UnityEngine.Ping::get_time")]
    pub fn get_time(&self) -> i32 {}

    #[unity_icall("UnityEngine.Ping::get_ip")]
    pub fn get_ip(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Ping::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.Ping::Internal_Create(System.String)")]
    pub fn internal_create(address: &str) -> isize {}

    #[unity_icall("UnityEngine.Ping::Internal_IsDone")]
    pub fn internal_is_done(&self) -> bool {}

}
