#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "LayerMask", namespace = "UnityEngine", value_type)]
pub struct LayerMask {
    pub m_mask: i32,
}

#[unity_impl]
impl LayerMask {
    #[unity_method(name = "get_value")]
    pub fn get_value(&self) -> i32 {}

    #[unity_method(name = "set_value")]
    pub fn set_value(&self, value: i32) {}

    #[unity_icall("UnityEngine.LayerMask::LayerToName(System.Int32)")]
    pub fn layer_to_name(layer: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.LayerMask::NameToLayer(System.String)")]
    pub fn name_to_layer(layer_name: &str) -> i32 {}

}
