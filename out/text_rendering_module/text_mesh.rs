#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::font::Font;
use super::font_style::FontStyle;
use super::text_alignment::TextAlignment;
use super::text_anchor::TextAnchor;
use crate::core_module::Color;
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "TextMesh", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct TextMesh(pub *mut c_void);

#[unity_impl]
impl TextMesh {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.TextMesh::get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.TextMesh::set_text(System.String)")]
    pub fn set_text(&self, value: &str) {}

    #[unity_icall("UnityEngine.TextMesh::get_font")]
    pub fn get_font(&self) -> Option<Font> {}

    #[unity_icall("UnityEngine.TextMesh::set_font(Font)")]
    pub fn set_font(&self, value: Option<Font>) {}

    #[unity_icall("UnityEngine.TextMesh::get_fontSize")]
    pub fn get_font_size(&self) -> i32 {}

    #[unity_icall("UnityEngine.TextMesh::set_fontSize(System.Int32)")]
    pub fn set_font_size(&self, value: i32) {}

    #[unity_icall("UnityEngine.TextMesh::get_fontStyle")]
    pub fn get_font_style(&self) -> FontStyle {}

    #[unity_icall("UnityEngine.TextMesh::set_fontStyle(FontStyle)")]
    pub fn set_font_style(&self, value: FontStyle) {}

    #[unity_icall("UnityEngine.TextMesh::get_offsetZ")]
    pub fn get_offset_z(&self) -> f32 {}

    #[unity_icall("UnityEngine.TextMesh::set_offsetZ(System.Single)")]
    pub fn set_offset_z(&self, value: f32) {}

    #[unity_icall("UnityEngine.TextMesh::get_alignment")]
    pub fn get_alignment(&self) -> TextAlignment {}

    #[unity_icall("UnityEngine.TextMesh::set_alignment(TextAlignment)")]
    pub fn set_alignment(&self, value: TextAlignment) {}

    #[unity_icall("UnityEngine.TextMesh::get_anchor")]
    pub fn get_anchor(&self) -> TextAnchor {}

    #[unity_icall("UnityEngine.TextMesh::set_anchor(TextAnchor)")]
    pub fn set_anchor(&self, value: TextAnchor) {}

    #[unity_icall("UnityEngine.TextMesh::get_characterSize")]
    pub fn get_character_size(&self) -> f32 {}

    #[unity_icall("UnityEngine.TextMesh::set_characterSize(System.Single)")]
    pub fn set_character_size(&self, value: f32) {}

    #[unity_icall("UnityEngine.TextMesh::get_lineSpacing")]
    pub fn get_line_spacing(&self) -> f32 {}

    #[unity_icall("UnityEngine.TextMesh::set_lineSpacing(System.Single)")]
    pub fn set_line_spacing(&self, value: f32) {}

    #[unity_icall("UnityEngine.TextMesh::get_tabSize")]
    pub fn get_tab_size(&self) -> f32 {}

    #[unity_icall("UnityEngine.TextMesh::set_tabSize(System.Single)")]
    pub fn set_tab_size(&self, value: f32) {}

    #[unity_icall("UnityEngine.TextMesh::get_richText")]
    pub fn get_rich_text(&self) -> bool {}

    #[unity_icall("UnityEngine.TextMesh::set_richText(System.Boolean)")]
    pub fn set_rich_text(&self, value: bool) {}

    #[unity_icall("UnityEngine.TextMesh::get_color_Injected(Color&)")]
    pub fn get_color(&self, ret: &mut Color) {}

    #[unity_icall("UnityEngine.TextMesh::set_color_Injected(Color&)")]
    pub fn set_color(&self, value: &mut Color) {}

}
