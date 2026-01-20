#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::battery_status::BatteryStatus;
use super::copy_texture_support::CopyTextureSupport;
use super::default_format::DefaultFormat;
use super::device_type::DeviceType;
use super::format_usage::FormatUsage;
use super::graphics_device_type::GraphicsDeviceType;
use super::graphics_format::GraphicsFormat;
use super::hdr_display_support_flags::HDRDisplaySupportFlags;
use super::npot_support::NPOTSupport;
use super::operating_system_family::OperatingSystemFamily;
use super::rendering_threading_mode::RenderingThreadingMode;
use super::render_texture_descriptor::RenderTextureDescriptor;
use super::render_texture_format::RenderTextureFormat;
use super::texture_format::TextureFormat;
use super::vertex_attribute_format::VertexAttributeFormat;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "SystemInfo", namespace = "UnityEngine")]
pub struct SystemInfo(pub *mut c_void);

#[unity_impl]
impl SystemInfo {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_batteryLevel", static)]
    pub fn get_battery_level() -> f32 {}

    #[unity_method(name = "get_batteryStatus", static)]
    pub fn get_battery_status() -> BatteryStatus {}

    #[unity_method(name = "get_operatingSystem", static)]
    pub fn get_operating_system() -> Option<SystemString> {}

    #[unity_method(name = "get_operatingSystemFamily", static)]
    pub fn get_operating_system_family() -> OperatingSystemFamily {}

    #[unity_method(name = "get_processorType", static)]
    pub fn get_processor_type() -> Option<SystemString> {}

    #[unity_method(name = "get_processorFrequency", static)]
    pub fn get_processor_frequency() -> i32 {}

    #[unity_method(name = "get_processorCount", static)]
    pub fn get_processor_count() -> i32 {}

    #[unity_method(name = "get_systemMemorySize", static)]
    pub fn get_system_memory_size() -> i32 {}

    #[unity_method(name = "get_deviceUniqueIdentifier", static)]
    pub fn get_device_unique_identifier() -> Option<SystemString> {}

    #[unity_method(name = "get_deviceName", static)]
    pub fn get_device_name() -> Option<SystemString> {}

    #[unity_method(name = "get_deviceModel", static)]
    pub fn get_device_model() -> Option<SystemString> {}

    #[unity_method(name = "get_supportsAccelerometer", static)]
    pub fn get_supports_accelerometer() -> bool {}

    #[unity_method(name = "get_supportsGyroscope", static)]
    pub fn get_supports_gyroscope() -> bool {}

    #[unity_method(name = "get_supportsLocationService", static)]
    pub fn get_supports_location_service() -> bool {}

    #[unity_method(name = "get_supportsVibration", static)]
    pub fn get_supports_vibration() -> bool {}

    #[unity_method(name = "get_supportsAudio", static)]
    pub fn get_supports_audio() -> bool {}

    #[unity_method(name = "get_deviceType", static)]
    pub fn get_device_type() -> DeviceType {}

    #[unity_method(name = "get_graphicsMemorySize", static)]
    pub fn get_graphics_memory_size() -> i32 {}

    #[unity_method(name = "get_graphicsDeviceName", static)]
    pub fn get_graphics_device_name() -> Option<SystemString> {}

    #[unity_method(name = "get_graphicsDeviceVendor", static)]
    pub fn get_graphics_device_vendor() -> Option<SystemString> {}

    #[unity_method(name = "get_graphicsDeviceID", static)]
    pub fn get_graphics_device_id() -> i32 {}

    #[unity_method(name = "get_graphicsDeviceVendorID", static)]
    pub fn get_graphics_device_vendor_id() -> i32 {}

    #[unity_method(name = "get_graphicsDeviceType", static)]
    pub fn get_graphics_device_type() -> GraphicsDeviceType {}

    #[unity_method(name = "get_graphicsUVStartsAtTop", static)]
    pub fn get_graphics_uv_starts_at_top() -> bool {}

    #[unity_method(name = "get_graphicsDeviceVersion", static)]
    pub fn get_graphics_device_version() -> Option<SystemString> {}

