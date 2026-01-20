#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use super::mesh::Mesh;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CombineInstance", namespace = "UnityEngine", value_type)]
pub struct CombineInstance {
    pub m_mesh_instance_id: i32,
    pub m_sub_mesh_index: i32,
    pub m_transform: Matrix4x4,
    pub m_lightmap_scale_offset: Vector4,
    pub m_realtime_lightmap_scale_offset: Vector4,
}

#[unity_impl]
impl CombineInstance {
    #[unity_method(name = "get_mesh")]
    pub fn get_mesh(&self) -> Option<Mesh> {}

    #[unity_method(name = "set_mesh")]
    pub fn set_mesh(&self, value: Option<Mesh>) {}

    #[unity_method(name = "get_subMeshIndex")]
    pub fn get_sub_mesh_index(&self) -> i32 {}

    #[unity_method(name = "set_subMeshIndex")]
    pub fn set_sub_mesh_index(&self, value: i32) {}

    #[unity_method(name = "get_transform")]
    pub fn get_transform(&self) -> Matrix4x4 {}

    #[unity_method(name = "set_transform")]
    pub fn set_transform(&self, value: Matrix4x4) {}

    #[unity_method(name = "get_lightmapScaleOffset")]
    pub fn get_lightmap_scale_offset(&self) -> Vector4 {}

    #[unity_method(name = "set_lightmapScaleOffset")]
    pub fn set_lightmap_scale_offset(&self, value: Vector4) {}

    #[unity_method(name = "get_realtimeLightmapScaleOffset")]
    pub fn get_realtime_lightmap_scale_offset(&self) -> Vector4 {}

    #[unity_method(name = "set_realtimeLightmapScaleOffset")]
    pub fn set_realtime_lightmap_scale_offset(&self, value: Vector4) {}

}
