#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::collections::{Array};
use super::gui_layout_option::GUILayoutOption;
use super::gui_style::GUIStyle;
use crate::imgui_module::Scope;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "ScrollViewScope", namespace = "UnityEngine", inherit = "Scope")]
pub struct ScrollViewScope(pub *mut c_void);

#[unity_impl]
impl ScrollViewScope {
    #[unity_ctor]
    pub fn new(scroll_position: Vector2, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(scroll_position: Vector2, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(scroll_position: Vector2, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_5(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, background: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<Self> {}

    #[unity_method(name = "get_scrollPosition")]
    pub fn get_scroll_position(&self) -> Vector2 {}

    #[unity_method(name = "set_scrollPosition")]
    pub fn set_scroll_position(&self, value: Vector2) {}

    #[unity_method(name = "get_handleScrollWheel")]
    pub fn get_handle_scroll_wheel(&self) -> bool {}

    #[unity_method(name = "set_handleScrollWheel")]
    pub fn set_handle_scroll_wheel(&self, value: bool) {}

}
