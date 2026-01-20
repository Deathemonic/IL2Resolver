#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::graphics_fence_type::GraphicsFenceType;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GraphicsFence", namespace = "UnityEngine.Rendering", value_type)]
pub struct GraphicsFence {
    pub m_ptr: isize,
    pub m_version: i32,
    pub m_fence_type: GraphicsFenceType,
}

#[unity_impl]
impl GraphicsFence {
    #[unity_method(name = "get_passed")]
    pub fn get_passed(&self) -> bool {}

    #[unity_icall("UnityEngine.Rendering.GraphicsFence::HasFencePassed_Internal(System.IntPtr)")]
    pub fn has_fence_passed_internal(fence_ptr: isize) -> bool {}

    #[unity_icall("UnityEngine.Rendering.GraphicsFence::GetVersionNumber(System.IntPtr)")]
    pub fn get_version_number(fence_ptr: isize) -> i32 {}

}
