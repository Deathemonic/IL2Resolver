#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::render_buffer_load_action::RenderBufferLoadAction;
use super::render_buffer_store_action::RenderBufferStoreAction;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderBuffer", namespace = "UnityEngine", value_type)]
pub struct RenderBuffer {
    pub m_render_texture_instance_id: i32,
    pub m_buffer_ptr: isize,
}

#[unity_impl]
impl RenderBuffer {
    #[unity_icall("UnityEngine.RenderBuffer::SetLoadAction(RenderBufferLoadAction)")]
    pub fn set_load_action(&self, action: RenderBufferLoadAction) {}

    #[unity_icall("UnityEngine.RenderBuffer::SetStoreAction(RenderBufferStoreAction)")]
    pub fn set_store_action(&self, action: RenderBufferStoreAction) {}

    #[unity_icall("UnityEngine.RenderBuffer::GetLoadAction")]
    pub fn get_load_action(&self) -> RenderBufferLoadAction {}

    #[unity_icall("UnityEngine.RenderBuffer::GetStoreAction")]
    pub fn get_store_action(&self) -> RenderBufferStoreAction {}

    #[unity_icall("UnityEngine.RenderBuffer::GetNativeRenderBufferPtr")]
    pub fn get_native_render_buffer_ptr(&self) -> isize {}

    #[unity_icall("UnityEngine.RenderBuffer::SetLoadAction_Injected(RenderBuffer&,RenderBufferLoadAction)")]
    pub fn set_load_action_1(_unity_self: &mut RenderBuffer, action: RenderBufferLoadAction) {}

    #[unity_icall("UnityEngine.RenderBuffer::SetStoreAction_Injected(RenderBuffer&,RenderBufferStoreAction)")]
    pub fn set_store_action_1(_unity_self: &mut RenderBuffer, action: RenderBufferStoreAction) {}

    #[unity_icall("UnityEngine.RenderBuffer::GetLoadAction_Injected(RenderBuffer&)")]
    pub fn get_load_action_1(_unity_self: &mut RenderBuffer) -> RenderBufferLoadAction {}

    #[unity_icall("UnityEngine.RenderBuffer::GetStoreAction_Injected(RenderBuffer&)")]
    pub fn get_store_action_1(_unity_self: &mut RenderBuffer) -> RenderBufferStoreAction {}

    #[unity_icall("UnityEngine.RenderBuffer::GetNativeRenderBufferPtr_Injected(RenderBuffer&)")]
    pub fn get_native_render_buffer_ptr_1(_unity_self: &mut RenderBuffer) -> isize {}

}
