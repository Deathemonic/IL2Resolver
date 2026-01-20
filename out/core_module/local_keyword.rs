#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::compute_shader::ComputeShader;
use super::local_keyword_space::LocalKeywordSpace;
use super::shader::Shader;
use super::shader_keyword_type::ShaderKeywordType;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "LocalKeyword", namespace = "UnityEngine.Rendering", value_type)]
pub struct LocalKeyword {
    pub m_space_info: LocalKeywordSpace,
    pub m_name: Option<SystemString>,
    pub m_index: u32,
}

#[unity_impl]
impl LocalKeyword {
    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_isOverridable")]
    pub fn get_is_overridable(&self) -> bool {}

    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "get_type")]
    pub fn get_type(&self) -> ShaderKeywordType {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::IsOverridable(LocalKeyword)")]
    pub fn is_overridable(kw: LocalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetShaderKeywordCount(Shader)")]
    pub fn get_shader_keyword_count(shader: Option<Shader>) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetShaderKeywordIndex(Shader,System.String)")]
    pub fn get_shader_keyword_index(shader: Option<Shader>, keyword: &str) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetComputeShaderKeywordCount(ComputeShader)")]
    pub fn get_compute_shader_keyword_count(shader: Option<ComputeShader>) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetComputeShaderKeywordIndex(ComputeShader,System.String)")]
    pub fn get_compute_shader_keyword_index(shader: Option<ComputeShader>, keyword: &str) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetKeywordType(LocalKeywordSpace,System.UInt32)")]
    pub fn get_keyword_type(space_info: LocalKeywordSpace, keyword: u32) -> ShaderKeywordType {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, o: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, rhs: LocalKeyword) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::IsOverridable_Injected(LocalKeyword&)")]
    pub fn is_overridable_1(kw: &mut LocalKeyword) -> bool {}

    #[unity_icall("UnityEngine.Rendering.LocalKeyword::GetKeywordType_Injected(LocalKeywordSpace&,System.UInt32)")]
    pub fn get_keyword_type_1(space_info: &mut LocalKeywordSpace, keyword: u32) -> ShaderKeywordType {}

}
