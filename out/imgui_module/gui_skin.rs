#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::gui_settings::GUISettings;
use super::gui_style::GUIStyle;
use crate::text_rendering_module::Font;
use crate::core_module::{Object, ScriptableObject};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUISkin", namespace = "UnityEngine", inherit = "ScriptableObject,Object")]
pub struct GUISkin(pub *mut c_void);

#[unity_impl]
impl GUISkin {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_font")]
    pub fn get_font(&self) -> Option<Font> {}

    #[unity_method(name = "set_font")]
    pub fn set_font(&self, value: Option<Font>) {}

    #[unity_method(name = "get_box")]
    pub fn get_box(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_box")]
    pub fn set_box(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_label")]
    pub fn get_label(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_label")]
    pub fn set_label(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_textField")]
    pub fn get_text_field(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_textField")]
    pub fn set_text_field(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_textArea")]
    pub fn get_text_area(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_textArea")]
    pub fn set_text_area(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_button")]
    pub fn get_button(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_button")]
    pub fn set_button(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_toggle")]
    pub fn get_toggle(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_toggle")]
    pub fn set_toggle(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_window")]
    pub fn get_window(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_window")]
    pub fn set_window(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalSlider")]
    pub fn get_horizontal_slider(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalSlider")]
    pub fn set_horizontal_slider(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalSliderThumb")]
    pub fn get_horizontal_slider_thumb(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalSliderThumb")]
    pub fn set_horizontal_slider_thumb(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalSlider")]
    pub fn get_vertical_slider(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalSlider")]
    pub fn set_vertical_slider(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalSliderThumb")]
    pub fn get_vertical_slider_thumb(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalSliderThumb")]
    pub fn set_vertical_slider_thumb(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalScrollbar")]
    pub fn get_horizontal_scrollbar(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalScrollbar")]
    pub fn set_horizontal_scrollbar(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalScrollbarThumb")]
    pub fn get_horizontal_scrollbar_thumb(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalScrollbarThumb")]
    pub fn set_horizontal_scrollbar_thumb(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalScrollbarLeftButton")]
    pub fn get_horizontal_scrollbar_left_button(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalScrollbarLeftButton")]
    pub fn set_horizontal_scrollbar_left_button(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_horizontalScrollbarRightButton")]
    pub fn get_horizontal_scrollbar_right_button(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_horizontalScrollbarRightButton")]
    pub fn set_horizontal_scrollbar_right_button(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalScrollbar")]
    pub fn get_vertical_scrollbar(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalScrollbar")]
    pub fn set_vertical_scrollbar(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalScrollbarThumb")]
    pub fn get_vertical_scrollbar_thumb(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalScrollbarThumb")]
    pub fn set_vertical_scrollbar_thumb(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalScrollbarUpButton")]
    pub fn get_vertical_scrollbar_up_button(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalScrollbarUpButton")]
    pub fn set_vertical_scrollbar_up_button(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_verticalScrollbarDownButton")]
    pub fn get_vertical_scrollbar_down_button(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_verticalScrollbarDownButton")]
    pub fn set_vertical_scrollbar_down_button(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_scrollView")]
    pub fn get_scroll_view(&self) -> Option<GUIStyle> {}

    #[unity_method(name = "set_scrollView")]
    pub fn set_scroll_view(&self, value: Option<GUIStyle>) {}

    #[unity_method(name = "get_customStyles")]
    pub fn get_custom_styles(&self) -> Array<GUIStyle> {}

    #[unity_method(name = "set_customStyles")]
    pub fn set_custom_styles(&self, value: Array<GUIStyle>) {}

    #[unity_method(name = "get_settings")]
    pub fn get_settings(&self) -> Option<GUISettings> {}

    #[unity_method(name = "GetStyle")]
    pub fn get_style(&self, style_name: &str) -> Option<GUIStyle> {}

    #[unity_method(name = "FindStyle")]
    pub fn find_style(&self, style_name: &str) -> Option<GUIStyle> {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> *mut c_void {}

}
