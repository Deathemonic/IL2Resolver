#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Color, Texture2D};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUIStyleState", namespace = "UnityEngine")]
pub struct GUIStyleState(pub *mut c_void);

#[unity_impl]
impl GUIStyleState {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.GUIStyleState::get_background")]
    pub fn get_background(&self) -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.GUIStyleState::set_background(Texture2D)")]
    pub fn set_background(&self, value: Option<Texture2D>) {}

    #[unity_icall("UnityEngine.GUIStyleState::get_textColor_Injected(Color&)")]
    pub fn get_text_color(&self, ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUIStyleState::set_textColor_Injected(Color&)")]
    pub fn set_text_color(&self, value: &mut Color) {}

    #[unity_icall("UnityEngine.GUIStyleState::Init")]
    pub fn init() -> isize {}

    #[unity_icall("UnityEngine.GUIStyleState::Cleanup")]
    pub fn cleanup(&self) {}

}
