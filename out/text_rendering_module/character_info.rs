#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2};
use super::font_style::FontStyle;
use crate::core_module::Rect;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "CharacterInfo", namespace = "UnityEngine", value_type)]
pub struct CharacterInfo {
    pub index: i32,
    pub uv: Rect,
    pub vert: Rect,
    pub width: f32,
    pub size: i32,
    pub style: FontStyle,
    pub flipped: bool,
}

#[unity_impl]
impl CharacterInfo {
    #[unity_method(name = "get_advance")]
    pub fn get_advance(&self) -> i32 {}

    #[unity_method(name = "set_advance")]
    pub fn set_advance(&self, value: i32) {}

    #[unity_method(name = "get_glyphWidth")]
    pub fn get_glyph_width(&self) -> i32 {}

    #[unity_method(name = "set_glyphWidth")]
    pub fn set_glyph_width(&self, value: i32) {}

    #[unity_method(name = "get_glyphHeight")]
    pub fn get_glyph_height(&self) -> i32 {}

    #[unity_method(name = "set_glyphHeight")]
    pub fn set_glyph_height(&self, value: i32) {}

    #[unity_method(name = "get_bearing")]
    pub fn get_bearing(&self) -> i32 {}

    #[unity_method(name = "set_bearing")]
    pub fn set_bearing(&self, value: i32) {}

    #[unity_method(name = "get_minY")]
    pub fn get_min_y(&self) -> i32 {}

    #[unity_method(name = "set_minY")]
    pub fn set_min_y(&self, value: i32) {}

    #[unity_method(name = "get_maxY")]
    pub fn get_max_y(&self) -> i32 {}

    #[unity_method(name = "set_maxY")]
    pub fn set_max_y(&self, value: i32) {}

    #[unity_method(name = "get_minX")]
    pub fn get_min_x(&self) -> i32 {}

    #[unity_method(name = "set_minX")]
    pub fn set_min_x(&self, value: i32) {}

    #[unity_method(name = "get_maxX")]
    pub fn get_max_x(&self) -> i32 {}

    #[unity_method(name = "set_maxX")]
    pub fn set_max_x(&self, value: i32) {}

    #[unity_method(name = "get_uvBottomLeft")]
    pub fn get_uv_bottom_left(&self) -> Vector2 {}

    #[unity_method(name = "set_uvBottomLeft")]
    pub fn set_uv_bottom_left(&self, value: Vector2) {}

    #[unity_method(name = "get_uvBottomRight")]
    pub fn get_uv_bottom_right(&self) -> Vector2 {}

    #[unity_method(name = "set_uvBottomRight")]
    pub fn set_uv_bottom_right(&self, value: Vector2) {}

    #[unity_method(name = "get_uvTopRight")]
    pub fn get_uv_top_right(&self) -> Vector2 {}

    #[unity_method(name = "set_uvTopRight")]
    pub fn set_uv_top_right(&self, value: Vector2) {}

    #[unity_method(name = "get_uvTopLeft")]
    pub fn get_uv_top_left(&self) -> Vector2 {}

    #[unity_method(name = "set_uvTopLeft")]
    pub fn set_uv_top_left(&self, value: Vector2) {}

}
