#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3, Vector4};
use crate::core_module::Color32;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.TextRenderingModule", class = "UIVertex", namespace = "UnityEngine", value_type)]
pub struct UIVertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tangent: Vector4,
    pub color: Color32,
    pub uv0: Vector4,
    pub uv1: Vector4,
    pub uv2: Vector4,
    pub uv3: Vector4,
}
