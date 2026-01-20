#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::graphics_buffer;
use super::graphics_buffer::GraphicsBuffer;
use super::mesh::Mesh;
use super::skin_quality::SkinQuality;
use super::transform::Transform;
use crate::core_module::{Component, Object, Renderer};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "SkinnedMeshRenderer", namespace = "UnityEngine", inherit = "Renderer,Component,Object")]
pub struct SkinnedMeshRenderer(pub *mut c_void);

#[unity_impl]
impl SkinnedMeshRenderer {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_quality")]
    pub fn get_quality(&self) -> SkinQuality {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_quality(SkinQuality)")]
    pub fn set_quality(&self, value: SkinQuality) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_updateWhenOffscreen")]
    pub fn get_update_when_offscreen(&self) -> bool {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_updateWhenOffscreen(System.Boolean)")]
    pub fn set_update_when_offscreen(&self, value: bool) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_forceMatrixRecalculationPerRender")]
    pub fn get_force_matrix_recalculation_per_render(&self) -> bool {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_forceMatrixRecalculationPerRender(System.Boolean)")]
    pub fn set_force_matrix_recalculation_per_render(&self, value: bool) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_rootBone")]
    pub fn get_root_bone(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_rootBone(Transform)")]
    pub fn set_root_bone(&self, value: Option<Transform>) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_bones")]
    pub fn get_bones(&self) -> Array<Transform> {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_bones(Transform[])")]
    pub fn set_bones(&self, value: Array<Transform>) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_sharedMesh")]
    pub fn get_shared_mesh(&self) -> Option<Mesh> {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_sharedMesh(Mesh)")]
    pub fn set_shared_mesh(&self, value: Option<Mesh>) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_skinnedMotionVectors")]
    pub fn get_skinned_motion_vectors(&self) -> bool {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_skinnedMotionVectors(System.Boolean)")]
    pub fn set_skinned_motion_vectors(&self, value: bool) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::get_vertexBufferTarget")]
    pub fn get_vertex_buffer_target(&self) -> graphics_buffer::Target {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::set_vertexBufferTarget(GraphicsBuffer.Target)")]
    pub fn set_vertex_buffer_target(&self, value: graphics_buffer::Target) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::GetBlendShapeWeight(System.Int32)")]
    pub fn get_blend_shape_weight(&self, index: i32) -> f32 {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::SetBlendShapeWeight(System.Int32,System.Single)")]
    pub fn set_blend_shape_weight(&self, index: i32, value: f32) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::BakeMesh(Mesh,System.Boolean)")]
    pub fn bake_mesh(&self, mesh: Option<Mesh>, use_scale: bool) {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::GetVertexBufferImpl")]
    pub fn get_vertex_buffer_impl(&self) -> Option<GraphicsBuffer> {}

    #[unity_icall("UnityEngine.SkinnedMeshRenderer::GetPreviousVertexBufferImpl")]
    pub fn get_previous_vertex_buffer_impl(&self) -> Option<GraphicsBuffer> {}

}
