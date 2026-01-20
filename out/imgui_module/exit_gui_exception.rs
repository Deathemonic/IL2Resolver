#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::Exception;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "ExitGUIException", namespace = "UnityEngine", inherit = "Exception")]
pub struct ExitGUIException(pub *mut c_void);
