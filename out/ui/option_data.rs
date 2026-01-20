#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::core_module::Sprite;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "OptionData", namespace = "UnityEngine.UI")]
pub struct OptionData(pub *mut c_void);

#[unity_impl]
impl OptionData {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(text: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(image: Option<Sprite>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(text: &str, image: Option<Sprite>) -> Option<Self> {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_text")]
    pub fn set_text(&self, value: &str) {}

    #[unity_method(name = "get_image")]
    pub fn get_image(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_image")]
    pub fn set_image(&self, value: Option<Sprite>) {}

}
