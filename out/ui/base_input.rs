#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use crate::input_legacy_module::{IMECompositionMode, Touch};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "BaseInput", namespace = "UnityEngine.EventSystems", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct BaseInput(pub *mut c_void);

#[unity_impl]
impl BaseInput {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_compositionString")]
    pub fn get_composition_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_imeCompositionMode")]
    pub fn get_ime_composition_mode(&self) -> IMECompositionMode {}

    #[unity_method(name = "set_imeCompositionMode")]
    pub fn set_ime_composition_mode(&self, value: IMECompositionMode) {}

    #[unity_method(name = "get_compositionCursorPos")]
    pub fn get_composition_cursor_pos(&self) -> Vector2 {}

    #[unity_method(name = "set_compositionCursorPos")]
    pub fn set_composition_cursor_pos(&self, value: Vector2) {}

    #[unity_method(name = "get_mousePresent")]
    pub fn get_mouse_present(&self) -> bool {}

    #[unity_method(name = "get_mousePosition")]
    pub fn get_mouse_position(&self) -> Vector2 {}

    #[unity_method(name = "get_mouseScrollDelta")]
    pub fn get_mouse_scroll_delta(&self) -> Vector2 {}

    #[unity_method(name = "get_touchSupported")]
    pub fn get_touch_supported(&self) -> bool {}

    #[unity_method(name = "get_touchCount")]
    pub fn get_touch_count(&self) -> i32 {}

    #[unity_method(name = "GetMouseButtonDown")]
    pub fn get_mouse_button_down(&self, button: i32) -> bool {}

    #[unity_method(name = "GetMouseButtonUp")]
    pub fn get_mouse_button_up(&self, button: i32) -> bool {}

    #[unity_method(name = "GetMouseButton")]
    pub fn get_mouse_button(&self, button: i32) -> bool {}

    #[unity_method(name = "GetTouch")]
    pub fn get_touch(&self, index: i32) -> Touch {}

    #[unity_method(name = "GetAxisRaw")]
    pub fn get_axis_raw(&self, axis_name: &str) -> f32 {}

    #[unity_method(name = "GetButtonDown")]
    pub fn get_button_down(&self, button_name: &str) -> bool {}

}
