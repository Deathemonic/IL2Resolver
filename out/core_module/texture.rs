#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::anisotropic_filtering::AnisotropicFiltering;
use super::filter_mode::FilterMode;
use super::graphics_format::GraphicsFormat;
use super::texture_dimension::TextureDimension;
use super::texture_wrap_mode::TextureWrapMode;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Texture", namespace = "UnityEngine", inherit = "Object")]
pub struct Texture(pub *mut c_void);

#[unity_impl]
impl Texture {
    #[unity_icall("UnityEngine.Texture::get_masterTextureLimit")]
    pub fn get_master_texture_limit() -> i32 {}

    #[unity_icall("UnityEngine.Texture::set_masterTextureLimit(System.Int32)")]
    pub fn set_master_texture_limit(value: i32) {}

    #[unity_icall("UnityEngine.Texture::get_mipmapCount")]
    pub fn get_mipmap_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture::get_anisotropicFiltering")]
    pub fn get_anisotropic_filtering() -> AnisotropicFiltering {}

    #[unity_icall("UnityEngine.Texture::set_anisotropicFiltering(AnisotropicFiltering)")]
    pub fn set_anisotropic_filtering(value: AnisotropicFiltering) {}

    #[unity_method(name = "get_graphicsFormat")]
    pub fn get_graphics_format(&self) -> GraphicsFormat {}

    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_method(name = "set_width")]
    pub fn set_width(&self, value: i32) {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_method(name = "set_height")]
    pub fn set_height(&self, value: i32) {}

    #[unity_method(name = "get_dimension")]
    pub fn get_dimension(&self) -> TextureDimension {}

    #[unity_method(name = "set_dimension")]
    pub fn set_dimension(&self, value: TextureDimension) {}

    #[unity_icall("UnityEngine.Texture::get_isReadable")]
    pub fn get_is_readable(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture::get_wrapMode")]
    pub fn get_wrap_mode(&self) -> TextureWrapMode {}

    #[unity_icall("UnityEngine.Texture::set_wrapMode(TextureWrapMode)")]
    pub fn set_wrap_mode(&self, value: TextureWrapMode) {}

    #[unity_icall("UnityEngine.Texture::get_wrapModeU")]
    pub fn get_wrap_mode_u(&self) -> TextureWrapMode {}

    #[unity_icall("UnityEngine.Texture::set_wrapModeU(TextureWrapMode)")]
    pub fn set_wrap_mode_u(&self, value: TextureWrapMode) {}

    #[unity_icall("UnityEngine.Texture::get_wrapModeV")]
    pub fn get_wrap_mode_v(&self) -> TextureWrapMode {}

    #[unity_icall("UnityEngine.Texture::set_wrapModeV(TextureWrapMode)")]
    pub fn set_wrap_mode_v(&self, value: TextureWrapMode) {}

    #[unity_icall("UnityEngine.Texture::get_wrapModeW")]
    pub fn get_wrap_mode_w(&self) -> TextureWrapMode {}

    #[unity_icall("UnityEngine.Texture::set_wrapModeW(TextureWrapMode)")]
    pub fn set_wrap_mode_w(&self, value: TextureWrapMode) {}

    #[unity_icall("UnityEngine.Texture::get_filterMode")]
    pub fn get_filter_mode(&self) -> FilterMode {}

    #[unity_icall("UnityEngine.Texture::set_filterMode(FilterMode)")]
    pub fn set_filter_mode(&self, value: FilterMode) {}

    #[unity_icall("UnityEngine.Texture::get_anisoLevel")]
    pub fn get_aniso_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture::set_anisoLevel(System.Int32)")]
    pub fn set_aniso_level(&self, value: i32) {}

    #[unity_icall("UnityEngine.Texture::get_mipMapBias")]
    pub fn get_mip_map_bias(&self) -> f32 {}

    #[unity_icall("UnityEngine.Texture::set_mipMapBias(System.Single)")]
    pub fn set_mip_map_bias(&self, value: f32) {}

    #[unity_icall("UnityEngine.Texture::get_texelSize_Injected(Vector2&)")]
    pub fn get_texel_size(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Texture::get_updateCount")]
    pub fn get_update_count(&self) -> u32 {}

    #[unity_method(name = "get_isDataSRGB")]
    pub fn get_is_data_srgb(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture::get_totalTextureMemory")]
    pub fn get_total_texture_memory() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_desiredTextureMemory")]
    pub fn get_desired_texture_memory() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_targetTextureMemory")]
    pub fn get_target_texture_memory() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_currentTextureMemory")]
    pub fn get_current_texture_memory() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_nonStreamingTextureMemory")]
    pub fn get_non_streaming_texture_memory() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingMipmapUploadCount")]
    pub fn get_streaming_mipmap_upload_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingRendererCount")]
    pub fn get_streaming_renderer_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingTextureCount")]
    pub fn get_streaming_texture_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_nonStreamingTextureCount")]
    pub fn get_non_streaming_texture_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingTexturePendingLoadCount")]
    pub fn get_streaming_texture_pending_load_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingTextureLoadingCount")]
    pub fn get_streaming_texture_loading_count() -> u64 {}

    #[unity_icall("UnityEngine.Texture::get_streamingTextureForceLoadAll")]
    pub fn get_streaming_texture_force_load_all() -> bool {}

    #[unity_icall("UnityEngine.Texture::set_streamingTextureForceLoadAll(System.Boolean)")]
    pub fn set_streaming_texture_force_load_all(value: bool) {}

    #[unity_icall("UnityEngine.Texture::get_streamingTextureDiscardUnusedMips")]
    pub fn get_streaming_texture_discard_unused_mips() -> bool {}

    #[unity_icall("UnityEngine.Texture::set_streamingTextureDiscardUnusedMips(System.Boolean)")]
    pub fn set_streaming_texture_discard_unused_mips(value: bool) {}

    #[unity_icall("UnityEngine.Texture::get_allowThreadedTextureCreation")]
    pub fn get_allow_threaded_texture_creation() -> bool {}

    #[unity_icall("UnityEngine.Texture::set_allowThreadedTextureCreation(System.Boolean)")]
    pub fn set_allow_threaded_texture_creation(value: bool) {}

    #[unity_icall("UnityEngine.Texture::SetGlobalAnisotropicFilteringLimits(System.Int32,System.Int32)")]
    pub fn set_global_anisotropic_filtering_limits(forced_min: i32, global_max: i32) {}

    #[unity_icall("UnityEngine.Texture::GetDataWidth")]
    pub fn get_data_width(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture::GetDataHeight")]
    pub fn get_data_height(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture::GetNativeTexturePtr")]
    pub fn get_native_texture_ptr(&self) -> isize {}

    #[unity_icall("UnityEngine.Texture::IncrementUpdateCount")]
    pub fn increment_update_count(&self) {}

    #[unity_icall("UnityEngine.Texture::Internal_GetActiveTextureColorSpace")]
    pub fn internal_get_active_texture_color_space(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture::SetStreamingTextureMaterialDebugProperties")]
    pub fn set_streaming_texture_material_debug_properties() {}

    #[unity_icall("UnityEngine.Texture::GetPixelDataSize(System.Int32,System.Int32)")]
    pub fn get_pixel_data_size(&self, mip_level: i32, element: i32) -> u64 {}

    #[unity_icall("UnityEngine.Texture::GetPixelDataOffset(System.Int32,System.Int32)")]
    pub fn get_pixel_data_offset(&self, mip_level: i32, element: i32) -> u64 {}

}
