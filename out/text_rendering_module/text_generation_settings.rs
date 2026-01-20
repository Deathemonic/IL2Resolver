#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2};
use super::font::Font;
use super::font_style::FontStyle;
use super::horizontal_wrap_mode::HorizontalWrapMode;
use super::text_anchor::TextAnchor;
use super::vertical_wrap_mode::VerticalWrapMode;
use crate::core_module::Color;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "TextGenerationSettings", namespace = "UnityEngine", value_type)]
pub struct TextGenerationSettings {
    pub font: Option<Font>,
    pub color: Color,
    pub font_size: i32,
    pub line_spacing: f32,
    pub rich_text: bool,
    pub scale_factor: f32,
    pub font_style: FontStyle,
    pub text_anchor: TextAnchor,
    pub align_by_geometry: bool,
    pub resize_text_for_best_fit: bool,
    pub resize_text_min_size: i32,
    pub resize_text_max_size: i32,
    pub update_bounds: bool,
    pub vertical_overflow: VerticalWrapMode,
    pub horizontal_overflow: HorizontalWrapMode,
    pub generation_extents: Vector2,
    pub pivot: Vector2,
    pub generate_out_of_bounds: bool,
}

#[unity_impl]
impl TextGenerationSettings {
    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: TextGenerationSettings) -> bool {}

}
