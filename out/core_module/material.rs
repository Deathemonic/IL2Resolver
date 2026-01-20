#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector4};
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::compute_buffer::ComputeBuffer;
use super::graphics_buffer::GraphicsBuffer;
use super::local_keyword::LocalKeyword;
use super::material_global_illumination_flags::MaterialGlobalIlluminationFlags;
use super::render_texture::RenderTexture;
use super::render_texture_sub_element::RenderTextureSubElement;
use super::shader::Shader;
use super::shader_property_flags::ShaderPropertyFlags;
use super::texture::Texture;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Material", namespace = "UnityEngine", inherit = "Object")]
pub struct Material(pub *mut c_void);

#[unity_impl]
impl Material {
    #[unity_ctor]
    pub fn new(shader: Option<Shader>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(source: Option<Material>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(contents: &str) -> Option<Self> {}

    #[unity_icall("UnityEngine.Material::get_shader")]
    pub fn get_shader(&self) -> Option<Shader> {}

    #[unity_icall("UnityEngine.Material::set_shader(Shader)")]
    pub fn set_shader(&self, value: Option<Shader>) {}

    #[unity_method(name = "get_color")]
    pub fn get_color(&self) -> Color {}

    #[unity_method(name = "set_color")]
    pub fn set_color(&self, value: Color) {}

    #[unity_method(name = "get_mainTexture")]
    pub fn get_main_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "set_mainTexture")]
    pub fn set_main_texture(&self, value: Option<Texture>) {}

    #[unity_method(name = "get_mainTextureOffset")]
    pub fn get_main_texture_offset(&self) -> Vector2 {}

    #[unity_method(name = "set_mainTextureOffset")]
    pub fn set_main_texture_offset(&self, value: Vector2) {}

    #[unity_method(name = "get_mainTextureScale")]
    pub fn get_main_texture_scale(&self) -> Vector2 {}

    #[unity_method(name = "set_mainTextureScale")]
    pub fn set_main_texture_scale(&self, value: Vector2) {}

    #[unity_icall("UnityEngine.Material::get_renderQueue")]
    pub fn get_render_queue(&self) -> i32 {}

    #[unity_icall("UnityEngine.Material::set_renderQueue(System.Int32)")]
    pub fn set_render_queue(&self, value: i32) {}

    #[unity_method(name = "get_enabledKeywords")]
    pub fn get_enabled_keywords(&self) -> Array<LocalKeyword> {}

    #[unity_method(name = "set_enabledKeywords")]
    pub fn set_enabled_keywords(&self, value: Array<LocalKeyword>) {}

    #[unity_icall("UnityEngine.Material::get_globalIlluminationFlags")]
    pub fn get_global_illumination_flags(&self) -> MaterialGlobalIlluminationFlags {}

    #[unity_icall("UnityEngine.Material::set_globalIlluminationFlags(MaterialGlobalIlluminationFlags)")]
    pub fn set_global_illumination_flags(&self, value: MaterialGlobalIlluminationFlags) {}

    #[unity_icall("UnityEngine.Material::get_doubleSidedGI")]
    pub fn get_double_sided_gi(&self) -> bool {}

    #[unity_icall("UnityEngine.Material::set_doubleSidedGI(System.Boolean)")]
    pub fn set_double_sided_gi(&self, value: bool) {}

    #[unity_icall("UnityEngine.Material::get_enableInstancing")]
    pub fn get_enable_instancing(&self) -> bool {}

    #[unity_icall("UnityEngine.Material::set_enableInstancing(System.Boolean)")]
    pub fn set_enable_instancing(&self, value: bool) {}

    #[unity_icall("UnityEngine.Material::get_passCount")]
    pub fn get_pass_count(&self) -> i32 {}

    #[unity_method(name = "get_shaderKeywords")]
    pub fn get_shader_keywords(&self) -> Array<SystemString> {}

    #[unity_method(name = "set_shaderKeywords")]
    pub fn set_shader_keywords(&self, value: Array<SystemString>) {}

    #[unity_icall("UnityEngine.Material::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_int(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Material::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_int_1(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Material::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_float(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Material::SetFloatImpl(System.Int32,System.Single)")]
    pub fn set_float_1(&self, name: i32, value: f32) {}

