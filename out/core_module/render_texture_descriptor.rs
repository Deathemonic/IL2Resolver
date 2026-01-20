#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::graphics_format::GraphicsFormat;
use super::render_texture_creation_flags::RenderTextureCreationFlags;
use super::render_texture_format::RenderTextureFormat;
use super::render_texture_memoryless::RenderTextureMemoryless;
use super::shadow_sampling_mode::ShadowSamplingMode;
use super::texture_dimension::TextureDimension;
use super::vr_texture_usage::VRTextureUsage;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderTextureDescriptor", namespace = "UnityEngine", value_type)]
pub struct RenderTextureDescriptor {
    pub width: i32,
    pub height: i32,
    pub msaa_samples: i32,
    pub volume_depth: i32,
    pub mip_count: i32,
    pub _graphics_format: GraphicsFormat,
    pub stencil_format: GraphicsFormat,
    pub depth_stencil_format: GraphicsFormat,
    pub dimension: TextureDimension,
    pub shadow_sampling_mode: ShadowSamplingMode,
    pub vr_usage: VRTextureUsage,
    pub _flags: RenderTextureCreationFlags,
    pub memoryless: RenderTextureMemoryless,
}

#[unity_impl]
impl RenderTextureDescriptor {
    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_method(name = "set_width")]
    pub fn set_width(&self, value: i32) {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_method(name = "set_height")]
    pub fn set_height(&self, value: i32) {}

    #[unity_method(name = "get_msaaSamples")]
    pub fn get_msaa_samples(&self) -> i32 {}

    #[unity_method(name = "set_msaaSamples")]
    pub fn set_msaa_samples(&self, value: i32) {}

    #[unity_method(name = "get_volumeDepth")]
    pub fn get_volume_depth(&self) -> i32 {}

    #[unity_method(name = "set_volumeDepth")]
    pub fn set_volume_depth(&self, value: i32) {}

    #[unity_method(name = "get_mipCount")]
    pub fn get_mip_count(&self) -> i32 {}

    #[unity_method(name = "set_mipCount")]
    pub fn set_mip_count(&self, value: i32) {}

    #[unity_method(name = "get_graphicsFormat")]
    pub fn get_graphics_format(&self) -> GraphicsFormat {}

    #[unity_method(name = "set_graphicsFormat")]
    pub fn set_graphics_format(&self, value: GraphicsFormat) {}

    #[unity_method(name = "get_stencilFormat")]
    pub fn get_stencil_format(&self) -> GraphicsFormat {}

    #[unity_method(name = "set_stencilFormat")]
    pub fn set_stencil_format(&self, value: GraphicsFormat) {}

    #[unity_method(name = "get_depthStencilFormat")]
    pub fn get_depth_stencil_format(&self) -> GraphicsFormat {}

    #[unity_method(name = "set_depthStencilFormat")]
    pub fn set_depth_stencil_format(&self, value: GraphicsFormat) {}

    #[unity_method(name = "get_colorFormat")]
    pub fn get_color_format(&self) -> RenderTextureFormat {}

    #[unity_method(name = "set_colorFormat")]
    pub fn set_color_format(&self, value: RenderTextureFormat) {}

    #[unity_method(name = "get_sRGB")]
    pub fn get_s_rgb(&self) -> bool {}

    #[unity_method(name = "set_sRGB")]
    pub fn set_s_rgb(&self, value: bool) {}

    #[unity_method(name = "get_depthBufferBits")]
    pub fn get_depth_buffer_bits(&self) -> i32 {}

    #[unity_method(name = "set_depthBufferBits")]
    pub fn set_depth_buffer_bits(&self, value: i32) {}

    #[unity_method(name = "get_dimension")]
    pub fn get_dimension(&self) -> TextureDimension {}

    #[unity_method(name = "set_dimension")]
    pub fn set_dimension(&self, value: TextureDimension) {}

    #[unity_method(name = "get_shadowSamplingMode")]
    pub fn get_shadow_sampling_mode(&self) -> ShadowSamplingMode {}

    #[unity_method(name = "set_shadowSamplingMode")]
    pub fn set_shadow_sampling_mode(&self, value: ShadowSamplingMode) {}

    #[unity_method(name = "get_vrUsage")]
    pub fn get_vr_usage(&self) -> VRTextureUsage {}

    #[unity_method(name = "set_vrUsage")]
    pub fn set_vr_usage(&self, value: VRTextureUsage) {}

    #[unity_method(name = "get_flags")]
    pub fn get_flags(&self) -> RenderTextureCreationFlags {}

    #[unity_method(name = "get_memoryless")]
    pub fn get_memoryless(&self) -> RenderTextureMemoryless {}

    #[unity_method(name = "set_memoryless")]
    pub fn set_memoryless(&self, value: RenderTextureMemoryless) {}

    #[unity_method(name = "get_useMipMap")]
    pub fn get_use_mip_map(&self) -> bool {}

    #[unity_method(name = "set_useMipMap")]
    pub fn set_use_mip_map(&self, value: bool) {}

    #[unity_method(name = "get_autoGenerateMips")]
    pub fn get_auto_generate_mips(&self) -> bool {}

    #[unity_method(name = "set_autoGenerateMips")]
    pub fn set_auto_generate_mips(&self, value: bool) {}

    #[unity_method(name = "get_enableRandomWrite")]
    pub fn get_enable_random_write(&self) -> bool {}

    #[unity_method(name = "set_enableRandomWrite")]
    pub fn set_enable_random_write(&self, value: bool) {}

    #[unity_method(name = "get_bindMS")]
    pub fn get_bind_ms(&self) -> bool {}

    #[unity_method(name = "set_bindMS")]
    pub fn set_bind_ms(&self, value: bool) {}

    #[unity_method(name = "get_useDynamicScale")]
    pub fn get_use_dynamic_scale(&self) -> bool {}

    #[unity_method(name = "set_useDynamicScale")]
    pub fn set_use_dynamic_scale(&self, value: bool) {}

}
