#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::builtin_render_texture_type::BuiltinRenderTextureType;
use super::cubemap_face::CubemapFace;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderTargetIdentifier", namespace = "UnityEngine.Rendering", value_type)]
pub struct RenderTargetIdentifier {
    pub m_type: BuiltinRenderTextureType,
    pub m_name_id: i32,
    pub m_instance_id: i32,
    pub m_buffer_pointer: isize,
    pub m_mip_level: i32,
    pub m_cube_face: CubemapFace,
    pub m_depth_slice: i32,
}

#[unity_impl]
impl RenderTargetIdentifier {
    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, rhs: RenderTargetIdentifier) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Option<SystemObject>) -> bool {}

}
