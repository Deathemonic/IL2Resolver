#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::gui_content::GUIContent;
use super::gui_layout_option::GUILayoutOption;
use super::gui_style::GUIStyle;
use crate::core_module::Rect;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUILayoutUtility", namespace = "UnityEngine")]
pub struct GUILayoutUtility(pub *mut c_void);

#[unity_impl]
impl GUILayoutUtility {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group(group_name: &str) {}

    #[unity_method(name = "EndGroup", static)]
    pub fn end_group(group_name: &str) {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect(content: Option<GUIContent>, style: Option<GUIStyle>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_1(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_2(width: f32, height: f32) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_3(width: f32, height: f32, style: Option<GUIStyle>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_4(width: f32, height: f32, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_5(width: f32, height: f32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_6(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_7(min_width: f32, max_width: f32, min_height: f32, max_height: f32, style: Option<GUIStyle>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_8(min_width: f32, max_width: f32, min_height: f32, max_height: f32, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetRect", static)]
    pub fn get_rect_9(min_width: f32, max_width: f32, min_height: f32, max_height: f32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetLastRect", static)]
    pub fn get_last_rect() -> Rect {}

    #[unity_method(name = "GetAspectRect", static)]
    pub fn get_aspect_rect(aspect: f32) -> Rect {}

    #[unity_method(name = "GetAspectRect", static)]
    pub fn get_aspect_rect_1(aspect: f32, style: Option<GUIStyle>) -> Rect {}

    #[unity_method(name = "GetAspectRect", static)]
    pub fn get_aspect_rect_2(aspect: f32, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "GetAspectRect", static)]
    pub fn get_aspect_rect_3(aspect: f32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_icall("UnityEngine.GUILayoutUtility::Internal_GetWindowRect_Injected(System.Int32,Rect&)")]
    pub fn internal_get_window_rect(window_id: i32, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.GUILayoutUtility::Internal_MoveWindow_Injected(System.Int32,Rect&)")]
    pub fn internal_move_window(window_id: i32, r: &mut Rect) {}

    #[unity_icall("UnityEngine.GUILayoutUtility::GetWindowsBounds_Injected(Rect&)")]
    pub fn get_windows_bounds(ret: &mut Rect) {}

}