    #[unity_method(name = "get_graphicsShaderLevel", static)]
    pub fn get_graphics_shader_level() -> i32 {}

    #[unity_method(name = "get_graphicsMultiThreaded", static)]
    pub fn get_graphics_multi_threaded() -> bool {}

    #[unity_method(name = "get_renderingThreadingMode", static)]
    pub fn get_rendering_threading_mode() -> RenderingThreadingMode {}

    #[unity_method(name = "get_hasHiddenSurfaceRemovalOnGPU", static)]
    pub fn get_has_hidden_surface_removal_on_gpu() -> bool {}

    #[unity_method(name = "get_hasDynamicUniformArrayIndexingInFragmentShaders", static)]
    pub fn get_has_dynamic_uniform_array_indexing_in_fragment_shaders() -> bool {}

    #[unity_method(name = "get_supportsShadows", static)]
    pub fn get_supports_shadows() -> bool {}

    #[unity_method(name = "get_supportsRawShadowDepthSampling", static)]
    pub fn get_supports_raw_shadow_depth_sampling() -> bool {}

    #[unity_method(name = "get_supportsRenderTextures", static)]
    pub fn get_supports_render_textures() -> bool {}

    #[unity_method(name = "get_supportsMotionVectors", static)]
    pub fn get_supports_motion_vectors() -> bool {}

    #[unity_method(name = "get_supportsRenderToCubemap", static)]
    pub fn get_supports_render_to_cubemap() -> bool {}

    #[unity_method(name = "get_supportsImageEffects", static)]
    pub fn get_supports_image_effects() -> bool {}

    #[unity_method(name = "get_supports3DTextures", static)]
    pub fn get_supports3d_textures() -> bool {}

    #[unity_method(name = "get_supportsCompressed3DTextures", static)]
    pub fn get_supports_compressed3d_textures() -> bool {}

    #[unity_method(name = "get_supports2DArrayTextures", static)]
    pub fn get_supports2d_array_textures() -> bool {}

    #[unity_method(name = "get_supports3DRenderTextures", static)]
    pub fn get_supports3d_render_textures() -> bool {}

    #[unity_method(name = "get_supportsCubemapArrayTextures", static)]
    pub fn get_supports_cubemap_array_textures() -> bool {}

    #[unity_method(name = "get_supportsAnisotropicFilter", static)]
    pub fn get_supports_anisotropic_filter() -> bool {}

    #[unity_method(name = "get_copyTextureSupport", static)]
    pub fn get_copy_texture_support() -> CopyTextureSupport {}

    #[unity_method(name = "get_supportsComputeShaders", static)]
    pub fn get_supports_compute_shaders() -> bool {}

    #[unity_method(name = "get_supportsGeometryShaders", static)]
    pub fn get_supports_geometry_shaders() -> bool {}

    #[unity_method(name = "get_supportsTessellationShaders", static)]
    pub fn get_supports_tessellation_shaders() -> bool {}

    #[unity_method(name = "get_supportsRenderTargetArrayIndexFromVertexShader", static)]
    pub fn get_supports_render_target_array_index_from_vertex_shader() -> bool {}

    #[unity_method(name = "get_supportsInstancing", static)]
    pub fn get_supports_instancing() -> bool {}

    #[unity_method(name = "get_supportsHardwareQuadTopology", static)]
    pub fn get_supports_hardware_quad_topology() -> bool {}

    #[unity_method(name = "get_supports32bitsIndexBuffer", static)]
    pub fn get_supports32bits_index_buffer() -> bool {}

    #[unity_method(name = "get_supportsSparseTextures", static)]
    pub fn get_supports_sparse_textures() -> bool {}

    #[unity_method(name = "get_supportedRenderTargetCount", static)]
    pub fn get_supported_render_target_count() -> i32 {}

    #[unity_method(name = "get_supportsSeparatedRenderTargetsBlend", static)]
    pub fn get_supports_separated_render_targets_blend() -> bool {}

    #[unity_method(name = "get_supportedRandomWriteTargetCount", static)]
    pub fn get_supported_random_write_target_count() -> i32 {}

