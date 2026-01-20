#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::gui_content::GUIContent;
use super::gui_style_state::GUIStyleState;
use super::image_position::ImagePosition;
use super::text_clipping::TextClipping;
use crate::core_module::{Color, Rect, RectOffset};
use crate::text_rendering_module::{Font, FontStyle, TextAnchor};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUIStyle", namespace = "UnityEngine")]
pub struct GUIStyle(pub *mut c_void);

#[unity_impl]
impl GUIStyle {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(other: Option<GUIStyle>) -> Option<Self> {}

    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_name")]
    pub fn set_name(&self, value: &str) {}

    #[unity_method(name = "get_normal")]
    pub fn get_normal(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_normal")]
    pub fn set_normal(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_hover")]
    pub fn get_hover(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_hover")]
    pub fn set_hover(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_active")]
    pub fn get_active(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_active")]
    pub fn set_active(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_onNormal")]
    pub fn get_on_normal(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_onNormal")]
    pub fn set_on_normal(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_onHover")]
    pub fn get_on_hover(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_onHover")]
    pub fn set_on_hover(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_onActive")]
    pub fn get_on_active(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_onActive")]
    pub fn set_on_active(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_focused")]
    pub fn get_focused(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_focused")]
    pub fn set_focused(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_onFocused")]
    pub fn get_on_focused(&self) -> Option<GUIStyleState> {}

    #[unity_method(name = "set_onFocused")]
    pub fn set_on_focused(&self, value: Option<GUIStyleState>) {}

    #[unity_method(name = "get_border")]
    pub fn get_border(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_border")]
    pub fn set_border(&self, value: Option<RectOffset>) {}

    #[unity_method(name = "get_margin")]
    pub fn get_margin(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_margin")]
    pub fn set_margin(&self, value: Option<RectOffset>) {}

    #[unity_method(name = "get_padding")]
    pub fn get_padding(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_padding")]
    pub fn set_padding(&self, value: Option<RectOffset>) {}

    #[unity_method(name = "get_overflow")]
    pub fn get_overflow(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_overflow")]
    pub fn set_overflow(&self, value: Option<RectOffset>) {}

    #[unity_method(name = "get_lineHeight")]
    pub fn get_line_height(&self) -> f32 {}

    #[unity_method(name = "get_none", static)]
    pub fn get_none() -> Option<GUIStyle> {}

    #[unity_method(name = "get_isHeightDependantOnWidth")]
    pub fn get_is_height_dependant_on_width(&self) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::get_font")]
    pub fn get_font(&self) -> Option<Font> {}

    #[unity_icall("UnityEngine.GUIStyle::set_font(Font)")]
    pub fn set_font(&self, value: Option<Font>) {}

    #[unity_icall("UnityEngine.GUIStyle::get_imagePosition")]
    pub fn get_image_position(&self) -> ImagePosition {}

    #[unity_icall("UnityEngine.GUIStyle::set_imagePosition(ImagePosition)")]
    pub fn set_image_position(&self, value: ImagePosition) {}

    #[unity_icall("UnityEngine.GUIStyle::get_alignment")]
    pub fn get_alignment(&self) -> TextAnchor {}

    #[unity_icall("UnityEngine.GUIStyle::set_alignment(TextAnchor)")]
    pub fn set_alignment(&self, value: TextAnchor) {}

    #[unity_icall("UnityEngine.GUIStyle::get_wordWrap")]
    pub fn get_word_wrap(&self) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::set_wordWrap(System.Boolean)")]
    pub fn set_word_wrap(&self, value: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::get_clipping")]
    pub fn get_clipping(&self) -> TextClipping {}

    #[unity_icall("UnityEngine.GUIStyle::set_clipping(TextClipping)")]
    pub fn set_clipping(&self, value: TextClipping) {}

    #[unity_icall("UnityEngine.GUIStyle::get_contentOffset_Injected(Vector2&)")]
    pub fn get_content_offset(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::set_contentOffset_Injected(Vector2&)")]
    pub fn set_content_offset(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::get_fixedWidth")]
    pub fn get_fixed_width(&self) -> f32 {}

    #[unity_icall("UnityEngine.GUIStyle::set_fixedWidth(System.Single)")]
    pub fn set_fixed_width(&self, value: f32) {}

    #[unity_icall("UnityEngine.GUIStyle::get_fixedHeight")]
    pub fn get_fixed_height(&self) -> f32 {}

    #[unity_icall("UnityEngine.GUIStyle::set_fixedHeight(System.Single)")]
    pub fn set_fixed_height(&self, value: f32) {}

    #[unity_icall("UnityEngine.GUIStyle::get_stretchWidth")]
    pub fn get_stretch_width(&self) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::set_stretchWidth(System.Boolean)")]
    pub fn set_stretch_width(&self, value: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::get_stretchHeight")]
    pub fn get_stretch_height(&self) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::set_stretchHeight(System.Boolean)")]
    pub fn set_stretch_height(&self, value: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::get_fontSize")]
    pub fn get_font_size(&self) -> i32 {}

    #[unity_icall("UnityEngine.GUIStyle::set_fontSize(System.Int32)")]
    pub fn set_font_size(&self, value: i32) {}

    #[unity_icall("UnityEngine.GUIStyle::get_fontStyle")]
    pub fn get_font_style(&self) -> FontStyle {}

    #[unity_icall("UnityEngine.GUIStyle::set_fontStyle(FontStyle)")]
    pub fn set_font_style(&self, value: FontStyle) {}

    #[unity_icall("UnityEngine.GUIStyle::get_richText")]
    pub fn get_rich_text(&self) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::set_richText(System.Boolean)")]
    pub fn set_rich_text(&self, value: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::get_clipOffset_Injected(Vector2&)")]
    pub fn get_clip_offset(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::set_clipOffset_Injected(Vector2&)")]
    pub fn set_clip_offset(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn draw(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn draw_1(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn draw_2(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn draw_3(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn draw_4(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetCursorFlashOffset")]
    pub fn draw_with_text_selection() -> f32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Create(GUIStyle)")]
    pub fn internal_create(this: Option<GUIStyle>) -> isize {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Copy(GUIStyle,GUIStyle)")]
    pub fn internal_copy(this: Option<GUIStyle>, other: Option<GUIStyle>) -> isize {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(this: isize) {}

    #[unity_icall("UnityEngine.GUIStyle::GetStyleStatePtr(System.Int32)")]
    pub fn get_style_state_ptr(&self, idx: i32) -> isize {}

    #[unity_icall("UnityEngine.GUIStyle::AssignStyleState(System.Int32,System.IntPtr)")]
    pub fn assign_style_state(&self, idx: i32, src_style_state: isize) {}

    #[unity_icall("UnityEngine.GUIStyle::GetRectOffsetPtr(System.Int32)")]
    pub fn get_rect_offset_ptr(&self, idx: i32) -> isize {}

    #[unity_icall("UnityEngine.GUIStyle::AssignRectOffset(System.Int32,System.IntPtr)")]
    pub fn assign_rect_offset(&self, idx: i32, src_rect_offset: isize) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetLineHeight(System.IntPtr)")]
    pub fn internal_get_line_height(target: isize) -> f32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw2(Rect,GUIContent,System.Int32,System.Boolean)")]
    pub fn internal_draw2(&self, position: Rect, content: Option<GUIContent>, control_id: i32, on: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_DrawCursor(Rect,GUIContent,System.Int32,Color)")]
    pub fn internal_draw_cursor(&self, position: Rect, content: Option<GUIContent>, pos: i32, cursor_color: Color) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_DrawWithTextSelection(Rect,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean,System.Boolean,System.Int32,System.Int32,Color,Color)")]
    pub fn internal_draw_with_text_selection(&self, screen_rect: Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool, draw_selection_as_composition: bool, cursor_first: i32, cursor_last: i32, cursor_color: Color, selection_color: Color) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetCursorPixelPosition(Rect,GUIContent,System.Int32)")]
    pub fn internal_get_cursor_pixel_position(&self, position: Rect, content: Option<GUIContent>, cursor_string_index: i32) -> Vector2 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetCursorStringIndex(Rect,GUIContent,Vector2)")]
    pub fn internal_get_cursor_string_index(&self, position: Rect, content: Option<GUIContent>, cursor_pixel_position: Vector2) -> i32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetSelectedRenderedText(Rect,GUIContent,System.Int32,System.Int32)")]
    pub fn internal_get_selected_rendered_text(&self, local_position: Rect, m_content: Option<GUIContent>, select_index: i32, cursor_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetHyperlinksRect(Rect,GUIContent)")]
    pub fn internal_get_hyperlinks_rect(&self, local_position: Rect, m_content: Option<GUIContent>) -> Array<Rect> {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetNumCharactersThatFitWithinWidth(System.String,System.Single)")]
    pub fn internal_get_num_characters_that_fit_within_width(&self, text: &str, width: f32) -> i32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcSize(GUIContent)")]
    pub fn internal_calc_size(&self, content: Option<GUIContent>) -> Vector2 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcSizeWithConstraints(GUIContent,Vector2)")]
    pub fn internal_calc_size_with_constraints(&self, content: Option<GUIContent>, max_size: Vector2) -> Vector2 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcHeight(GUIContent,System.Single)")]
    pub fn internal_calc_height(&self, content: Option<GUIContent>, width: f32) -> f32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcMinMaxWidth(GUIContent)")]
    pub fn internal_calc_min_max_width(&self, content: Option<GUIContent>) -> Vector2 {}

    #[unity_icall("UnityEngine.GUIStyle::SetMouseTooltip(System.String,Rect)")]
    pub fn set_mouse_tooltip(tooltip: &str, screen_rect: Rect) {}

    #[unity_icall("UnityEngine.GUIStyle::IsTooltipActive(System.String)")]
    pub fn is_tooltip_active(tooltip: &str) -> bool {}

    #[unity_icall("UnityEngine.GUIStyle::SetDefaultFont(Font)")]
    pub fn set_default_font(font: Option<Font>) {}

    #[unity_icall("UnityEngine.GUIStyle::get_Internal_clipOffset_Injected(Vector2&)")]
    pub fn get_internal_clip_offset(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::set_Internal_clipOffset_Injected(Vector2&)")]
    pub fn set_internal_clip_offset(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw_Injected(Rect&,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean)")]
    pub fn internal_draw(&self, screen_rect: &mut Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_Draw2_Injected(Rect&,GUIContent,System.Int32,System.Boolean)")]
    pub fn internal_draw2_1(&self, position: &mut Rect, content: Option<GUIContent>, control_id: i32, on: bool) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_DrawCursor_Injected(Rect&,GUIContent,System.Int32,Color&)")]
    pub fn internal_draw_cursor_1(&self, position: &mut Rect, content: Option<GUIContent>, pos: i32, cursor_color: &mut Color) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_DrawWithTextSelection_Injected(Rect&,GUIContent,System.Boolean,System.Boolean,System.Boolean,System.Boolean,System.Boolean,System.Int32,System.Int32,Color&,Color&)")]
    pub fn internal_draw_with_text_selection_1(&self, screen_rect: &mut Rect, content: Option<GUIContent>, is_hover: bool, is_active: bool, on: bool, has_keyboard_focus: bool, draw_selection_as_composition: bool, cursor_first: i32, cursor_last: i32, cursor_color: &mut Color, selection_color: &mut Color) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetCursorPixelPosition_Injected(Rect&,GUIContent,System.Int32,Vector2&)")]
    pub fn internal_get_cursor_pixel_position_1(&self, position: &mut Rect, content: Option<GUIContent>, cursor_string_index: i32, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetCursorStringIndex_Injected(Rect&,GUIContent,Vector2&)")]
    pub fn internal_get_cursor_string_index_1(&self, position: &mut Rect, content: Option<GUIContent>, cursor_pixel_position: &mut Vector2) -> i32 {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetSelectedRenderedText_Injected(Rect&,GUIContent,System.Int32,System.Int32)")]
    pub fn internal_get_selected_rendered_text_1(&self, local_position: &mut Rect, m_content: Option<GUIContent>, select_index: i32, cursor_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_GetHyperlinksRect_Injected(Rect&,GUIContent)")]
    pub fn internal_get_hyperlinks_rect_1(&self, local_position: &mut Rect, m_content: Option<GUIContent>) -> Array<Rect> {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcSize_Injected(GUIContent,Vector2&)")]
    pub fn internal_calc_size_1(&self, content: Option<GUIContent>, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcSizeWithConstraints_Injected(GUIContent,Vector2&,Vector2&)")]
    pub fn internal_calc_size_with_constraints_1(&self, content: Option<GUIContent>, max_size: &mut Vector2, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::Internal_CalcMinMaxWidth_Injected(GUIContent,Vector2&)")]
    pub fn internal_calc_min_max_width_1(&self, content: Option<GUIContent>, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIStyle::SetMouseTooltip_Injected(System.String,Rect&)")]
    pub fn set_mouse_tooltip_1(tooltip: &str, screen_rect: &mut Rect) {}

}
