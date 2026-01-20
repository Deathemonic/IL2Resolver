#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::compute_buffer::ComputeBuffer;
use super::graphics_buffer::GraphicsBuffer;
use super::local_keyword::LocalKeyword;
use super::local_keyword_space::LocalKeywordSpace;
use super::render_texture::RenderTexture;
use super::render_texture_sub_element::RenderTextureSubElement;
use super::texture::Texture;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ComputeShader", namespace = "UnityEngine", inherit = "Object")]
pub struct ComputeShader(pub *mut c_void);

#[unity_impl]
impl ComputeShader {
    #[unity_icall("UnityEngine.ComputeShader::get_keywordSpace_Injected(LocalKeywordSpace&)")]
    pub fn get_keyword_space(&self, ret: &mut LocalKeywordSpace) {}

    #[unity_method(name = "get_shaderKeywords")]
    pub fn get_shader_keywords(&self) -> Array<SystemString> {}

    #[unity_method(name = "set_shaderKeywords")]
    pub fn set_shader_keywords(&self, value: Array<SystemString>) {}

    #[unity_method(name = "get_enabledKeywords")]
    pub fn get_enabled_keywords(&self) -> Array<LocalKeyword> {}

    #[unity_method(name = "set_enabledKeywords")]
    pub fn set_enabled_keywords(&self, value: Array<LocalKeyword>) {}

    #[unity_icall("UnityEngine.ComputeShader::FindKernel(System.String)")]
    pub fn find_kernel(&self, name: &str) -> i32 {}