    #[unity_method(name = "get_supportsMultisampledTextures", static)]
    pub fn get_supports_multisampled_textures() -> i32 {}

    #[unity_method(name = "get_supportsMultisampled2DArrayTextures", static)]
    pub fn get_supports_multisampled2d_array_textures() -> bool {}

    #[unity_method(name = "get_supportsMultisampleAutoResolve", static)]
    pub fn get_supports_multisample_auto_resolve() -> bool {}

    #[unity_method(name = "get_supportsTextureWrapMirrorOnce", static)]
    pub fn get_supports_texture_wrap_mirror_once() -> i32 {}

    #[unity_method(name = "get_usesReversedZBuffer", static)]
    pub fn get_uses_reversed_z_buffer() -> bool {}

    #[unity_method(name = "get_supportsStencil", static)]
    pub fn get_supports_stencil() -> i32 {}

    #[unity_method(name = "get_npotSupport", static)]
    pub fn get_npot_support() -> NPOTSupport {}

    #[unity_method(name = "get_maxTextureSize", static)]
    pub fn get_max_texture_size() -> i32 {}

    #[unity_method(name = "get_maxTexture3DSize", static)]
    pub fn get_max_texture3d_size() -> i32 {}

    #[unity_method(name = "get_maxTextureArraySlices", static)]
    pub fn get_max_texture_array_slices() -> i32 {}

    #[unity_method(name = "get_maxCubemapSize", static)]
    pub fn get_max_cubemap_size() -> i32 {}

    #[unity_method(name = "get_maxAnisotropyLevel", static)]
    pub fn get_max_anisotropy_level() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsVertex", static)]
    pub fn get_max_compute_buffer_inputs_vertex() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsFragment", static)]
    pub fn get_max_compute_buffer_inputs_fragment() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsGeometry", static)]
    pub fn get_max_compute_buffer_inputs_geometry() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsDomain", static)]
    pub fn get_max_compute_buffer_inputs_domain() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsHull", static)]
    pub fn get_max_compute_buffer_inputs_hull() -> i32 {}

    #[unity_method(name = "get_maxComputeBufferInputsCompute", static)]
    pub fn get_max_compute_buffer_inputs_compute() -> i32 {}

    #[unity_method(name = "get_maxComputeWorkGroupSize", static)]
    pub fn get_max_compute_work_group_size() -> i32 {}

    #[unity_method(name = "get_maxComputeWorkGroupSizeX", static)]
    pub fn get_max_compute_work_group_size_x() -> i32 {}

    #[unity_method(name = "get_maxComputeWorkGroupSizeY", static)]
    pub fn get_max_compute_work_group_size_y() -> i32 {}

    #[unity_method(name = "get_maxComputeWorkGroupSizeZ", static)]
    pub fn get_max_compute_work_group_size_z() -> i32 {}

    #[unity_method(name = "get_computeSubGroupSize", static)]
    pub fn get_compute_sub_group_size() -> i32 {}

    #[unity_method(name = "get_supportsAsyncCompute", static)]
    pub fn get_supports_async_compute() -> bool {}

    #[unity_method(name = "get_supportsGpuRecorder", static)]
    pub fn get_supports_gpu_recorder() -> bool {}

    #[unity_method(name = "get_supportsGraphicsFence", static)]
    pub fn get_supports_graphics_fence() -> bool {}

    #[unity_method(name = "get_supportsAsyncGPUReadback", static)]
    pub fn get_supports_async_gpu_readback() -> bool {}

    #[unity_method(name = "get_supportsRayTracing", static)]
    pub fn get_supports_ray_tracing() -> bool {}

    #[unity_method(name = "get_supportsSetConstantBuffer", static)]
    pub fn get_supports_set_constant_buffer() -> bool {}

    #[unity_method(name = "get_constantBufferOffsetAlignment", static)]
    pub fn get_constant_buffer_offset_alignment() -> i32 {}

    #[unity_method(name = "get_maxGraphicsBufferSize", static)]
    pub fn get_max_graphics_buffer_size() -> i64 {}

