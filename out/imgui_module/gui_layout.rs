#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::gui;
use super::gui_content::GUIContent;
use super::gui_layout_option::GUILayoutOption;
use super::gui_style::GUIStyle;
use super::window_function::WindowFunction;
use crate::core_module::{Rect, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUILayout", namespace = "UnityEngine")]
pub struct GUILayout(pub *mut c_void);

#[unity_impl]
impl GUILayout {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "Label", static)]
    pub fn label(image: Option<Texture>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_1(text: &str, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_2(content: Option<GUIContent>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_4(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Label", static)]
    pub fn label_5(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box(image: Option<Texture>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_1(text: &str, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_2(content: Option<GUIContent>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_4(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Box", static)]
    pub fn draw_box_5(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "Button", static)]
    pub fn button(image: Option<Texture>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Button", static)]
    pub fn button_1(text: &str, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Button", static)]
    pub fn button_2(content: Option<GUIContent>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Button", static)]
    pub fn button_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Button", static)]
    pub fn button_4(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Button", static)]
    pub fn button_5(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button(image: Option<Texture>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_1(text: &str, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_2(content: Option<GUIContent>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_4(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "RepeatButton", static)]
    pub fn repeat_button_5(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "TextField", static)]
    pub fn text_field(text: &str, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextField", static)]
    pub fn text_field_1(text: &str, max_length: i32, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextField", static)]
    pub fn text_field_2(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextField", static)]
    pub fn text_field_3(text: &str, max_length: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "PasswordField", static)]
    pub fn password_field(password: &str, mask_char: u16, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "PasswordField", static)]
    pub fn password_field_1(password: &str, mask_char: u16, max_length: i32, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "PasswordField", static)]
    pub fn password_field_2(password: &str, mask_char: u16, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "PasswordField", static)]
    pub fn password_field_3(password: &str, mask_char: u16, max_length: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextArea", static)]
    pub fn text_area(text: &str, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextArea", static)]
    pub fn text_area_1(text: &str, max_length: i32, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextArea", static)]
    pub fn text_area_2(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "TextArea", static)]
    pub fn text_area_3(text: &str, max_length: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Option<SystemString> {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle(value: bool, image: Option<Texture>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle_1(value: bool, text: &str, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle_2(value: bool, content: Option<GUIContent>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle_3(value: bool, image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle_4(value: bool, text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toggle", static)]
    pub fn toggle_5(value: bool, content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> bool {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar(selected: i32, texts: Array<SystemString>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_1(selected: i32, images: Array<Texture>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_2(selected: i32, contents: Array<GUIContent>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_3(selected: i32, texts: Array<SystemString>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_4(selected: i32, images: Array<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_5(selected: i32, texts: Array<SystemString>, style: Option<GUIStyle>, button_size: gui::ToolbarButtonSize, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_6(selected: i32, images: Array<Texture>, style: Option<GUIStyle>, button_size: gui::ToolbarButtonSize, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_7(selected: i32, contents: Array<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_8(selected: i32, contents: Array<GUIContent>, style: Option<GUIStyle>, button_size: gui::ToolbarButtonSize, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_9(selected: i32, contents: Array<GUIContent>, enabled: Array<bool>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "Toolbar", static)]
    pub fn toolbar_10(selected: i32, contents: Array<GUIContent>, enabled: Array<bool>, style: Option<GUIStyle>, button_size: gui::ToolbarButtonSize, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid(selected: i32, texts: Array<SystemString>, x_count: i32, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid_1(selected: i32, images: Array<Texture>, x_count: i32, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid_2(selected: i32, content: Array<GUIContent>, x_count: i32, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid_3(selected: i32, texts: Array<SystemString>, x_count: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid_4(selected: i32, images: Array<Texture>, x_count: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "SelectionGrid", static)]
    pub fn selection_grid_5(selected: i32, contents: Array<GUIContent>, x_count: i32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> i32 {}

    #[unity_method(name = "HorizontalSlider", static)]
    pub fn horizontal_slider(value: f32, left_value: f32, right_value: f32, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "HorizontalSlider", static)]
    pub fn horizontal_slider_1(value: f32, left_value: f32, right_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "VerticalSlider", static)]
    pub fn vertical_slider(value: f32, left_value: f32, right_value: f32, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "VerticalSlider", static)]
    pub fn vertical_slider_1(value: f32, left_value: f32, right_value: f32, slider: Option<GUIStyle>, thumb: Option<GUIStyle>, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "HorizontalScrollbar", static)]
    pub fn horizontal_scrollbar(value: f32, size: f32, left_value: f32, right_value: f32, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "HorizontalScrollbar", static)]
    pub fn horizontal_scrollbar_1(value: f32, size: f32, left_value: f32, right_value: f32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "VerticalScrollbar", static)]
    pub fn vertical_scrollbar(value: f32, size: f32, top_value: f32, bottom_value: f32, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "VerticalScrollbar", static)]
    pub fn vertical_scrollbar_1(value: f32, size: f32, top_value: f32, bottom_value: f32, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> f32 {}

    #[unity_method(name = "Space", static)]
    pub fn space(pixels: f32) {}

    #[unity_method(name = "FlexibleSpace", static)]
    pub fn flexible_space() {}

    #[unity_method(name = "BeginHorizontal", static)]
    pub fn begin_horizontal(options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginHorizontal", static)]
    pub fn begin_horizontal_1(style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginHorizontal", static)]
    pub fn begin_horizontal_2(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginHorizontal", static)]
    pub fn begin_horizontal_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginHorizontal", static)]
    pub fn begin_horizontal_4(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "EndHorizontal", static)]
    pub fn end_horizontal() {}

    #[unity_method(name = "BeginVertical", static)]
    pub fn begin_vertical(options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginVertical", static)]
    pub fn begin_vertical_1(style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginVertical", static)]
    pub fn begin_vertical_2(text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginVertical", static)]
    pub fn begin_vertical_3(image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "BeginVertical", static)]
    pub fn begin_vertical_4(content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) {}

    #[unity_method(name = "EndVertical", static)]
    pub fn end_vertical() {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area(screen_rect: Rect) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_1(screen_rect: Rect, text: &str) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_2(screen_rect: Rect, image: Option<Texture>) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_3(screen_rect: Rect, content: Option<GUIContent>) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_4(screen_rect: Rect, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_5(screen_rect: Rect, text: &str, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_6(screen_rect: Rect, image: Option<Texture>, style: Option<GUIStyle>) {}

    #[unity_method(name = "BeginArea", static)]
    pub fn begin_area_7(screen_rect: Rect, content: Option<GUIContent>, style: Option<GUIStyle>) {}

    #[unity_method(name = "EndArea", static)]
    pub fn end_area() {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view(scroll_position: Vector2, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_1(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_2(scroll_position: Vector2, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_3(scroll_position: Vector2, style: Option<GUIStyle>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_4(scroll_position: Vector2, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_5(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "BeginScrollView", static)]
    pub fn begin_scroll_view_6(scroll_position: Vector2, always_show_horizontal: bool, always_show_vertical: bool, horizontal_scrollbar: Option<GUIStyle>, vertical_scrollbar: Option<GUIStyle>, background: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Vector2 {}

    #[unity_method(name = "EndScrollView", static)]
    pub fn end_scroll_view() {}

    #[unity_method(name = "Window", static)]
    pub fn window(id: i32, screen_rect: Rect, func: Option<WindowFunction>, text: &str, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Window", static)]
    pub fn window_1(id: i32, screen_rect: Rect, func: Option<WindowFunction>, image: Option<Texture>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Window", static)]
    pub fn window_2(id: i32, screen_rect: Rect, func: Option<WindowFunction>, content: Option<GUIContent>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Window", static)]
    pub fn window_3(id: i32, screen_rect: Rect, func: Option<WindowFunction>, text: &str, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Window", static)]
    pub fn window_4(id: i32, screen_rect: Rect, func: Option<WindowFunction>, image: Option<Texture>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Window", static)]
    pub fn window_5(id: i32, screen_rect: Rect, func: Option<WindowFunction>, content: Option<GUIContent>, style: Option<GUIStyle>, options: Array<GUILayoutOption>) -> Rect {}

    #[unity_method(name = "Width", static)]
    pub fn width(width: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "MinWidth", static)]
    pub fn min_width(min_width: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "MaxWidth", static)]
    pub fn max_width(max_width: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "Height", static)]
    pub fn height(height: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "MinHeight", static)]
    pub fn min_height(min_height: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "MaxHeight", static)]
    pub fn max_height(max_height: f32) -> Option<GUILayoutOption> {}

    #[unity_method(name = "ExpandWidth", static)]
    pub fn expand_width(expand: bool) -> Option<GUILayoutOption> {}

    #[unity_method(name = "ExpandHeight", static)]
    pub fn expand_height(expand: bool) -> Option<GUILayoutOption> {}

}