    #[unity_icall("UnityEngine.ComputeShader::HasKernel(System.String)")]
    pub fn has_kernel(&self, name: &str) -> bool {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_SetBuffer(System.Int32,System.Int32,ComputeBuffer)")]
    pub fn set_buffer(&self, kernel_index: i32, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_SetGraphicsBuffer(System.Int32,System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_1(&self, kernel_index: i32, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.ComputeShader::GetKernelThreadGroupSizes(System.Int32,System.UInt32&,System.UInt32&,System.UInt32&)")]
    pub fn get_kernel_thread_group_sizes(&self, kernel_index: i32, x: &mut u32, y: &mut u32, z: &mut u32) {}

    #[unity_icall("UnityEngine.ComputeShader::Dispatch(System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn dispatch(&self, kernel_index: i32, thread_groups_x: i32, thread_groups_y: i32, thread_groups_z: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::EnableKeyword(System.String)")]
    pub fn enable_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.ComputeShader::DisableKeyword(System.String)")]
    pub fn disable_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.ComputeShader::IsKeywordEnabled(System.String)")]
    pub fn is_keyword_enabled(&self, keyword: &str) -> bool {}

    #[unity_icall("UnityEngine.ComputeShader::EnableLocalKeyword(LocalKeyword)")]
    pub fn enable_local_keyword(&self, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.ComputeShader::DisableLocalKeyword(LocalKeyword)")]
    pub fn disable_local_keyword(&self, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.ComputeShader::SetLocalKeyword(LocalKeyword,System.Boolean)")]
    pub fn set_local_keyword(&self, keyword: LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.ComputeShader::IsLocalKeywordEnabled(LocalKeyword)")]
    pub fn is_local_keyword_enabled(&self, keyword: LocalKeyword) -> bool {}

    #[unity_icall("UnityEngine.ComputeShader::IsSupported(System.Int32)")]
    pub fn is_supported(&self, kernel_index: i32) -> bool {}

    #[unity_icall("UnityEngine.ComputeShader::SetFloat(System.Int32,System.Single)")]
    pub fn set_float(&self, name_id: i32, val: f32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetInt(System.Int32,System.Int32)")]
    pub fn set_int(&self, name_id: i32, val: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetVector(System.Int32,Vector4)")]
    pub fn set_vector(&self, name_id: i32, val: Vector4) {}

    #[unity_icall("UnityEngine.ComputeShader::SetMatrix(System.Int32,Matrix4x4)")]
    pub fn set_matrix(&self, name_id: i32, val: Matrix4x4) {}

    #[unity_icall("UnityEngine.ComputeShader::SetVectorArray(System.Int32,Vector4[])")]
    pub fn set_vector_array(&self, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetMatrixArray(System.Int32,Matrix4x4[])")]
    pub fn set_matrix_array(&self, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetFloatArray(System.Int32,System.Single[])")]
    pub fn set_floats(&self, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetFloatArray(System.Int32,System.Single[])")]
    pub fn set_floats_1(&self, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetIntArray(System.Int32,System.Int32[])")]
    pub fn set_ints(&self, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetIntArray(System.Int32,System.Int32[])")]
    pub fn set_ints_1(&self, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetTexture(System.Int32,System.Int32,Texture,System.Int32)")]
    pub fn set_texture(&self, kernel_index: i32, name_id: i32, texture: Option<Texture>, mip_level: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetTexture(System.Int32,System.Int32,Texture,System.Int32)")]
    pub fn set_texture_1(&self, kernel_index: i32, name_id: i32, texture: Option<Texture>, mip_level: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetTexture(System.Int32,System.Int32,Texture,System.Int32)")]
    pub fn set_texture_2(&self, kernel_index: i32, name_id: i32, texture: Option<Texture>, mip_level: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetRenderTexture(System.Int32,System.Int32,RenderTexture,System.Int32,RenderTextureSubElement)")]
    pub fn set_texture_3(&self, kernel_index: i32, name_id: i32, texture: Option<RenderTexture>, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.ComputeShader::SetRenderTexture(System.Int32,System.Int32,RenderTexture,System.Int32,RenderTextureSubElement)")]
    pub fn set_texture_4(&self, kernel_index: i32, name_id: i32, texture: Option<RenderTexture>, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.ComputeShader::SetTextureFromGlobal(System.Int32,System.Int32,System.Int32)")]
    pub fn set_texture_from_global(&self, kernel_index: i32, name_id: i32, global_texture_name_id: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_SetBuffer(System.Int32,System.Int32,ComputeBuffer)")]
    pub fn set_buffer_2(&self, kernel_index: i32, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_SetGraphicsBuffer(System.Int32,System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_3(&self, kernel_index: i32, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.ComputeShader::SetConstantComputeBuffer(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer(&self, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetConstantComputeBuffer(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_1(&self, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetConstantGraphicsBuffer(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_2(&self, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetConstantGraphicsBuffer(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_3(&self, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_DispatchIndirect(System.Int32,ComputeBuffer,System.UInt32)")]
    pub fn dispatch_indirect(&self, kernel_index: i32, args_buffer: Option<ComputeBuffer>, args_offset: u32) {}

    #[unity_icall("UnityEngine.ComputeShader::Internal_DispatchIndirectGraphicsBuffer(System.Int32,GraphicsBuffer,System.UInt32)")]
    pub fn dispatch_indirect_1(&self, kernel_index: i32, args_buffer: Option<GraphicsBuffer>, args_offset: u32) {}

    #[unity_icall("UnityEngine.ComputeShader::SetVector_Injected(System.Int32,Vector4&)")]
    pub fn set_vector_1(&self, name_id: i32, val: &mut Vector4) {}

    #[unity_icall("UnityEngine.ComputeShader::SetMatrix_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix_1(&self, name_id: i32, val: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.ComputeShader::EnableLocalKeyword_Injected(LocalKeyword&)")]
    pub fn enable_local_keyword_1(&self, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.ComputeShader::DisableLocalKeyword_Injected(LocalKeyword&)")]
    pub fn disable_local_keyword_1(&self, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.ComputeShader::SetLocalKeyword_Injected(LocalKeyword&,System.Boolean)")]
    pub fn set_local_keyword_1(&self, keyword: &mut LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.ComputeShader::IsLocalKeywordEnabled_Injected(LocalKeyword&)")]
    pub fn is_local_keyword_enabled_1(&self, keyword: &mut LocalKeyword) -> bool {}

}
