#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject};
use crate::mscorlib::collections::{Array, List};
use super::text_generation_settings::TextGenerationSettings;
use super::ui_char_info::UICharInfo;
use super::ui_line_info::UILineInfo;
use super::ui_vertex::UIVertex;
use crate::core_module::{GameObject, Rect};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "TextGenerator", namespace = "UnityEngine")]
pub struct TextGenerator(pub *mut c_void);

#[unity_impl]
impl TextGenerator {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(initial_capacity: i32) -> Option<Self> {}

    #[unity_method(name = "get_characterCountVisible")]
    pub fn get_character_count_visible(&self) -> i32 {}

    #[unity_method(name = "get_verts")]
    pub fn get_verts(&self) -> *mut c_void {}

    #[unity_method(name = "get_characters")]
    pub fn get_characters(&self) -> *mut c_void {}

    #[unity_method(name = "get_lines")]
    pub fn get_lines(&self) -> *mut c_void {}

    #[unity_method(name = "get_rectExtents")]
    pub fn get_rect_extents(&self) -> Rect {}

    #[unity_icall("UnityEngine.TextGenerator::get_vertexCount")]
    pub fn get_vertex_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.TextGenerator::get_characterCount")]
    pub fn get_character_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.TextGenerator::get_lineCount")]
    pub fn get_line_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.TextGenerator::get_fontSizeUsedForBestFit")]
    pub fn get_font_size_used_for_best_fit(&self) -> i32 {}

    #[unity_method(name = "Invalidate")]
    pub fn invalidate(&self) {}

    #[unity_method(name = "GetCharacters")]
    pub fn get_characters_1(&self, characters: List<UICharInfo>) {}

    #[unity_method(name = "GetLines")]
    pub fn get_lines_1(&self, lines: List<UILineInfo>) {}

    #[unity_method(name = "GetVertices")]
    pub fn get_vertices(&self, vertices: List<UIVertex>) {}

    #[unity_method(name = "GetPreferredWidth")]
    pub fn get_preferred_width(&self, str: &str, settings: TextGenerationSettings) -> f32 {}

    #[unity_method(name = "GetPreferredHeight")]
    pub fn get_preferred_height(&self, str: &str, settings: TextGenerationSettings) -> f32 {}

    #[unity_method(name = "PopulateWithErrors")]
    pub fn populate_with_errors(&self, str: &str, settings: TextGenerationSettings, context: Option<GameObject>) -> bool {}

    #[unity_method(name = "Populate")]
    pub fn populate(&self, str: &str, settings: TextGenerationSettings) -> bool {}

    #[unity_icall("UnityEngine.TextGenerator::Internal_Create")]
    pub fn internal_create() -> isize {}

    #[unity_icall("UnityEngine.TextGenerator::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.TextGenerator::GetVerticesArray")]
    pub fn get_vertices_array(&self) -> Array<UIVertex> {}

    #[unity_icall("UnityEngine.TextGenerator::GetCharactersArray")]
    pub fn get_characters_array(&self) -> Array<UICharInfo> {}

    #[unity_icall("UnityEngine.TextGenerator::GetLinesArray")]
    pub fn get_lines_array(&self) -> Array<UILineInfo> {}

    #[unity_icall("UnityEngine.TextGenerator::GetVerticesInternal(System.Object)")]
    pub fn get_vertices_internal(&self, vertices: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.TextGenerator::GetCharactersInternal(System.Object)")]
    pub fn get_characters_internal(&self, characters: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.TextGenerator::GetLinesInternal(System.Object)")]
    pub fn get_lines_internal(&self, lines: Option<SystemObject>) {}

}
