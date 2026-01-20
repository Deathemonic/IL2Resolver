#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::core_module::Texture;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUIContent", namespace = "UnityEngine")]
pub struct GUIContent(pub *mut c_void);

#[unity_impl]
impl GUIContent {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(text: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(image: Option<Texture>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(src: Option<GUIContent>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(text: &str, image: Option<Texture>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_5(text: &str, tooltip: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_6(image: Option<Texture>, tooltip: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_7(text: &str, image: Option<Texture>, tooltip: &str) -> Option<Self> {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_text")]
    pub fn set_text(&self, value: &str) {}

    #[unity_method(name = "get_image")]
    pub fn get_image(&self) -> Option<Texture> {}

    #[unity_method(name = "set_image")]
    pub fn set_image(&self, value: Option<Texture>) {}

    #[unity_method(name = "get_tooltip")]
    pub fn get_tooltip(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_tooltip")]
    pub fn set_tooltip(&self, value: &str) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