    #[unity_method(name = "get_minConstantBufferOffsetAlignment", static)]
    pub fn get_min_constant_buffer_offset_alignment() -> bool {}

    #[unity_method(name = "get_hasMipMaxLevel", static)]
    pub fn get_has_mip_max_level() -> bool {}

    #[unity_method(name = "get_supportsMipStreaming", static)]
    pub fn get_supports_mip_streaming() -> bool {}

    #[unity_method(name = "get_graphicsPixelFillrate", static)]
    pub fn get_graphics_pixel_fillrate() -> i32 {}

    #[unity_method(name = "get_usesLoadStoreActions", static)]
    pub fn get_uses_load_store_actions() -> bool {}

    #[unity_method(name = "get_hdrDisplaySupportFlags", static)]
    pub fn get_hdr_display_support_flags() -> HDRDisplaySupportFlags {}

    #[unity_method(name = "get_supportsConservativeRaster", static)]
    pub fn get_supports_conservative_raster() -> bool {}

    #[unity_method(name = "get_supportsMultiview", static)]
    pub fn get_supports_multiview() -> bool {}

    #[unity_method(name = "get_supportsStoreAndResolveAction", static)]
    pub fn get_supports_store_and_resolve_action() -> bool {}

    #[unity_method(name = "get_supportsMultisampleResolveDepth", static)]
    pub fn get_supports_multisample_resolve_depth() -> bool {}

    #[unity_method(name = "get_supportsVertexPrograms", static)]
    pub fn get_supports_vertex_programs() -> bool {}

    #[unity_method(name = "get_supportsGPUFence", static)]
    pub fn get_supports_gpu_fence() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::GetProcessorFrequencyMHz")]
    pub fn get_processor_frequency_m_hz() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::GetPhysicalMemoryMB")]
    pub fn get_physical_memory_mb() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsAccelerometer")]
    pub fn supports_accelerometer() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::IsGyroAvailable")]
    pub fn is_gyro_available() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsLocationService")]
    pub fn supports_location_service() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsVibration")]
    pub fn supports_vibration() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsAudio")]
    pub fn supports_audio() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::HasHiddenSurfaceRemovalOnGPU")]
    pub fn has_hidden_surface_removal_on_gpu() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::HasDynamicUniformArrayIndexingInFragmentShaders")]
    pub fn has_dynamic_uniform_array_indexing_in_fragment_shaders() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsShadows")]
    pub fn supports_shadows() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsRawShadowDepthSampling")]
    pub fn supports_raw_shadow_depth_sampling() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMotionVectors")]
    pub fn supports_motion_vectors() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::Supports3DTextures")]
    pub fn supports3d_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsCompressed3DTextures")]
    pub fn supports_compressed3d_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::Supports2DArrayTextures")]
    pub fn supports2d_array_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::Supports3DRenderTextures")]
    pub fn supports3d_render_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsCubemapArrayTextures")]
    pub fn supports_cubemap_array_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsAnisotropicFilter")]
    pub fn supports_anisotropic_filter() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsComputeShaders")]
    pub fn supports_compute_shaders() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsGeometryShaders")]
    pub fn supports_geometry_shaders() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsTessellationShaders")]
    pub fn supports_tessellation_shaders() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsRenderTargetArrayIndexFromVertexShader")]
    pub fn supports_render_target_array_index_from_vertex_shader() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsInstancing")]
    pub fn supports_instancing() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsHardwareQuadTopology")]
    pub fn supports_hardware_quad_topology() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::Supports32bitsIndexBuffer")]
    pub fn supports32bits_index_buffer() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsSparseTextures")]
    pub fn supports_sparse_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportedRenderTargetCount")]
    pub fn supported_render_target_count() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsSeparatedRenderTargetsBlend")]
    pub fn supports_separated_render_targets_blend() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportedRandomWriteTargetCount")]
    pub fn supported_random_write_target_count() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsVertex")]
    pub fn max_compute_buffer_inputs_vertex() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsFragment")]
    pub fn max_compute_buffer_inputs_fragment() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsGeometry")]
    pub fn max_compute_buffer_inputs_geometry() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsDomain")]
    pub fn max_compute_buffer_inputs_domain() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsHull")]
    pub fn max_compute_buffer_inputs_hull() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxComputeBufferInputsCompute")]
    pub fn max_compute_buffer_inputs_compute() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMultisampledTextures")]
    pub fn supports_multisampled_textures() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMultisampled2DArrayTextures")]
    pub fn supports_multisampled2d_array_textures() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMultisampleAutoResolve")]
    pub fn supports_multisample_auto_resolve() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsTextureWrapMirrorOnce")]
    pub fn supports_texture_wrap_mirror_once() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::UsesReversedZBuffer")]
    pub fn uses_reversed_z_buffer() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::HasRenderTextureNative(RenderTextureFormat)")]
    pub fn has_render_texture_native(format: RenderTextureFormat) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsBlendingOnRenderTextureFormatNative(RenderTextureFormat)")]
    pub fn supports_blending_on_render_texture_format_native(format: RenderTextureFormat) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsRandomWriteOnRenderTextureFormatNative(RenderTextureFormat)")]
    pub fn supports_random_write_on_render_texture_format_native(format: RenderTextureFormat) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsTextureFormatNative(TextureFormat)")]
    pub fn supports_texture_format_native(format: TextureFormat) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsVertexAttributeFormatNative(VertexAttributeFormat,System.Int32)")]
    pub fn supports_vertex_attribute_format_native(format: VertexAttributeFormat, dimension: i32) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::GetMaxRenderTextureSize")]
    pub fn get_max_render_texture_size() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsAsyncCompute")]
    pub fn supports_async_compute() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsGpuRecorder")]
    pub fn supports_gpu_recorder() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsGPUFence")]
    pub fn supports_gpu_fence() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsAsyncGPUReadback")]
    pub fn supports_async_gpu_readback() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsRayTracing")]
    pub fn supports_ray_tracing() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsSetConstantBuffer")]
    pub fn supports_set_constant_buffer() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::MinConstantBufferOffsetAlignment")]
    pub fn min_constant_buffer_offset_alignment() -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::MaxGraphicsBufferSize")]
    pub fn max_graphics_buffer_size() -> i64 {}

