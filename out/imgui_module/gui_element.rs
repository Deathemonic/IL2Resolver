#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::core_module::{Camera, Rect};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUIElement", namespace = "UnityEngine")]
pub struct GUIElement(pub *mut c_void);

#[unity_impl]
impl GUIElement {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "HitTest")]
    pub fn hit_test(&self, screen_position: Vector3) -> bool {}

    #[unity_method(name = "HitTest")]
    pub fn hit_test_1(&self, screen_position: Vector3, camera: Option<Camera>) -> bool {}

    #[unity_method(name = "GetScreenRect")]
    pub fn get_screen_rect(&self, camera: Option<Camera>) -> Rect {}

    #[unity_method(name = "GetScreenRect")]
    pub fn get_screen_rect_1(&self) -> Rect {}

}
