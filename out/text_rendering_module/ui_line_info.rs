#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "UILineInfo", namespace = "UnityEngine", value_type)]
pub struct UILineInfo {
    pub start_char_idx: i32,
    pub height: i32,
    pub top_y: f32,
    pub leading: f32,
}
