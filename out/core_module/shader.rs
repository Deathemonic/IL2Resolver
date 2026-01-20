#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::compute_buffer::ComputeBuffer;
use super::global_keyword::GlobalKeyword;
use super::graphics_buffer::GraphicsBuffer;
use super::local_keyword_space::LocalKeywordSpace;
use super::render_texture::RenderTexture;
use super::render_texture_sub_element::RenderTextureSubElement;
use super::shader_hardware_tier::ShaderHardwareTier;
use super::shader_property_flags::ShaderPropertyFlags;
use super::shader_property_type::ShaderPropertyType;
use super::texture::Texture;
use super::texture_dimension::TextureDimension;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Shader", namespace = "UnityEngine", inherit = "Object")]
pub struct Shader(pub *mut c_void);

#[unity_impl]
impl Shader {
    #[unity_icall("UnityEngine.Shader::get_maximumChunksOverride")]
    pub fn get_maximum_chunks_override() -> i32 {}

    #[unity_icall("UnityEngine.Shader::set_maximumChunksOverride(System.Int32)")]
    pub fn set_maximum_chunks_override(value: i32) {}

    #[unity_icall("UnityEngine.Shader::get_maximumLOD")]
    pub fn get_maximum_lod(&self) -> i32 {}

    #[unity_icall("UnityEngine.Shader::set_maximumLOD(System.Int32)")]
    pub fn set_maximum_lod(&self, value: i32) {}

    #[unity_icall("UnityEngine.Shader::get_globalMaximumLOD")]
    pub fn get_global_maximum_lod() -> i32 {}

    #[unity_icall("UnityEngine.Shader::set_globalMaximumLOD(System.Int32)")]
    pub fn set_global_maximum_lod(value: i32) {}

    #[unity_icall("UnityEngine.Shader::get_isSupported")]
    pub fn get_is_supported(&self) -> bool {}

    #[unity_icall("UnityEngine.Shader::get_globalRenderPipeline")]
    pub fn get_global_render_pipeline() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Shader::set_globalRenderPipeline(System.String)")]
    pub fn set_global_render_pipeline(value: &str) {}

    #[unity_method(name = "get_enabledGlobalKeywords", static)]
    pub fn get_enabled_global_keywords() -> Array<GlobalKeyword> {}

    #[unity_method(name = "get_globalKeywords", static)]
    pub fn get_global_keywords() -> Array<GlobalKeyword> {}

    #[unity_icall("UnityEngine.Shader::get_keywordSpace_Injected(LocalKeywordSpace&)")]
    pub fn get_keyword_space(&self, ret: &mut LocalKeywordSpace) {}

    #[unity_icall("UnityEngine.Shader::get_renderQueue")]
    pub fn get_render_queue(&self) -> i32 {}

    #[unity_icall("UnityEngine.Shader::get_passCount")]
    pub fn get_pass_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Shader::get_subshaderCount")]
    pub fn get_subshader_count(&self) -> i32 {}

    #[unity_method(name = "get_globalShaderHardwareTier", static)]
    pub fn get_global_shader_hardware_tier() -> ShaderHardwareTier {}

    #[unity_method(name = "set_globalShaderHardwareTier", static)]
    pub fn set_global_shader_hardware_tier(value: ShaderHardwareTier) {}

