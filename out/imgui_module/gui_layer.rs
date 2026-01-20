#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use super::gui_element::GUIElement;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUILayer", namespace = "UnityEngine")]
pub struct GUILayer(pub *mut c_void);

#[unity_impl]
impl GUILayer {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "HitTest")]
    pub fn hit_test(&self, screen_position: Vector3) -> Option<GUIElement> {}

}