    #[unity_icall("UnityEngine.SystemInfo::HasMipMaxLevel")]
    pub fn has_mip_max_level() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMipStreaming")]
    pub fn supports_mip_streaming() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::IsFormatSupported(GraphicsFormat,FormatUsage)")]
    pub fn is_format_supported(format: GraphicsFormat, usage: FormatUsage) -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::GetCompatibleFormat(GraphicsFormat,FormatUsage)")]
    pub fn get_compatible_format(format: GraphicsFormat, usage: FormatUsage) -> GraphicsFormat {}

    #[unity_icall("UnityEngine.SystemInfo::GetGraphicsFormat(DefaultFormat)")]
    pub fn get_graphics_format(format: DefaultFormat) -> GraphicsFormat {}

    #[unity_icall("UnityEngine.SystemInfo::GetRenderTextureSupportedMSAASampleCount(RenderTextureDescriptor)")]
    pub fn get_render_texture_supported_msaa_sample_count(desc: RenderTextureDescriptor) -> i32 {}

    #[unity_icall("UnityEngine.SystemInfo::UsesLoadStoreActions")]
    pub fn uses_load_store_actions() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsConservativeRaster")]
    pub fn supports_conservative_raster() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMultiview")]
    pub fn supports_multiview() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsStoreAndResolveAction")]
    pub fn supports_store_and_resolve_action() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::SupportsMultisampleResolveDepth")]
    pub fn supports_multisample_resolve_depth() -> bool {}

    #[unity_icall("UnityEngine.SystemInfo::GetRenderTextureSupportedMSAASampleCount_Injected(RenderTextureDescriptor&)")]
    pub fn get_render_texture_supported_msaa_sample_count_1(desc: &mut RenderTextureDescriptor) -> i32 {}

}
