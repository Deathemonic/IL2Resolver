#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "YieldInstruction", namespace = "UnityEngine")]
pub struct YieldInstruction(pub *mut c_void);
