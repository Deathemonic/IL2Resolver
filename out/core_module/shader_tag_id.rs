#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ShaderTagId", namespace = "UnityEngine.Rendering", value_type)]
pub struct ShaderTagId {
    pub m_id: i32,
}

#[unity_impl]
impl ShaderTagId {
    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: ShaderTagId) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
