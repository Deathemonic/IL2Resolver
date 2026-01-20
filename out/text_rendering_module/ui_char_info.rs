#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "UICharInfo", namespace = "UnityEngine", value_type)]
pub struct UICharInfo {
    pub cursor_pos: Vector2,
    pub char_width: f32,
}
