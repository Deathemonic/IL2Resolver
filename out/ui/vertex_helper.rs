#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3, Vector4};
use crate::mscorlib::collections::{Array, List};
use crate::core_module::{Color32, Mesh};
use crate::text_rendering_module::UIVertex;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "VertexHelper", namespace = "UnityEngine.UI")]
pub struct VertexHelper(pub *mut c_void);

#[unity_impl]
impl VertexHelper {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(m: Option<Mesh>) -> Option<Self> {}

    #[unity_method(name = "get_currentVertCount")]
    pub fn get_current_vert_count(&self) -> i32 {}

    #[unity_method(name = "get_currentIndexCount")]
    pub fn get_current_index_count(&self) -> i32 {}

    #[unity_method(name = "Dispose")]
    pub fn dispose(&self) {}

    #[unity_method(name = "Clear")]
    pub fn clear(&self) {}

    #[unity_method(name = "PopulateUIVertex")]
    pub fn populate_ui_vertex(&self, vertex: &mut UIVertex, i: i32) {}

    #[unity_method(name = "SetUIVertex")]
    pub fn set_ui_vertex(&self, vertex: UIVertex, i: i32) {}

    #[unity_method(name = "FillMesh")]
    pub fn fill_mesh(&self, mesh: Option<Mesh>) {}

    #[unity_method(name = "AddVert")]
    pub fn add_vert(&self, position: Vector3, color: Color32, uv0: Vector4, uv1: Vector4, uv2: Vector4, uv3: Vector4, normal: Vector3, tangent: Vector4) {}

    #[unity_method(name = "AddVert")]
    pub fn add_vert_1(&self, position: Vector3, color: Color32, uv0: Vector4, uv1: Vector4, normal: Vector3, tangent: Vector4) {}

    #[unity_method(name = "AddVert")]
    pub fn add_vert_2(&self, position: Vector3, color: Color32, uv0: Vector4) {}

    #[unity_method(name = "AddVert")]
    pub fn add_vert_3(&self, v: UIVertex) {}

    #[unity_method(name = "AddTriangle")]
    pub fn add_triangle(&self, idx0: i32, idx1: i32, idx2: i32) {}

    #[unity_method(name = "AddUIVertexQuad")]
    pub fn add_ui_vertex_quad(&self, verts: Array<UIVertex>) {}

    #[unity_method(name = "AddUIVertexStream")]
    pub fn add_ui_vertex_stream(&self, verts: List<UIVertex>, indices: List<i32>) {}

    #[unity_method(name = "AddUIVertexTriangleStream")]
    pub fn add_ui_vertex_triangle_stream(&self, verts: List<UIVertex>) {}

    #[unity_method(name = "GetUIVertexStream")]
    pub fn get_ui_vertex_stream(&self, stream: List<UIVertex>) {}

}
