#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GlobalKeyword", namespace = "UnityEngine.Rendering", value_type)]
pub struct GlobalKeyword {
    pub m_name: Option<SystemString>,
    pub m_index: u32,
}

#[unity_impl]
impl GlobalKeyword {
    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Rendering.GlobalKeyword::GetGlobalKeywordCount")]
    pub fn get_global_keyword_count() -> u32 {}

    #[unity_icall("UnityEngine.Rendering.GlobalKeyword::GetGlobalKeywordIndex(System.String)")]
    pub fn get_global_keyword_index(keyword: &str) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.GlobalKeyword::CreateGlobalKeyword(System.String)")]
    pub fn create_global_keyword(keyword: &str) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
