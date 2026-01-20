#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::gui_content::GUIContent;
use super::gui_layout_option::GUILayoutOption;
use super::gui_style::GUIStyle;
use crate::core_module::Texture;
use crate::imgui_module::Scope;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "HorizontalScope", namespace = "UnityEngine", inherit = "Scope")]
pub struct HorizontalScope(pub *mut c_void);
