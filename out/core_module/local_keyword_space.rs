#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::local_keyword::LocalKeyword;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "LocalKeywordSpace", namespace = "UnityEngine.Rendering", value_type)]
pub struct LocalKeywordSpace {
    pub m_keyword_space: isize,
}

#[unity_impl]
impl LocalKeywordSpace {
    #[unity_method(name = "get_keywords")]
    pub fn get_keywords(&self) -> Array<LocalKeyword> {}

    #[unity_method(name = "get_keywordNames")]
    pub fn get_keyword_names(&self) -> Array<SystemString> {}

    #[unity_method(name = "get_keywordCount")]
    pub fn get_keyword_count(&self) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeywordSpace::GetKeyword(System.String)")]
    pub fn get_keyword(&self, name: &str) -> LocalKeyword {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, o: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, rhs: LocalKeywordSpace) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeywordSpace::GetKeywords_Injected(LocalKeywordSpace&)")]
    pub fn get_keywords_1(_unity_self: &mut LocalKeywordSpace) -> Array<LocalKeyword> {}

    #[unity_icall("UnityEngine.Rendering.LocalKeywordSpace::GetKeywordNames_Injected(LocalKeywordSpace&)")]
    pub fn get_keyword_names_1(_unity_self: &mut LocalKeywordSpace) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Rendering.LocalKeywordSpace::GetKeywordCount_Injected(LocalKeywordSpace&)")]
    pub fn get_keyword_count_1(_unity_self: &mut LocalKeywordSpace) -> u32 {}

    #[unity_icall("UnityEngine.Rendering.LocalKeywordSpace::GetKeyword_Injected(LocalKeywordSpace&,System.String,LocalKeyword&)")]
    pub fn get_keyword_1(_unity_self: &mut LocalKeywordSpace, name: &str, ret: &mut LocalKeyword) {}

}
