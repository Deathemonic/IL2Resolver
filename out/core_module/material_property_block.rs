#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::compute_buffer::ComputeBuffer;
use super::graphics_buffer::GraphicsBuffer;
use super::render_texture::RenderTexture;
use super::render_texture_sub_element::RenderTextureSubElement;
use super::spherical_harmonics_l2::SphericalHarmonicsL2;
use super::texture::Texture;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "MaterialPropertyBlock", namespace = "UnityEngine")]
pub struct MaterialPropertyBlock(pub *mut c_void);

#[unity_impl]
impl MaterialPropertyBlock {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::get_isEmpty")]
    pub fn get_is_empty(&self) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetIntImpl(System.Int32)")]
    pub fn get_int_impl(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetFloatImpl(System.Int32)")]
    pub fn get_float_impl(&self, name: i32) -> f32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetTextureImpl(System.Int32)")]
    pub fn get_texture_impl(&self, name: i32) -> Option<Texture> {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasPropertyImpl(System.Int32)")]
    pub fn has_property_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasFloatImpl(System.Int32)")]
    pub fn has_float_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasIntImpl(System.Int32)")]
    pub fn has_int_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasTextureImpl(System.Int32)")]
    pub fn has_texture_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasMatrixImpl(System.Int32)")]
    pub fn has_matrix_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasVectorImpl(System.Int32)")]
    pub fn has_vector_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasBufferImpl(System.Int32)")]
    pub fn has_buffer_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::HasConstantBufferImpl(System.Int32)")]
    pub fn has_constant_buffer_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetFloatArrayImpl(System.Int32)")]
    pub fn get_float_array_impl(&self, name: i32) -> Array<f32> {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetVectorArrayImpl(System.Int32)")]
    pub fn get_vector_array_impl(&self, name: i32) -> Array<Vector4> {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetMatrixArrayImpl(System.Int32)")]
    pub fn get_matrix_array_impl(&self, name: i32) -> Array<Matrix4x4> {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::ExtractFloatArrayImpl(System.Int32,System.Single[])")]
    pub fn extract_float_array_impl(&self, name: i32, val: &mut Array<f32>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::ExtractVectorArrayImpl(System.Int32,Vector4[])")]
    pub fn extract_vector_array_impl(&self, name: i32, val: &mut Array<Vector4>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::ExtractMatrixArrayImpl(System.Int32,Matrix4x4[])")]
    pub fn extract_matrix_array_impl(&self, name: i32, val: &mut Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::CreateImpl")]
    pub fn create_impl() -> isize {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::DestroyImpl(System.IntPtr)")]
    pub fn destroy_impl(mpb: isize) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::Clear(System.Boolean)")]
    pub fn clear(&self, keep_memory: bool) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_int(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_int_1(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_float(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_float_1(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetIntImpl(System.Int32,System.Int32)")]
    pub fn set_integer(&self, name: i32, value: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetIntImpl(System.Int32,System.Int32)")]
    pub fn set_integer_1(&self, name: i32, value: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn set_vector(&self, name: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn set_vector_1(&self, name: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn set_color(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn set_color_1(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix_1(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetBufferImpl(System.Int32,ComputeBuffer)")]
    pub fn set_buffer(&self, name: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetBufferImpl(System.Int32,ComputeBuffer)")]
    pub fn set_buffer_1(&self, name: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetGraphicsBufferImpl(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_2(&self, name: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetGraphicsBufferImpl(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_3(&self, name: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetTextureImpl(System.Int32,Texture)")]
    pub fn set_texture(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetTextureImpl(System.Int32,Texture)")]
    pub fn set_texture_1(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetRenderTextureImpl(System.Int32,RenderTexture,RenderTextureSubElement)")]
    pub fn set_texture_2(&self, name: i32, value: Option<RenderTexture>, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetRenderTextureImpl(System.Int32,RenderTexture,RenderTextureSubElement)")]
    pub fn set_texture_3(&self, name: i32, value: Option<RenderTexture>, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetConstantBufferImpl(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer(&self, name: i32, value: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetConstantBufferImpl(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_1(&self, name: i32, value: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetConstantGraphicsBufferImpl(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_2(&self, name: i32, value: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetConstantGraphicsBufferImpl(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_3(&self, name: i32, value: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatArrayImpl(System.Int32,System.Single[],System.Int32)")]
    pub fn set_float_array(&self, name: i32, values: Array<f32>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatArrayImpl(System.Int32,System.Single[],System.Int32)")]
    pub fn set_float_array_1(&self, name: i32, values: Array<f32>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorArrayImpl(System.Int32,Vector4[],System.Int32)")]
    pub fn set_vector_array(&self, name: i32, values: Array<Vector4>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorArrayImpl(System.Int32,Vector4[],System.Int32)")]
    pub fn set_vector_array_1(&self, name: i32, values: Array<Vector4>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixArrayImpl(System.Int32,Matrix4x4[],System.Int32)")]
    pub fn set_matrix_array(&self, name: i32, values: Array<Matrix4x4>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixArrayImpl(System.Int32,Matrix4x4[],System.Int32)")]
    pub fn set_matrix_array_1(&self, name: i32, values: Array<Matrix4x4>, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetFloatArrayCountImpl(System.Int32)")]
    pub fn get_float_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetFloatArrayCountImpl(System.Int32)")]
    pub fn get_float_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetVectorArrayCountImpl(System.Int32)")]
    pub fn get_vector_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetVectorArrayCountImpl(System.Int32)")]
    pub fn get_vector_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetMatrixArrayCountImpl(System.Int32)")]
    pub fn get_matrix_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetMatrixArrayCountImpl(System.Int32)")]
    pub fn get_matrix_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::Internal_CopySHCoefficientArraysFrom(MaterialPropertyBlock,SphericalHarmonicsL2[],System.Int32,System.Int32,System.Int32)")]
    pub fn copy_sh_coefficient_arrays_from(properties: Option<MaterialPropertyBlock>, light_probes: Array<SphericalHarmonicsL2>, source_start: i32, dest_start: i32, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::Internal_CopyProbeOcclusionArrayFrom(MaterialPropertyBlock,Vector4[],System.Int32,System.Int32,System.Int32)")]
    pub fn copy_probe_occlusion_array_from(properties: Option<MaterialPropertyBlock>, occlusion_probes: Array<Vector4>, source_start: i32, dest_start: i32, count: i32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn add_float(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetFloatImpl(System.Int32,System.Single)")]
    pub fn add_float_1(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn add_vector(&self, name: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn add_vector_1(&self, name: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn add_color(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn add_color_1(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn add_matrix(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn add_matrix_1(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetTextureImpl(System.Int32,Texture)")]
    pub fn add_texture(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::SetTextureImpl(System.Int32,Texture)")]
    pub fn add_texture_1(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn get_vector_impl(&self, name: i32, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetColorImpl_Injected(System.Int32,Color&)")]
    pub fn get_color_impl(&self, name: i32, ret: &mut Color) {}

    #[unity_icall("UnityEngine.MaterialPropertyBlock::GetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn get_matrix_impl(&self, name: i32, ret: &mut Matrix4x4) {}

}