    #[unity_icall("UnityEngine.Material::SetIntImpl(System.Int32,System.Int32)")]
    pub fn set_integer(&self, name: i32, value: i32) {}

    #[unity_icall("UnityEngine.Material::SetIntImpl(System.Int32,System.Int32)")]
    pub fn set_integer_1(&self, name: i32, value: i32) {}

    #[unity_icall("UnityEngine.Material::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn set_vector(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.Material::SetColorImpl_Injected(System.Int32,Color&)")]
    pub fn set_vector_1(&self, name: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.Material::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Material::SetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix_1(&self, name: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Material::SetTextureImpl(System.Int32,Texture)")]
    pub fn set_texture(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.Material::SetTextureImpl(System.Int32,Texture)")]
    pub fn set_texture_1(&self, name: i32, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.Material::SetRenderTextureImpl(System.Int32,RenderTexture,RenderTextureSubElement)")]
    pub fn set_texture_2(&self, name: i32, value: Option<RenderTexture>, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Material::SetRenderTextureImpl(System.Int32,RenderTexture,RenderTextureSubElement)")]
    pub fn set_texture_3(&self, name: i32, value: Option<RenderTexture>, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Material::SetBufferImpl(System.Int32,ComputeBuffer)")]
    pub fn set_buffer(&self, name: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Material::SetBufferImpl(System.Int32,ComputeBuffer)")]
    pub fn set_buffer_1(&self, name: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Material::SetGraphicsBufferImpl(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_2(&self, name: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Material::SetGraphicsBufferImpl(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_3(&self, name: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Material::SetConstantBufferImpl(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer(&self, name: i32, value: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Material::SetConstantBufferImpl(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_1(&self, name: i32, value: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Material::SetConstantGraphicsBufferImpl(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_2(&self, name: i32, value: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Material::SetConstantGraphicsBufferImpl(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_3(&self, name: i32, value: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Material::SetFloatArrayImpl(System.Int32,System.Single[],System.Int32)")]
    pub fn set_float_array(&self, name: i32, values: Array<f32>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetFloatArrayImpl(System.Int32,System.Single[],System.Int32)")]
    pub fn set_float_array_1(&self, name: i32, values: Array<f32>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetColorArrayImpl(System.Int32,Color[],System.Int32)")]
    pub fn set_color_array(&self, name: i32, values: Array<Color>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetColorArrayImpl(System.Int32,Color[],System.Int32)")]
    pub fn set_color_array_1(&self, name: i32, values: Array<Color>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetVectorArrayImpl(System.Int32,Vector4[],System.Int32)")]
    pub fn set_vector_array(&self, name: i32, values: Array<Vector4>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetVectorArrayImpl(System.Int32,Vector4[],System.Int32)")]
    pub fn set_vector_array_1(&self, name: i32, values: Array<Vector4>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetMatrixArrayImpl(System.Int32,Matrix4x4[],System.Int32)")]
    pub fn set_matrix_array(&self, name: i32, values: Array<Matrix4x4>, count: i32) {}

    #[unity_icall("UnityEngine.Material::SetMatrixArrayImpl(System.Int32,Matrix4x4[],System.Int32)")]
    pub fn set_matrix_array_1(&self, name: i32, values: Array<Matrix4x4>, count: i32) {}

    #[unity_icall("UnityEngine.Material::GetFloatArrayCountImpl(System.Int32)")]
    pub fn get_float_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetFloatArrayCountImpl(System.Int32)")]
    pub fn get_float_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetColorArrayCountImpl(System.Int32)")]
    pub fn get_color_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetColorArrayCountImpl(System.Int32)")]
    pub fn get_color_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetVectorArrayCountImpl(System.Int32)")]
    pub fn get_vector_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetVectorArrayCountImpl(System.Int32)")]
    pub fn get_vector_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetMatrixArrayCountImpl(System.Int32)")]
    pub fn get_matrix_array(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetMatrixArrayCountImpl(System.Int32)")]
    pub fn get_matrix_array_1(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::SetTextureOffsetImpl_Injected(System.Int32,Vector2&)")]
    pub fn set_texture_offset(&self, name: i32, offset: &mut Vector2) {}

    #[unity_icall("UnityEngine.Material::SetTextureOffsetImpl_Injected(System.Int32,Vector2&)")]
    pub fn set_texture_offset_1(&self, name: i32, offset: &mut Vector2) {}

    #[unity_icall("UnityEngine.Material::SetTextureScaleImpl_Injected(System.Int32,Vector2&)")]
    pub fn set_texture_scale(&self, name: i32, scale: &mut Vector2) {}

    #[unity_icall("UnityEngine.Material::SetTextureScaleImpl_Injected(System.Int32,Vector2&)")]
    pub fn set_texture_scale_1(&self, name: i32, scale: &mut Vector2) {}

    #[unity_icall("UnityEngine.Material::CreateWithShader(Material,Shader)")]
    pub fn create_with_shader(this: Option<Material>, shader: Option<Shader>) {}

    #[unity_icall("UnityEngine.Material::CreateWithMaterial(Material,Material)")]
    pub fn create_with_material(this: Option<Material>, source: Option<Material>) {}

    #[unity_icall("UnityEngine.Material::CreateWithString(Material)")]
    pub fn create_with_string(this: Option<Material>) {}

    #[unity_icall("UnityEngine.Material::GetDefaultMaterial")]
    pub fn get_default_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Material::GetDefaultParticleMaterial")]
    pub fn get_default_particle_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Material::GetDefaultLineMaterial")]
    pub fn get_default_line_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Material::GetFirstPropertyNameIdByAttribute(ShaderPropertyFlags)")]
    pub fn get_first_property_name_id_by_attribute(&self, attribute_flag: ShaderPropertyFlags) -> i32 {}

    #[unity_icall("UnityEngine.Material::HasProperty(System.Int32)")]
    pub fn has_property(&self, name_id: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasFloatImpl(System.Int32)")]
    pub fn has_float_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasIntImpl(System.Int32)")]
    pub fn has_int_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasTextureImpl(System.Int32)")]
    pub fn has_texture_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasMatrixImpl(System.Int32)")]
    pub fn has_matrix_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasVectorImpl(System.Int32)")]
    pub fn has_vector_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasBufferImpl(System.Int32)")]
    pub fn has_buffer_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::HasConstantBufferImpl(System.Int32)")]
    pub fn has_constant_buffer_impl(&self, name: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::EnableKeyword(System.String)")]
    pub fn enable_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.Material::DisableKeyword(System.String)")]
    pub fn disable_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.Material::IsKeywordEnabled(System.String)")]
    pub fn is_keyword_enabled(&self, keyword: &str) -> bool {}

    #[unity_icall("UnityEngine.Material::EnableLocalKeyword(LocalKeyword)")]
    pub fn enable_local_keyword(&self, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Material::DisableLocalKeyword(LocalKeyword)")]
    pub fn disable_local_keyword(&self, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Material::SetLocalKeyword(LocalKeyword,System.Boolean)")]
    pub fn set_local_keyword(&self, keyword: LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Material::IsLocalKeywordEnabled(LocalKeyword)")]
    pub fn is_local_keyword_enabled(&self, keyword: LocalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Material::SetShaderPassEnabled(System.String,System.Boolean)")]
    pub fn set_shader_pass_enabled(&self, pass_name: &str, enabled: bool) {}

    #[unity_icall("UnityEngine.Material::GetShaderPassEnabled(System.String)")]
    pub fn get_shader_pass_enabled(&self, pass_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Material::GetPassName(System.Int32)")]
    pub fn get_pass_name(&self, pass: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Material::FindPass(System.String)")]
    pub fn find_pass(&self, pass_name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Material::SetOverrideTag(System.String,System.String)")]
    pub fn set_override_tag(&self, tag: &str, val: &str) {}

    #[unity_icall("UnityEngine.Material::GetTagImpl(System.String,System.Boolean,System.String)")]
    pub fn get_tag_impl(&self, tag: &str, current_sub_shader_only: bool, default_value: &str) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Material::Lerp(Material,Material,System.Single)")]
    pub fn lerp(&self, start: Option<Material>, end: Option<Material>, t: f32) {}

    #[unity_icall("UnityEngine.Material::SetPass(System.Int32)")]
    pub fn set_pass(&self, pass: i32) -> bool {}

    #[unity_icall("UnityEngine.Material::CopyPropertiesFromMaterial(Material)")]
    pub fn copy_properties_from_material(&self, mat: Option<Material>) {}

    #[unity_icall("UnityEngine.Material::CopyMatchingPropertiesFromMaterial(Material)")]
    pub fn copy_matching_properties_from_material(&self, mat: Option<Material>) {}

    #[unity_icall("UnityEngine.Material::ComputeCRC")]
    pub fn compute_crc(&self) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetTexturePropertyNames")]
    pub fn get_texture_property_names(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Material::GetTexturePropertyNameIDs")]
    pub fn get_texture_property_name_i_ds(&self) -> Array<i32> {}

    #[unity_icall("UnityEngine.Material::GetTexturePropertyNamesInternal(System.Object)")]
    pub fn get_texture_property_names_internal(&self, out_names: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Material::GetTexturePropertyNameIDsInternal(System.Object)")]
    pub fn get_texture_property_name_i_ds_internal(&self, out_names: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Material::GetIntImpl(System.Int32)")]
    pub fn get_int_impl(&self, name: i32) -> i32 {}

    #[unity_icall("UnityEngine.Material::GetFloatImpl(System.Int32)")]
    pub fn get_float_impl(&self, name: i32) -> f32 {}

    #[unity_icall("UnityEngine.Material::GetTextureImpl(System.Int32)")]
    pub fn get_texture_impl(&self, name: i32) -> Option<Texture> {}

    #[unity_icall("UnityEngine.Material::GetFloatArrayImpl(System.Int32)")]
    pub fn get_float_array_impl(&self, name: i32) -> Array<f32> {}

    #[unity_icall("UnityEngine.Material::GetVectorArrayImpl(System.Int32)")]
    pub fn get_vector_array_impl(&self, name: i32) -> Array<Vector4> {}

    #[unity_icall("UnityEngine.Material::GetColorArrayImpl(System.Int32)")]
    pub fn get_color_array_impl(&self, name: i32) -> Array<Color> {}

    #[unity_icall("UnityEngine.Material::GetMatrixArrayImpl(System.Int32)")]
    pub fn get_matrix_array_impl(&self, name: i32) -> Array<Matrix4x4> {}

    #[unity_icall("UnityEngine.Material::ExtractFloatArrayImpl(System.Int32,System.Single[])")]
    pub fn extract_float_array_impl(&self, name: i32, val: &mut Array<f32>) {}

    #[unity_icall("UnityEngine.Material::ExtractVectorArrayImpl(System.Int32,Vector4[])")]
    pub fn extract_vector_array_impl(&self, name: i32, val: &mut Array<Vector4>) {}

    #[unity_icall("UnityEngine.Material::ExtractColorArrayImpl(System.Int32,Color[])")]
    pub fn extract_color_array_impl(&self, name: i32, val: &mut Array<Color>) {}

    #[unity_icall("UnityEngine.Material::ExtractMatrixArrayImpl(System.Int32,Matrix4x4[])")]
    pub fn extract_matrix_array_impl(&self, name: i32, val: &mut Array<Matrix4x4>) {}

    #[unity_method(name = "Create", static)]
    pub fn create(script_contents: &str) -> Option<Material> {}

    #[unity_icall("UnityEngine.Material::EnableLocalKeyword_Injected(LocalKeyword&)")]
    pub fn enable_local_keyword_1(&self, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Material::DisableLocalKeyword_Injected(LocalKeyword&)")]
    pub fn disable_local_keyword_1(&self, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Material::SetLocalKeyword_Injected(LocalKeyword&,System.Boolean)")]
    pub fn set_local_keyword_1(&self, keyword: &mut LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Material::IsLocalKeywordEnabled_Injected(LocalKeyword&)")]
    pub fn is_local_keyword_enabled_1(&self, keyword: &mut LocalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Material::GetColorImpl_Injected(System.Int32,Color&)")]
    pub fn get_color_impl(&self, name: i32, ret: &mut Color) {}

    #[unity_icall("UnityEngine.Material::GetMatrixImpl_Injected(System.Int32,Matrix4x4&)")]
    pub fn get_matrix_impl(&self, name: i32, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Material::GetTextureScaleAndOffsetImpl_Injected(System.Int32,Vector4&)")]
    pub fn get_texture_scale_and_offset_impl(&self, name: i32, ret: &mut Vector4) {}

}
