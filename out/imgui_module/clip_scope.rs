#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::Rect;
use crate::imgui_module::Scope;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "ClipScope", namespace = "UnityEngine", inherit = "Scope")]
pub struct ClipScope(pub *mut c_void);
