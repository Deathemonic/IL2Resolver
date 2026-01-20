#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2};
use crate::mscorlib::{SystemObject, SystemString};
use super::gui_content::GUIContent;
use super::gui_skin::GUISkin;
use super::gui_style::GUIStyle;
use super::window_function::WindowFunction;
use crate::core_module::{Color, Material, Rect, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUI", namespace = "UnityEngine")]
pub struct GUI(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarButtonSize {
    #[default]
    Fixed = 0,
    FitToContents = 1,
}

#[unity_impl]
impl GUI {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_skin", static)]
    pub fn get_skin() -> Option<GUISkin> {}

    #[unity_method(name = "set_skin", static)]
    pub fn set_skin(value: Option<GUISkin>) {}

    #[unity_method(name = "get_matrix", static)]
    pub fn get_matrix() -> Matrix4x4 {}

    #[unity_method(name = "set_matrix", static)]
    pub fn set_matrix(value: Matrix4x4) {}

    #[unity_method(name = "get_tooltip", static)]
    pub fn get_tooltip() -> Option<SystemString> {}

    #[unity_method(name = "set_tooltip", static)]
    pub fn set_tooltip(value: &str) {}

    #[unity_icall("UnityEngine.GUI::get_color_Injected(Color&)")]
    pub fn get_color(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::set_color_Injected(Color&)")]
    pub fn set_color(value: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_backgroundColor_Injected(Color&)")]
    pub fn get_background_color(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::set_backgroundColor_Injected(Color&)")]
    pub fn set_background_color(value: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_contentColor_Injected(Color&)")]
    pub fn get_content_color(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::set_contentColor_Injected(Color&)")]
    pub fn set_content_color(value: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_changed")]
    pub fn get_changed() -> bool {}

    #[unity_icall("UnityEngine.GUI::set_changed(System.Boolean)")]
    pub fn set_changed(value: bool) {}

    #[unity_icall("UnityEngine.GUI::get_enabled")]
    pub fn get_enabled() -> bool {}

    #[unity_icall("UnityEngine.GUI::set_enabled(System.Boolean)")]
    pub fn set_enabled(value: bool) {}

    #[unity_icall("UnityEngine.GUI::get_depth")]
    pub fn get_depth() -> i32 {}

    #[unity_icall("UnityEngine.GUI::set_depth(System.Int32)")]
    pub fn set_depth(value: i32) {}

    #[unity_method(name = "Label", static)]
    pub fn label(position: Rect, text: &str) {}

    #[unity_method(name = "Label", static)]
    pub fn label_1(position: Rect, image: Option<Texture>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_2(position: Rect, content: Option<GUIContent>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_3(position: Rect, text: &str, style: Option<GUIStyle>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_4(position: Rect, image: Option<Texture>, style: Option<GUIStyle>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_5(position: Rect, content: Option<GUIContent>, style: Option<GUIStyle>) {}

    #[unity_icall("UnityEngine.GUI::get_color_Injected(Color&)")]
    pub fn draw_texture(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_color_Injected(Color&)")]
    pub fn draw_texture_1(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_color_Injected(Color&)")]
    pub fn draw_texture_2(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_color_Injected(Color&)")]
    pub fn draw_texture_3(ret: &mut Color) {}

    #[unity_icall("UnityEngine.GUI::get_roundedRectWithColorPerBorderMaterial")]
    pub fn draw_texture_4() -> Option<Material> {}

    #[unity_icall("UnityEngine.GUI::get_blitMaterial")]
    pub fn draw_texture_with_tex_coords() -> Option<Material> {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box(position: Rect, text: &str) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_1(position: Rect, image: Option<Texture>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_2(position: Rect, content: Option<GUIContent>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_3(position: Rect, text: &str, style: Option<GUIStyle>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_4(position: Rect, image: Option<Texture>, style: Option<GUIStyle>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_5(position: Rect, content: Option<GUIContent>, style: Option<GUIStyle>) {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button(position: Rect, text: &str) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_1(position: Rect, image: Option<Texture>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_2(position: Rect, content: Option<GUIContent>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_3(position: Rect, text: &str, style: Option<GUIStyle>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_4(position: Rect, image: Option<Texture>, style: Option<GUIStyle>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_5(position: Rect, content: Option<GUIContent>, style: Option<GUIStyle>) -> bool {}

    #[unity_method(name = "HorizontalSlider", static)]
    pub fn horizontal_slider(position: Rect, value: f32, left_value: f32, right_value: f32) -> f32 {}

    #[unity_method(name = "HorizontalSlider", static)]
    pub fn horizontal_slider_1(position: Rect, value: f32, left_value: f32, right_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>) -> f32 {}

    #[unity_method(name = "HorizontalSlider", static)]
    pub fn horizontal_slider_2(position: Rect, value: f32, left_value: f32, right_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>, thumb_extent: Option<GUIStyle>) -> f32 {}

    #[unity_method(name = "VerticalSlider", static)]
    pub fn vertical_slider(position: Rect, value: f32, top_value: f32, bottom_value: f32) -> f32 {}

    #[unity_method(name = "VerticalSlider", static)]
    pub fn vertical_slider_1(position: Rect, value: f32, top_value: f32, bottom_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>) -> f32 {}

    #[unity_method(name = "VerticalSlider", static)]
    pub fn vertical_slider_2(position: Rect, value: f32, top_value: f32, bottom_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>, thumb_extent: Option<GUIStyle>) -> f32 {}

    #[unity_method(name = "Slider", static)]
    pub fn slider(position: Rect, value: f32, size: f32, start: f32, end: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>, horiz: bool, id: i32, thumb_extent: Option<GUIStyle>) -> f32 {}

    #[unity_method(name = "BeginClip", static)]
    pub fn begin_clip(position: Rect, scroll_offset: Vector2, render_offset: Vector2, reset_offset: bool) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group(position: Rect) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_1(position: Rect, text: &str) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_2(position: Rect, image: Option<Texture>) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_3(position: Rect, content: Option<GUIContent>) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_4(position: Rect, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_5(position: Rect, text: &str, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_6(position: Rect, image: Option<Texture>, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginGroup", static)]
    pub fn begin_group_7(position: Rect, content: Option<GUIContent>, style: Option<GUIStyle>) {}

    #[unity_method(name = "EndGroup", static)]
    pub fn end_group() {}

    #[unity_method(name = "BeginClip", static)]
    pub fn begin_clip_1(position: Rect) {}

    #[unity_method(name = "EndClip", static)]
    pub fn end_clip() {}

    #[unity_icall("UnityEngine.GUI::InternalRepaintEditorWindow")]
    pub fn end_scroll_view() {}

    #[unity_method(name = "ScrollTo", static)]
    pub fn scroll_to(position: Rect) {}

    #[unity_method(name = "ScrollTowards", static)]
    pub fn scroll_towards(position: Rect, max_delta: f32) -> bool {}

    #[unity_icall("UnityEngine.GUI::GrabMouseControl(System.Int32)")]
    pub fn grab_mouse_control(id: i32) {}

    #[unity_icall("UnityEngine.GUI::HasMouseControl(System.Int32)")]
    pub fn has_mouse_control(id: i32) -> bool {}

    #[unity_icall("UnityEngine.GUI::ReleaseMouseControl")]
    pub fn release_mouse_control() {}

    #[unity_icall("UnityEngine.GUI::SetNextControlName(System.String)")]
    pub fn set_next_control_name(name: &str) {}

    #[unity_icall("UnityEngine.GUI::GetNameOfFocusedControl")]
    pub fn get_name_of_focused_control() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUI::FocusControl(System.String)")]
    pub fn focus_control(name: &str) {}

    #[unity_icall("UnityEngine.GUI::Internal_GetTooltip")]
    pub fn internal_get_tooltip() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUI::Internal_SetTooltip(System.String)")]
    pub fn internal_set_tooltip(value: &str) {}

    #[unity_icall("UnityEngine.GUI::Internal_GetMouseTooltip")]
    pub fn internal_get_mouse_tooltip() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUI::DragWindow_Injected(Rect&)")]
    pub fn drag_window(position: &mut Rect) {}

    #[unity_icall("UnityEngine.GUI::BringWindowToFront(System.Int32)")]
    pub fn bring_window_to_front(window_id: i32) {}

    #[unity_icall("UnityEngine.GUI::BringWindowToBack(System.Int32)")]
    pub fn bring_window_to_back(window_id: i32) {}

    #[unity_icall("UnityEngine.GUI::FocusWindow(System.Int32)")]
    pub fn focus_window(window_id: i32) {}

    #[unity_icall("UnityEngine.GUI::UnfocusWindow")]
    pub fn unfocus_window() {}

    #[unity_icall("UnityEngine.GUI::Internal_BeginWindows")]
    pub fn internal_begin_windows() {}

    #[unity_icall("UnityEngine.GUI::Internal_EndWindows")]
    pub fn internal_end_windows() {}

    #[unity_icall("UnityEngine.GUI::Internal_Concatenate(GUIContent,GUIContent)")]
    pub fn internal_concatenate(first: Option<GUIContent>, second: Option<GUIContent>) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUI::Internal_DoModalWindow_Injected(System.Int32,System.Int32,Rect&,GUI.WindowFunction,GUIContent,GUIStyle,System.Object,Rect&)")]
    pub fn internal_do_modal_window(id: i32, instance_id: i32, client_rect: &mut Rect, func: Option<WindowFunction>, content: Option<GUIContent>, style: Option<GUIStyle>, skin: Option<SystemObject>, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.GUI::Internal_DoWindow_Injected(System.Int32,System.Int32,Rect&,GUI.WindowFunction,GUIContent,GUIStyle,System.Object,System.Boolean,Rect&)")]
    pub fn internal_do_window(id: i32, instance_id: i32, client_rect: &mut Rect, func: Option<WindowFunction>, title: Option<GUIContent>, style: Option<GUIStyle>, skin: Option<SystemObject>, force_rect_on_layout: bool, ret: &mut Rect) {}

}
