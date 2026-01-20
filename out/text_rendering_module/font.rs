#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::character_info::CharacterInfo;
use super::font_style::FontStyle;
use super::font_texture_rebuild_callback::FontTextureRebuildCallback;
use crate::core_module::Material;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "Font", namespace = "UnityEngine", inherit = "Object")]
pub struct Font(pub *mut c_void);

#[unity_impl]
impl Font {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(name: &str) -> Option<Self> {}

    #[unity_icall("UnityEngine.Font::get_material")]
    pub fn get_material(&self) -> Option<Material> {}

    #[unity_icall("UnityEngine.Font::set_material(Material)")]
    pub fn set_material(&self, value: Option<Material>) {}

    #[unity_icall("UnityEngine.Font::get_fontNames")]
    pub fn get_font_names(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Font::set_fontNames(System.String[])")]
    pub fn set_font_names(&self, value: Array<SystemString>) {}

    #[unity_icall("UnityEngine.Font::get_dynamic")]
    pub fn get_dynamic(&self) -> bool {}

    #[unity_icall("UnityEngine.Font::get_ascent")]
    pub fn get_ascent(&self) -> i32 {}

    #[unity_icall("UnityEngine.Font::get_fontSize")]
    pub fn get_font_size(&self) -> i32 {}

    #[unity_icall("UnityEngine.Font::get_characterInfo")]
    pub fn get_character_info(&self) -> Array<CharacterInfo> {}

    #[unity_icall("UnityEngine.Font::set_characterInfo(CharacterInfo[])")]
    pub fn set_character_info(&self, value: Array<CharacterInfo>) {}

    #[unity_icall("UnityEngine.Font::get_lineHeight")]
    pub fn get_line_height(&self) -> i32 {}

    #[unity_method(name = "get_textureRebuildCallback")]
    pub fn get_texture_rebuild_callback(&self) -> Option<FontTextureRebuildCallback> {}

    #[unity_method(name = "set_textureRebuildCallback")]
    pub fn set_texture_rebuild_callback(&self, value: Option<FontTextureRebuildCallback>) {}

    #[unity_method(name = "add_textureRebuilt", static)]
    pub fn add_texture_rebuilt(value: *mut c_void) {}

    #[unity_method(name = "remove_textureRebuilt", static)]
    pub fn remove_texture_rebuilt(value: *mut c_void) {}

    #[unity_method(name = "CreateDynamicFontFromOSFont", static)]
    pub fn create_dynamic_font_from_os_font(fontname: &str, size: i32) -> Option<Font> {}

    #[unity_method(name = "CreateDynamicFontFromOSFont", static)]
    pub fn create_dynamic_font_from_os_font_1(fontnames: Array<SystemString>, size: i32) -> Option<Font> {}

    #[unity_method(name = "GetMaxVertsForString", static)]
    pub fn get_max_verts_for_string(str: &str) -> i32 {}

    #[unity_icall("UnityEngine.Font::GetDefault")]
    pub fn get_default() -> Option<Font> {}

    #[unity_icall("UnityEngine.Font::HasCharacter(System.Int32)")]
    pub fn has_character(&self, c: i32) -> bool {}

    #[unity_icall("UnityEngine.Font::GetOSInstalledFontNames")]
    pub fn get_os_installed_font_names() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Font::GetPathsToOSFonts")]
    pub fn get_paths_to_os_fonts() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Font::Internal_CreateFont(Font,System.String)")]
    pub fn internal_create_font(this: Option<Font>, name: &str) {}

    #[unity_icall("UnityEngine.Font::Internal_CreateFontFromPath(Font,System.String)")]
    pub fn internal_create_font_from_path(this: Option<Font>, font_path: &str) {}

    #[unity_icall("UnityEngine.Font::Internal_CreateDynamicFont(Font,System.String[],System.Int32)")]
    pub fn internal_create_dynamic_font(this: Option<Font>, _names: Array<SystemString>, size: i32) {}

    #[unity_icall("UnityEngine.Font::RequestCharactersInTexture(System.String,System.Int32,FontStyle)")]
    pub fn request_characters_in_texture(&self, characters: &str, size: i32, style: FontStyle) {}

    #[unity_icall("UnityEngine.Font::RequestCharactersInTexture(System.String,System.Int32,FontStyle)")]
    pub fn request_characters_in_texture_1(&self, characters: &str, size: i32, style: FontStyle) {}

}
