#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUILayoutOption", namespace = "UnityEngine")]
pub struct GUILayoutOption(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Type {
    #[default]
    fixedWidth = 0,
    fixedHeight = 1,
    minWidth = 2,
    maxWidth = 3,
    minHeight = 4,
    maxHeight = 5,
    stretchWidth = 6,
    stretchHeight = 7,
    alignStart = 8,
    alignMiddle = 9,
    alignEnd = 10,
    alignJustify = 11,
    equalSize = 12,
    spacing = 13,
}
