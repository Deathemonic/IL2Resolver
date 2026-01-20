#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "TrackedReference", namespace = "UnityEngine")]
pub struct TrackedReference(pub *mut c_void);

#[unity_impl]
impl TrackedReference {
    #[unity_method(name = "Equals")]
    pub fn equals(&self, o: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
