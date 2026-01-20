#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3, Vector4};
use super::bounds::Bounds;
use super::color::Color;
use super::reflection_probe_clear_flags::ReflectionProbeClearFlags;
use super::reflection_probe_mode::ReflectionProbeMode;
use super::reflection_probe_refresh_mode::ReflectionProbeRefreshMode;
use super::reflection_probe_time_slicing_mode::ReflectionProbeTimeSlicingMode;
use super::reflection_probe_type::ReflectionProbeType;
use super::render_texture::RenderTexture;
use super::texture::Texture;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ReflectionProbe", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct ReflectionProbe(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeEvent {
    #[default]
    ReflectionProbeAdded = 0,
    ReflectionProbeRemoved = 1,
}

#[unity_impl]
impl ReflectionProbe {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_type")]
    pub fn get_type(&self) -> ReflectionProbeType {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_type(ReflectionProbeType)")]
    pub fn set_type(&self, value: ReflectionProbeType) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_size_Injected(Vector3&)")]
    pub fn get_size(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_size_Injected(Vector3&)")]
    pub fn set_size(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_center_Injected(Vector3&)")]
    pub fn get_center(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_center_Injected(Vector3&)")]
    pub fn set_center(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_nearClipPlane")]
    pub fn get_near_clip_plane(&self) -> f32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_nearClipPlane(System.Single)")]
    pub fn set_near_clip_plane(&self, value: f32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_farClipPlane")]
    pub fn get_far_clip_plane(&self) -> f32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_farClipPlane(System.Single)")]
    pub fn set_far_clip_plane(&self, value: f32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_intensity")]
    pub fn get_intensity(&self) -> f32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_intensity(System.Single)")]
    pub fn set_intensity(&self, value: f32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_bounds_Injected(Bounds&)")]
    pub fn get_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_hdr")]
    pub fn get_hdr(&self) -> bool {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_hdr(System.Boolean)")]
    pub fn set_hdr(&self, value: bool) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_renderDynamicObjects")]
    pub fn get_render_dynamic_objects(&self) -> bool {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_renderDynamicObjects(System.Boolean)")]
    pub fn set_render_dynamic_objects(&self, value: bool) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_shadowDistance")]
    pub fn get_shadow_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_shadowDistance(System.Single)")]
    pub fn set_shadow_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_resolution")]
    pub fn get_resolution(&self) -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_resolution(System.Int32)")]
    pub fn set_resolution(&self, value: i32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_cullingMask")]
    pub fn get_culling_mask(&self) -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_cullingMask(System.Int32)")]
    pub fn set_culling_mask(&self, value: i32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_clearFlags")]
    pub fn get_clear_flags(&self) -> ReflectionProbeClearFlags {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_clearFlags(ReflectionProbeClearFlags)")]
    pub fn set_clear_flags(&self, value: ReflectionProbeClearFlags) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_backgroundColor_Injected(Color&)")]
    pub fn get_background_color(&self, ret: &mut Color) {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_backgroundColor_Injected(Color&)")]
    pub fn set_background_color(&self, value: &mut Color) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_blendDistance")]
    pub fn get_blend_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_blendDistance(System.Single)")]
    pub fn set_blend_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_boxProjection")]
    pub fn get_box_projection(&self) -> bool {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_boxProjection(System.Boolean)")]
    pub fn set_box_projection(&self, value: bool) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_mode")]
    pub fn get_mode(&self) -> ReflectionProbeMode {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_mode(ReflectionProbeMode)")]
    pub fn set_mode(&self, value: ReflectionProbeMode) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_importance")]
    pub fn get_importance(&self) -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_importance(System.Int32)")]
    pub fn set_importance(&self, value: i32) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_refreshMode")]
    pub fn get_refresh_mode(&self) -> ReflectionProbeRefreshMode {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_refreshMode(ReflectionProbeRefreshMode)")]
    pub fn set_refresh_mode(&self, value: ReflectionProbeRefreshMode) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_timeSlicingMode")]
    pub fn get_time_slicing_mode(&self) -> ReflectionProbeTimeSlicingMode {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_timeSlicingMode(ReflectionProbeTimeSlicingMode)")]
    pub fn set_time_slicing_mode(&self, value: ReflectionProbeTimeSlicingMode) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_bakedTexture")]
    pub fn get_baked_texture(&self) -> Option<Texture> {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_bakedTexture(Texture)")]
    pub fn set_baked_texture(&self, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_customBakedTexture")]
    pub fn get_custom_baked_texture(&self) -> Option<Texture> {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_customBakedTexture(Texture)")]
    pub fn set_custom_baked_texture(&self, value: Option<Texture>) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_realtimeTexture")]
    pub fn get_realtime_texture(&self) -> Option<RenderTexture> {}

    #[unity_icall("UnityEngine.ReflectionProbe::set_realtimeTexture(RenderTexture)")]
    pub fn set_realtime_texture(&self, value: Option<RenderTexture>) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_texture")]
    pub fn get_texture(&self) -> Option<Texture> {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_textureHDRDecodeValues_Injected(Vector4&)")]
    pub fn get_texture_hdr_decode_values(&self, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_minBakedCubemapResolution")]
    pub fn get_min_baked_cubemap_resolution() -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_maxBakedCubemapResolution")]
    pub fn get_max_baked_cubemap_resolution() -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_defaultTextureHDRDecodeValues_Injected(Vector4&)")]
    pub fn get_default_texture_hdr_decode_values(ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.ReflectionProbe::get_defaultTexture")]
    pub fn get_default_texture() -> Option<Texture> {}

    #[unity_icall("UnityEngine.ReflectionProbe::Reset")]
    pub fn reset(&self) {}

    #[unity_icall("UnityEngine.ReflectionProbe::IsFinishedRendering(System.Int32)")]
    pub fn is_finished_rendering(&self, render_id: i32) -> bool {}

    #[unity_icall("UnityEngine.ReflectionProbe::ScheduleRender(ReflectionProbeTimeSlicingMode,RenderTexture)")]
    pub fn schedule_render(&self, time_slicing_mode: ReflectionProbeTimeSlicingMode, target_texture: Option<RenderTexture>) -> i32 {}

    #[unity_icall("UnityEngine.ReflectionProbe::BlendCubemap(Texture,Texture,System.Single,RenderTexture)")]
    pub fn blend_cubemap(src: Option<Texture>, dst: Option<Texture>, blend: f32, target: Option<RenderTexture>) -> bool {}

    #[unity_icall("UnityEngine.ReflectionProbe::UpdateCachedState")]
    pub fn update_cached_state() {}

    #[unity_method(name = "add_reflectionProbeChanged", static)]
    pub fn add_reflection_probe_changed(value: *mut c_void) {}

    #[unity_method(name = "remove_reflectionProbeChanged", static)]
    pub fn remove_reflection_probe_changed(value: *mut c_void) {}

    #[unity_method(name = "add_defaultReflectionSet", static)]
    pub fn add_default_reflection_set(value: *mut c_void) {}

    #[unity_method(name = "remove_defaultReflectionSet", static)]
    pub fn remove_default_reflection_set(value: *mut c_void) {}

    #[unity_method(name = "add_defaultReflectionTexture", static)]
    pub fn add_default_reflection_texture(value: *mut c_void) {}

    #[unity_method(name = "remove_defaultReflectionTexture", static)]
    pub fn remove_default_reflection_texture(value: *mut c_void) {}

}
