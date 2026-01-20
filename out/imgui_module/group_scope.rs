#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::gui_content::GUIContent;
use super::gui_style::GUIStyle;
use crate::core_module::{Rect, Texture};
use crate::imgui_module::Scope;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GroupScope", namespace = "UnityEngine", inherit = "Scope")]
pub struct GroupScope(pub *mut c_void);
