#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RendererList", namespace = "UnityEngine.Rendering.RendererUtils", value_type)]
pub struct RendererList {
    pub context: usize,
    pub index: u32,
    pub frame: u32,
}

#[unity_impl]
impl RendererList {
    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_icall("UnityEngine.Rendering.RendererUtils.RendererList::get_isValid_Injected(RendererList&)")]
    pub fn get_is_valid_1(_unity_self: &mut RendererList) -> bool {}

}
