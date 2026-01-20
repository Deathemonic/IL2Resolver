#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemObject};
use crate::core_module::{Color, Material, Mesh, Rect, Texture};
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UIModule", class = "CanvasRenderer", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct CanvasRenderer(pub *mut c_void);

#[unity_impl]
impl CanvasRenderer {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_hasPopInstruction")]
    pub fn get_has_pop_instruction(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_hasPopInstruction(System.Boolean)")]
    pub fn set_has_pop_instruction(&self, value: bool) {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_materialCount")]
    pub fn get_material_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_materialCount(System.Int32)")]
    pub fn set_material_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_popMaterialCount")]
    pub fn get_pop_material_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_popMaterialCount(System.Int32)")]
    pub fn set_pop_material_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_absoluteDepth")]
    pub fn get_absolute_depth(&self) -> i32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_hasMoved")]
    pub fn get_has_moved(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_cullTransparentMesh")]
    pub fn get_cull_transparent_mesh(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_cullTransparentMesh(System.Boolean)")]
    pub fn set_cull_transparent_mesh(&self, value: bool) {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_hasRectClipping")]
    pub fn get_has_rect_clipping(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_relativeDepth")]
    pub fn get_relative_depth(&self) -> i32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_cull")]
    pub fn get_cull(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_cull(System.Boolean)")]
    pub fn set_cull(&self, value: bool) {}

    #[unity_method(name = "get_isMask")]
    pub fn get_is_mask(&self) -> bool {}

    #[unity_method(name = "set_isMask")]
    pub fn set_is_mask(&self, value: bool) {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_clippingSoftness_Injected(Vector2&)")]
    pub fn get_clipping_softness(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.CanvasRenderer::set_clippingSoftness_Injected(Vector2&)")]
    pub fn set_clipping_softness(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetColor_Injected(Color&)")]
    pub fn set_color(&self, color: &mut Color) {}

    #[unity_icall("UnityEngine.CanvasRenderer::EnableRectClipping_Injected(Rect&)")]
    pub fn enable_rect_clipping(&self, rect: &mut Rect) {}

    #[unity_icall("UnityEngine.CanvasRenderer::DisableRectClipping")]
    pub fn disable_rect_clipping(&self) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetMaterial(Material,System.Int32)")]
    pub fn set_material(&self, material: Option<Material>, index: i32) {}

    #[unity_icall("UnityEngine.CanvasRenderer::GetMaterial(System.Int32)")]
    pub fn get_material(&self, index: i32) -> Option<Material> {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetPopMaterial(Material,System.Int32)")]
    pub fn set_pop_material(&self, material: Option<Material>, index: i32) {}

    #[unity_icall("UnityEngine.CanvasRenderer::GetPopMaterial(System.Int32)")]
    pub fn get_pop_material(&self, index: i32) -> Option<Material> {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetTexture(Texture)")]
    pub fn set_texture(&self, texture: Option<Texture>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetAlphaTexture(Texture)")]
    pub fn set_alpha_texture(&self, texture: Option<Texture>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetMesh(Mesh)")]
    pub fn set_mesh(&self, mesh: Option<Mesh>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::Clear")]
    pub fn clear(&self) {}

    #[unity_icall("UnityEngine.CanvasRenderer::GetInheritedAlpha")]
    pub fn get_inherited_alpha(&self) -> f32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::get_materialCount")]
    pub fn set_material_1(&self) -> i32 {}

    #[unity_icall("UnityEngine.CanvasRenderer::SplitUIVertexStreamsInternal(System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object)")]
    pub fn split_ui_vertex_streams(verts: Option<SystemObject>, positions: Option<SystemObject>, colors: Option<SystemObject>, uv0s: Option<SystemObject>, uv1s: Option<SystemObject>, uv2s: Option<SystemObject>, uv3s: Option<SystemObject>, normals: Option<SystemObject>, tangents: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::CreateUIVertexStreamInternal(System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object)")]
    pub fn create_ui_vertex_stream(verts: Option<SystemObject>, positions: Option<SystemObject>, colors: Option<SystemObject>, uv0s: Option<SystemObject>, uv1s: Option<SystemObject>, uv2s: Option<SystemObject>, uv3s: Option<SystemObject>, normals: Option<SystemObject>, tangents: Option<SystemObject>, indices: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SplitUIVertexStreamsInternal(System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object,System.Object)")]
    pub fn add_ui_vertex_stream(verts: Option<SystemObject>, positions: Option<SystemObject>, colors: Option<SystemObject>, uv0s: Option<SystemObject>, uv1s: Option<SystemObject>, uv2s: Option<SystemObject>, uv3s: Option<SystemObject>, normals: Option<SystemObject>, tangents: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SetMesh(Mesh)")]
    pub fn set_vertices(&self, mesh: Option<Mesh>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::SplitIndicesStreamsInternal(System.Object,System.Object)")]
    pub fn split_indices_streams_internal(verts: Option<SystemObject>, indices: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.CanvasRenderer::GetColor_Injected(Color&)")]
    pub fn get_color(&self, ret: &mut Color) {}

}
