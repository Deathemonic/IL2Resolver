#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::Color;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUISettings", namespace = "UnityEngine")]
pub struct GUISettings(pub *mut c_void);

#[unity_impl]
impl GUISettings {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_doubleClickSelectsWord")]
    pub fn get_double_click_selects_word(&self) -> bool {}

    #[unity_method(name = "set_doubleClickSelectsWord")]
    pub fn set_double_click_selects_word(&self, value: bool) {}

    #[unity_method(name = "get_tripleClickSelectsLine")]
    pub fn get_triple_click_selects_line(&self) -> bool {}

    #[unity_method(name = "set_tripleClickSelectsLine")]
    pub fn set_triple_click_selects_line(&self, value: bool) {}

    #[unity_method(name = "get_cursorColor")]
    pub fn get_cursor_color(&self) -> Color {}

    #[unity_method(name = "set_cursorColor")]
    pub fn set_cursor_color(&self, value: Color) {}

    #[unity_method(name = "get_cursorFlashSpeed")]
    pub fn get_cursor_flash_speed(&self) -> f32 {}

    #[unity_method(name = "set_cursorFlashSpeed")]
    pub fn set_cursor_flash_speed(&self, value: f32) {}

    #[unity_method(name = "get_selectionColor")]
    pub fn get_selection_color(&self) -> Color {}

    #[unity_method(name = "set_selectionColor")]
    pub fn set_selection_color(&self, value: Color) {}

    #[unity_icall("UnityEngine.GUISettings::Internal_GetCursorFlashSpeed")]
    pub fn internal_get_cursor_flash_speed() -> f32 {}

}
