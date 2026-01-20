#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "TextAsset", namespace = "UnityEngine", inherit = "Object")]
pub struct TextAsset(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CreateOptions {
    #[default]
    None = 0,
    CreateNativeObject = 1,
}

#[unity_impl]
impl TextAsset {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(text: &str) -> Option<Self> {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_dataSize")]
    pub fn get_data_size(&self) -> i64 {}

    #[unity_icall("UnityEngine.TextAsset::get_bytes")]
    pub fn get_bytes(&self) -> Array<u8> {}

    #[unity_icall("UnityEngine.TextAsset::GetPreviewBytes(System.Int32)")]
    pub fn get_preview_bytes(&self, max_byte_count: i32) -> Array<u8> {}

    #[unity_icall("UnityEngine.TextAsset::Internal_CreateInstance(TextAsset,System.String)")]
    pub fn internal_create_instance(this: Option<TextAsset>, text: &str) {}

    #[unity_icall("UnityEngine.TextAsset::GetDataPtr")]
    pub fn get_data_ptr(&self) -> isize {}

}
