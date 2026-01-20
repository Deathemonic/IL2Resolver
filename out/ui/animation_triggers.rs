#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "AnimationTriggers", namespace = "UnityEngine.UI")]
pub struct AnimationTriggers(pub *mut c_void);

#[unity_impl]
impl AnimationTriggers {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_normalTrigger")]
    pub fn get_normal_trigger(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_normalTrigger")]
    pub fn set_normal_trigger(&self, value: &str) {}

    #[unity_method(name = "get_highlightedTrigger")]
    pub fn get_highlighted_trigger(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_highlightedTrigger")]
    pub fn set_highlighted_trigger(&self, value: &str) {}

    #[unity_method(name = "get_pressedTrigger")]
    pub fn get_pressed_trigger(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_pressedTrigger")]
    pub fn set_pressed_trigger(&self, value: &str) {}

    #[unity_method(name = "get_selectedTrigger")]
    pub fn get_selected_trigger(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_selectedTrigger")]
    pub fn set_selected_trigger(&self, value: &str) {}

    #[unity_method(name = "get_disabledTrigger")]
    pub fn get_disabled_trigger(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_disabledTrigger")]
    pub fn set_disabled_trigger(&self, value: &str) {}

}
