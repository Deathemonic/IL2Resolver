#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::render_buffer_load_action::RenderBufferLoadAction;
use super::render_buffer_store_action::RenderBufferStoreAction;
use super::render_target_flags::RenderTargetFlags;
use super::render_target_identifier::RenderTargetIdentifier;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderTargetBinding", namespace = "UnityEngine.Rendering", value_type)]
pub struct RenderTargetBinding {
    pub m_color_render_targets: Array<RenderTargetIdentifier>,
    pub m_depth_render_target: RenderTargetIdentifier,
    pub m_color_load_actions: Array<RenderBufferLoadAction>,
    pub m_color_store_actions: Array<RenderBufferStoreAction>,
    pub m_depth_load_action: RenderBufferLoadAction,
    pub m_depth_store_action: RenderBufferStoreAction,
    pub m_flags: RenderTargetFlags,
}

#[unity_impl]
impl RenderTargetBinding {
    #[unity_method(name = "get_colorRenderTargets")]
    pub fn get_color_render_targets(&self) -> Array<RenderTargetIdentifier> {}

    #[unity_method(name = "set_colorRenderTargets")]
    pub fn set_color_render_targets(&self, value: Array<RenderTargetIdentifier>) {}

    #[unity_method(name = "get_depthRenderTarget")]
    pub fn get_depth_render_target(&self) -> RenderTargetIdentifier {}

    #[unity_method(name = "set_depthRenderTarget")]
    pub fn set_depth_render_target(&self, value: RenderTargetIdentifier) {}

    #[unity_method(name = "get_colorLoadActions")]
    pub fn get_color_load_actions(&self) -> Array<RenderBufferLoadAction> {}

    #[unity_method(name = "set_colorLoadActions")]
    pub fn set_color_load_actions(&self, value: Array<RenderBufferLoadAction>) {}

    #[unity_method(name = "get_colorStoreActions")]
    pub fn get_color_store_actions(&self) -> Array<RenderBufferStoreAction> {}

    #[unity_method(name = "set_colorStoreActions")]
    pub fn set_color_store_actions(&self, value: Array<RenderBufferStoreAction>) {}

    #[unity_method(name = "get_depthLoadAction")]
    pub fn get_depth_load_action(&self) -> RenderBufferLoadAction {}

    #[unity_method(name = "set_depthLoadAction")]
    pub fn set_depth_load_action(&self, value: RenderBufferLoadAction) {}

    #[unity_method(name = "get_depthStoreAction")]
    pub fn get_depth_store_action(&self) -> RenderBufferStoreAction {}

    #[unity_method(name = "set_depthStoreAction")]
    pub fn set_depth_store_action(&self, value: RenderBufferStoreAction) {}

    #[unity_method(name = "get_flags")]
    pub fn get_flags(&self) -> RenderTargetFlags {}

    #[unity_method(name = "set_flags")]
    pub fn set_flags(&self, value: RenderTargetFlags) {}

}
