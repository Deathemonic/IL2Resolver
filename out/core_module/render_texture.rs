#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::camera;
use super::default_format::DefaultFormat;
use super::graphics_format::GraphicsFormat;
use super::render_buffer::RenderBuffer;
use super::render_texture_descriptor::RenderTextureDescriptor;
use super::render_texture_format::RenderTextureFormat;
use super::render_texture_memoryless::RenderTextureMemoryless;
use super::render_texture_read_write::RenderTextureReadWrite;
use super::shadow_sampling_mode::ShadowSamplingMode;
use super::texture_dimension::TextureDimension;
use super::vr_texture_usage::VRTextureUsage;
use crate::core_module::{Object, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderTexture", namespace = "UnityEngine", inherit = "Texture,Object")]
pub struct RenderTexture(pub *mut c_void);

#[unity_impl]
impl RenderTexture {
    #[unity_ctor]
    pub fn new(desc: RenderTextureDescriptor) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(texture_to_copy: Option<RenderTexture>) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(width: i32, height: i32, depth: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(width: i32, height: i32, depth: i32, format: DefaultFormat) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(width: i32, height: i32, depth: i32, format: GraphicsFormat) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_5(width: i32, height: i32, color_format: GraphicsFormat, depth_stencil_format: GraphicsFormat) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_6(width: i32, height: i32, depth: i32, format: RenderTextureFormat) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_7(width: i32, height: i32, depth: i32, format: GraphicsFormat, mip_count: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_8(width: i32, height: i32, color_format: GraphicsFormat, depth_stencil_format: GraphicsFormat, mip_count: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_9(width: i32, height: i32, depth: i32, format: RenderTextureFormat, read_write: RenderTextureReadWrite) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_10(width: i32, height: i32, depth: i32, format: RenderTextureFormat, mip_count: i32) -> Option<Self> {}

    #[unity_icall("UnityEngine.RenderTexture::get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_icall("UnityEngine.RenderTexture::set_width(System.Int32)")]
    pub fn set_width(&self, value: i32) {}

    #[unity_icall("UnityEngine.RenderTexture::get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_icall("UnityEngine.RenderTexture::set_height(System.Int32)")]
    pub fn set_height(&self, value: i32) {}

    #[unity_icall("UnityEngine.RenderTexture::get_dimension")]
    pub fn get_dimension(&self) -> TextureDimension {}

    #[unity_icall("UnityEngine.RenderTexture::set_dimension(TextureDimension)")]
    pub fn set_dimension(&self, value: TextureDimension) {}

    #[unity_icall("UnityEngine.RenderTexture::get_graphicsFormat")]
    pub fn get_graphics_format(&self) -> GraphicsFormat {}

    #[unity_icall("UnityEngine.RenderTexture::set_graphicsFormat(GraphicsFormat)")]
    pub fn set_graphics_format(&self, value: GraphicsFormat) {}

    #[unity_icall("UnityEngine.RenderTexture::get_useMipMap")]
    pub fn get_use_mip_map(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::set_useMipMap(System.Boolean)")]
    pub fn set_use_mip_map(&self, value: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::get_sRGB")]
    pub fn get_s_rgb(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::get_vrUsage")]
    pub fn get_vr_usage(&self) -> VRTextureUsage {}

    #[unity_icall("UnityEngine.RenderTexture::set_vrUsage(VRTextureUsage)")]
    pub fn set_vr_usage(&self, value: VRTextureUsage) {}

    #[unity_icall("UnityEngine.RenderTexture::get_memorylessMode")]
    pub fn get_memoryless_mode(&self) -> RenderTextureMemoryless {}

    #[unity_icall("UnityEngine.RenderTexture::set_memorylessMode(RenderTextureMemoryless)")]
    pub fn set_memoryless_mode(&self, value: RenderTextureMemoryless) {}

    #[unity_method(name = "get_format")]
    pub fn get_format(&self) -> RenderTextureFormat {}

    #[unity_method(name = "set_format")]
    pub fn set_format(&self, value: RenderTextureFormat) {}

    #[unity_icall("UnityEngine.RenderTexture::get_stencilFormat")]
    pub fn get_stencil_format(&self) -> GraphicsFormat {}

    #[unity_icall("UnityEngine.RenderTexture::set_stencilFormat(GraphicsFormat)")]
    pub fn set_stencil_format(&self, value: GraphicsFormat) {}

    #[unity_icall("UnityEngine.RenderTexture::get_depthStencilFormat")]
    pub fn get_depth_stencil_format(&self) -> GraphicsFormat {}

    #[unity_icall("UnityEngine.RenderTexture::set_depthStencilFormat(GraphicsFormat)")]
    pub fn set_depth_stencil_format(&self, value: GraphicsFormat) {}

    #[unity_icall("UnityEngine.RenderTexture::get_autoGenerateMips")]
    pub fn get_auto_generate_mips(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::set_autoGenerateMips(System.Boolean)")]
    pub fn set_auto_generate_mips(&self, value: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::get_volumeDepth")]
    pub fn get_volume_depth(&self) -> i32 {}

    #[unity_icall("UnityEngine.RenderTexture::set_volumeDepth(System.Int32)")]
    pub fn set_volume_depth(&self, value: i32) {}

    #[unity_icall("UnityEngine.RenderTexture::get_antiAliasing")]
    pub fn get_anti_aliasing(&self) -> i32 {}

    #[unity_icall("UnityEngine.RenderTexture::set_antiAliasing(System.Int32)")]
    pub fn set_anti_aliasing(&self, value: i32) {}

    #[unity_icall("UnityEngine.RenderTexture::get_bindTextureMS")]
    pub fn get_bind_texture_ms(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::set_bindTextureMS(System.Boolean)")]
    pub fn set_bind_texture_ms(&self, value: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::get_enableRandomWrite")]
    pub fn get_enable_random_write(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::set_enableRandomWrite(System.Boolean)")]
    pub fn set_enable_random_write(&self, value: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::get_useDynamicScale")]
    pub fn get_use_dynamic_scale(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::set_useDynamicScale(System.Boolean)")]
    pub fn set_use_dynamic_scale(&self, value: bool) {}

    #[unity_method(name = "get_isPowerOfTwo")]
    pub fn get_is_power_of_two(&self) -> bool {}

    #[unity_method(name = "set_isPowerOfTwo")]
    pub fn set_is_power_of_two(&self, value: bool) {}

    #[unity_method(name = "get_active", static)]
    pub fn get_active() -> Option<RenderTexture> {}

    #[unity_method(name = "set_active", static)]
    pub fn set_active(value: Option<RenderTexture>) {}

    #[unity_method(name = "get_colorBuffer")]
    pub fn get_color_buffer(&self) -> RenderBuffer {}

    #[unity_method(name = "get_depthBuffer")]
    pub fn get_depth_buffer(&self) -> RenderBuffer {}

    #[unity_icall("UnityEngine.RenderTexture::get_depth")]
    pub fn get_depth(&self) -> i32 {}

    #[unity_icall("UnityEngine.RenderTexture::set_depth(System.Int32)")]
    pub fn set_depth(&self, value: i32) {}

    #[unity_method(name = "get_isCubemap")]
    pub fn get_is_cubemap(&self) -> bool {}

    #[unity_method(name = "set_isCubemap")]
    pub fn set_is_cubemap(&self, value: bool) {}

    #[unity_method(name = "get_isVolume")]
    pub fn get_is_volume(&self) -> bool {}

    #[unity_method(name = "set_isVolume")]
    pub fn set_is_volume(&self, value: bool) {}

    #[unity_method(name = "get_enabled", static)]
    pub fn get_enabled() -> bool {}

    #[unity_method(name = "set_enabled", static)]
    pub fn set_enabled(value: bool) {}

    #[unity_method(name = "get_descriptor")]
    pub fn get_descriptor(&self) -> RenderTextureDescriptor {}

    #[unity_method(name = "set_descriptor")]
    pub fn set_descriptor(&self, value: RenderTextureDescriptor) {}

    #[unity_icall("UnityEngine.RenderTexture::SetMipMapCount(System.Int32)")]
    pub fn set_mip_map_count(&self, count: i32) {}

    #[unity_icall("UnityEngine.RenderTexture::SetShadowSamplingMode(ShadowSamplingMode)")]
    pub fn set_shadow_sampling_mode(&self, sampling_mode: ShadowSamplingMode) {}

    #[unity_icall("UnityEngine.RenderTexture::GetNativeDepthBufferPtr")]
    pub fn get_native_depth_buffer_ptr(&self) -> isize {}

    #[unity_icall("UnityEngine.RenderTexture::MarkRestoreExpected")]
    pub fn mark_restore_expected(&self) {}

    #[unity_icall("UnityEngine.RenderTexture::DiscardContents(System.Boolean,System.Boolean)")]
    pub fn discard_contents(&self, discard_color: bool, discard_depth: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::ResolveAA")]
    pub fn resolve_anti_aliased_surface(&self) {}

    #[unity_icall("UnityEngine.RenderTexture::ResolveAATo(RenderTexture)")]
    pub fn resolve_anti_aliased_surface_1(&self, rt: Option<RenderTexture>) {}

    #[unity_icall("UnityEngine.RenderTexture::SetGlobalShaderProperty(System.String)")]
    pub fn set_global_shader_property(&self, property_name: &str) {}

    #[unity_icall("UnityEngine.RenderTexture::Create")]
    pub fn create(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::Release")]
    pub fn release(&self) {}

    #[unity_icall("UnityEngine.RenderTexture::IsCreated")]
    pub fn is_created(&self) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::GenerateMips")]
    pub fn generate_mips(&self) {}

    #[unity_icall("UnityEngine.RenderTexture::ConvertToEquirect(RenderTexture,Camera.MonoOrStereoscopicEye)")]
    pub fn convert_to_equirect(&self, equirect: Option<RenderTexture>, eye: camera::MonoOrStereoscopicEye) {}

    #[unity_icall("UnityEngine.RenderTexture::SetSRGBReadWrite(System.Boolean)")]
    pub fn set_srgb_read_write(&self, srgb: bool) {}

    #[unity_icall("UnityEngine.RenderTexture::Internal_Create(RenderTexture)")]
    pub fn internal_create(rt: Option<RenderTexture>) {}

    #[unity_icall("UnityEngine.RenderTexture::SupportsStencil(RenderTexture)")]
    pub fn supports_stencil(rt: Option<RenderTexture>) -> bool {}

    #[unity_icall("UnityEngine.RenderTexture::GetTemporary_Internal(RenderTextureDescriptor)")]
    pub fn get_temporary_internal(desc: RenderTextureDescriptor) -> Option<RenderTexture> {}

    #[unity_icall("UnityEngine.RenderTexture::ReleaseTemporary(RenderTexture)")]
    pub fn release_temporary(temp: Option<RenderTexture>) {}

    #[unity_method(name = "GetTexelOffset")]
    pub fn get_texel_offset(&self) -> Vector2 {}

    #[unity_icall("UnityEngine.RenderTexture::GetColorBuffer_Injected(RenderBuffer&)")]
    pub fn get_color_buffer_1(&self, ret: &mut RenderBuffer) {}

    #[unity_icall("UnityEngine.RenderTexture::GetDepthBuffer_Injected(RenderBuffer&)")]
    pub fn get_depth_buffer_1(&self, ret: &mut RenderBuffer) {}

    #[unity_icall("UnityEngine.RenderTexture::SetRenderTextureDescriptor_Injected(RenderTextureDescriptor&)")]
    pub fn set_render_texture_descriptor(&self, desc: &mut RenderTextureDescriptor) {}

    #[unity_icall("UnityEngine.RenderTexture::GetDescriptor_Injected(RenderTextureDescriptor&)")]
    pub fn get_descriptor_1(&self, ret: &mut RenderTextureDescriptor) {}

    #[unity_icall("UnityEngine.RenderTexture::GetTemporary_Internal_Injected(RenderTextureDescriptor&)")]
    pub fn get_temporary_internal_1(desc: &mut RenderTextureDescriptor) -> Option<RenderTexture> {}

}
