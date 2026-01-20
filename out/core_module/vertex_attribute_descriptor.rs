#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::vertex_attribute::VertexAttribute;
use super::vertex_attribute_format::VertexAttributeFormat;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "VertexAttributeDescriptor", namespace = "UnityEngine.Rendering", value_type)]
pub struct VertexAttributeDescriptor {
    pub attribute: VertexAttribute,
    pub format: VertexAttributeFormat,
    pub dimension: i32,
    pub stream: i32,
}

#[unity_impl]
impl VertexAttributeDescriptor {
    #[unity_method(name = "get_attribute")]
    pub fn get_attribute(&self) -> VertexAttribute {}

    #[unity_method(name = "set_attribute")]
    pub fn set_attribute(&self, value: VertexAttribute) {}

    #[unity_method(name = "get_format")]
    pub fn get_format(&self) -> VertexAttributeFormat {}

    #[unity_method(name = "set_format")]
    pub fn set_format(&self, value: VertexAttributeFormat) {}

    #[unity_method(name = "get_dimension")]
    pub fn get_dimension(&self) -> i32 {}

    #[unity_method(name = "set_dimension")]
    pub fn set_dimension(&self, value: i32) {}

    #[unity_method(name = "get_stream")]
    pub fn get_stream(&self) -> i32 {}

    #[unity_method(name = "set_stream")]
    pub fn set_stream(&self, value: i32) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: VertexAttributeDescriptor) -> bool {}

}