    #[unity_icall("UnityEngine.Shader::GetPropertyName(Shader,System.Int32)")]
    pub fn get_property_name(shader: Option<Shader>, property_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Shader::GetPropertyNameId(Shader,System.Int32)")]
    pub fn get_property_name_id(shader: Option<Shader>, property_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetPropertyType(Shader,System.Int32)")]
    pub fn get_property_type(shader: Option<Shader>, property_index: i32) -> ShaderPropertyType {}

    #[unity_icall("UnityEngine.Shader::GetPropertyDescription(Shader,System.Int32)")]
    pub fn get_property_description(shader: Option<Shader>, property_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Shader::GetPropertyFlags(Shader,System.Int32)")]
    pub fn get_property_flags(shader: Option<Shader>, property_index: i32) -> ShaderPropertyFlags {}

    #[unity_icall("UnityEngine.Shader::GetPropertyAttributes(Shader,System.Int32)")]
    pub fn get_property_attributes(shader: Option<Shader>, property_index: i32) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Shader::GetPropertyDefaultIntValue(Shader,System.Int32)")]
    pub fn get_property_default_int_value(shader: Option<Shader>, property_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetPropertyDefaultValue(Shader,System.Int32)")]
    pub fn get_property_default_value(shader: Option<Shader>, property_index: i32) -> Vector4 {}

    #[unity_icall("UnityEngine.Shader::GetPropertyTextureDimension(Shader,System.Int32)")]
    pub fn get_property_texture_dimension(shader: Option<Shader>, property_index: i32) -> TextureDimension {}

    #[unity_icall("UnityEngine.Shader::GetPropertyTextureDefaultName(Shader,System.Int32)")]
    pub fn get_property_texture_default_name(shader: Option<Shader>, property_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Shader::FindTextureStackImpl(Shader,System.Int32,System.String&,System.Int32&)")]
    pub fn find_texture_stack_impl(s: Option<Shader>, property_idx: i32, stack_name: &mut Option<SystemString>, layer_index: &mut i32) -> bool {}

    #[unity_icall("UnityEngine.Shader::GetPropertyCount")]
    pub fn get_property_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Shader::FindPropertyIndex(System.String)")]
    pub fn find_property_index(&self, property_name: &str) -> i32 {}

    #[unity_method(name = "Find", static)]
    pub fn find(name: &str) -> Option<Shader> {}

    #[unity_icall("UnityEngine.Shader::FindBuiltin(System.String)")]
    pub fn find_builtin(name: &str) -> Option<Shader> {}

    #[unity_icall("UnityEngine.Shader::GetAllGlobalKeywords")]
    pub fn get_all_global_keywords() -> Array<GlobalKeyword> {}

    #[unity_icall("UnityEngine.Shader::EnableKeyword(System.String)")]
    pub fn enable_keyword(keyword: &str) {}

    #[unity_icall("UnityEngine.Shader::DisableKeyword(System.String)")]
    pub fn disable_keyword(keyword: &str) {}

    #[unity_icall("UnityEngine.Shader::IsKeywordEnabled(System.String)")]
    pub fn is_keyword_enabled(keyword: &str) -> bool {}

    #[unity_icall("UnityEngine.Shader::EnableKeywordFast(GlobalKeyword)")]
    pub fn enable_keyword_fast(keyword: GlobalKeyword) {}

    #[unity_icall("UnityEngine.Shader::DisableKeywordFast(GlobalKeyword)")]
    pub fn disable_keyword_fast(keyword: GlobalKeyword) {}

    #[unity_icall("UnityEngine.Shader::SetKeywordFast(GlobalKeyword,System.Boolean)")]
    pub fn set_keyword_fast(keyword: GlobalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Shader::IsKeywordEnabledFast(GlobalKeyword)")]
    pub fn is_keyword_enabled_fast(keyword: GlobalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Shader::WarmupAllShaders")]
    pub fn warmup_all_shaders() {}

    #[unity_icall("UnityEngine.Shader::TagToID(System.String)")]
    pub fn tag_to_id(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::IDToTag(System.Int32)")]
    pub fn id_to_tag(name: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Shader::GetDependency(System.String)")]
    pub fn get_dependency(&self, name: &str) -> Option<Shader> {}

    #[unity_icall("UnityEngine.Shader::GetPassCountInSubshader(System.Int32)")]
    pub fn get_pass_count_in_subshader(&self, subshader_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::Internal_FindPassTagValue(System.Int32,System.Int32)")]
    pub fn internal_find_pass_tag_value(&self, pass_index: i32, tag_name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::Internal_FindPassTagValueInSubShader(System.Int32,System.Int32,System.Int32)")]
    pub fn internal_find_pass_tag_value_in_sub_shader(&self, sub_shader_index: i32, pass_index: i32, tag_name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::Internal_FindSubshaderTagValue(System.Int32,System.Int32)")]
    pub fn internal_find_subshader_tag_value(&self, sub_shader_index: i32, tag_name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalIntImpl(System.Int32)")]
    pub fn get_global_int_impl(name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalFloatImpl(System.Int32)")]
    pub fn get_global_float_impl(name: i32) -> f32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalVectorImpl(System.Int32)")]
    pub fn get_global_vector_impl(name: i32) -> Vector4 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalMatrixImpl(System.Int32)")]
    pub fn get_global_matrix_impl(name: i32) -> Matrix4x4 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalTextureImpl(System.Int32)")]
    pub fn get_global_texture_impl(name: i32) -> Option<Texture> {}

    #[unity_icall("UnityEngine.Shader::GetGlobalFloatArrayImpl(System.Int32)")]
    pub fn get_global_float_array_impl(name: i32) -> Array<f32> {}

    #[unity_icall("UnityEngine.Shader::GetGlobalVectorArrayImpl(System.Int32)")]
    pub fn get_global_vector_array_impl(name: i32) -> Array<Vector4> {}

    #[unity_icall("UnityEngine.Shader::GetGlobalMatrixArrayImpl(System.Int32)")]
    pub fn get_global_matrix_array_impl(name: i32) -> Array<Matrix4x4> {}

    #[unity_icall("UnityEngine.Shader::ExtractGlobalFloatArrayImpl(System.Int32,System.Single[])")]
    pub fn extract_global_float_array_impl(name: i32, val: &mut Array<f32>) {}

    #[unity_icall("UnityEngine.Shader::ExtractGlobalVectorArrayImpl(System.Int32,Vector4[])")]
    pub fn extract_global_vector_array_impl(name: i32, val: &mut Array<Vector4>) {}

    #[unity_icall("UnityEngine.Shader::ExtractGlobalMatrixArrayImpl(System.Int32,Matrix4x4[])")]
    pub fn extract_global_matrix_array_impl(name: i32, val: &mut Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_int(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalFloatImpl(System.Int32,System.Single)")]
    pub fn set_global_int_1(name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_float(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalFloatImpl(System.Int32,System.Single)")]
    pub fn set_global_float_1(name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_integer(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalIntImpl(System.Int32,System.Int32)")]
    pub fn set_global_integer_1(name: i32, value: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_vector(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalVectorImpl(System.Int32,Vector4)")]
    pub fn set_global_vector_1(name: i32, value: Vector4) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_color(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalVectorImpl(System.Int32,Vector4)")]
    pub fn set_global_color_1(name: i32, value: Vector4) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_matrix(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalMatrixImpl(System.Int32,Matrix4x4)")]
    pub fn set_global_matrix_1(name: i32, value: Matrix4x4) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_texture(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalTextureImpl(System.Int32,Texture)")]
    pub fn set_global_texture_1(name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_texture_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalRenderTextureImpl(System.Int32,RenderTexture,RenderTextureSubElement)")]
    pub fn set_global_texture_3(name: i32, value: Option<RenderTexture>, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_buffer(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalBufferImpl(System.Int32,ComputeBuffer)")]
    pub fn set_global_buffer_1(name: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_buffer_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalGraphicsBufferImpl(System.Int32,GraphicsBuffer)")]
    pub fn set_global_buffer_3(name: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_constant_buffer(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalConstantBufferImpl(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer_1(name: i32, value: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_constant_buffer_2(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalConstantGraphicsBufferImpl(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer_3(name: i32, value: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_float_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalFloatArrayImpl(System.Int32,System.Single[],System.Int32)")]
    pub fn set_global_float_array_1(name: i32, values: Array<f32>, count: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_vector_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalVectorArrayImpl(System.Int32,Vector4[],System.Int32)")]
    pub fn set_global_vector_array_1(name: i32, values: Array<Vector4>, count: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn set_global_matrix_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::SetGlobalMatrixArrayImpl(System.Int32,Matrix4x4[],System.Int32)")]
    pub fn set_global_matrix_array_1(name: i32, values: Array<Matrix4x4>, count: i32) {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn get_global_float_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalFloatArrayCountImpl(System.Int32)")]
    pub fn get_global_float_array_1(name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn get_global_vector_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalVectorArrayCountImpl(System.Int32)")]
    pub fn get_global_vector_array_1(name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::PropertyToID(System.String)")]
    pub fn get_global_matrix_array(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetGlobalMatrixArrayCountImpl(System.Int32)")]
    pub fn get_global_matrix_array_1(name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Shader::GetPropertyDefaultValue_Injected(Shader,System.Int32,Vector4&)")]
    pub fn get_property_default_value_1(shader: Option<Shader>, property_index: i32, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Shader::EnableKeywordFast_Injected(GlobalKeyword&)")]
    pub fn enable_keyword_fast_1(keyword: &mut GlobalKeyword) {}

    #[unity_icall("UnityEngine.Shader::DisableKeywordFast_Injected(GlobalKeyword&)")]
    pub fn disable_keyword_fast_1(keyword: &mut GlobalKeyword) {}

    #[unity_icall("UnityEngine.Shader::SetKeywordFast_Injected(GlobalKeyword&,System.Boolean)")]
    pub fn set_keyword_fast_1(keyword: &mut GlobalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Shader::IsKeywordEnabledFast_Injected(GlobalKeyword&)")]
    pub fn is_keyword_enabled_fast_1(keyword: &mut GlobalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Shader::SetGlobalVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn set_global_vector_impl(name: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.Shader::SetGlobalMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_global_matrix_impl(name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Shader::GetGlobalVectorImpl_Injected(System.Int32,Vector4&)")]
    pub fn get_global_vector_impl_1(name: i32, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Shader::GetGlobalMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn get_global_matrix_impl_1(name: i32, ret: &mut Matrix4x4) {}

}
