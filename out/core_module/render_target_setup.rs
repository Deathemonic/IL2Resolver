#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::cubemap_face::CubemapFace;
use super::render_buffer::RenderBuffer;
use super::render_buffer_load_action::RenderBufferLoadAction;
use super::render_buffer_store_action::RenderBufferStoreAction;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderTargetSetup", namespace = "UnityEngine", value_type)]
pub struct RenderTargetSetup {
    pub color: Array<RenderBuffer>,
    pub depth: RenderBuffer,
    pub mip_level: i32,
    pub cubemap_face: CubemapFace,
    pub depth_slice: i32,
    pub color_load: Array<RenderBufferLoadAction>,
    pub color_store: Array<RenderBufferStoreAction>,
    pub depth_load: RenderBufferLoadAction,
    pub depth_store: RenderBufferStoreAction,
}
