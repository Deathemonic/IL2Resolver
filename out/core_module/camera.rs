#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector3, Vector4};
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::camera_clear_flags::CameraClearFlags;
use super::camera_event::CameraEvent;
use super::camera_type::CameraType;
use super::color::Color;
use super::command_buffer::CommandBuffer;
use super::compute_queue_type::ComputeQueueType;
use super::depth_texture_mode::DepthTextureMode;
use super::opaque_sort_mode::OpaqueSortMode;
use super::ray::Ray;
use super::rect::Rect;
use super::render_buffer::RenderBuffer;
use super::rendering_path::RenderingPath;
use super::render_texture::RenderTexture;
use super::scene::Scene;
use super::scriptable_culling_parameters::ScriptableCullingParameters;
use super::shader::Shader;
use super::stereo_target_eye_mask::StereoTargetEyeMask;
use super::texture::Texture;
use super::transparency_sort_mode::TransparencySortMode;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Camera", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct Camera(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectionMatrixMode {
    #[default]
    Explicit = 0,
    Implicit = 1,
    PhysicalPropertiesBased = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GateFitMode {
    Vertical = 1,
    Horizontal = 2,
    Fill = 3,
    Overscan = 4,
    #[default]
    None = 0,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldOfViewAxis {
    #[default]
    Vertical = 0,
    Horizontal = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StereoscopicEye {
    #[default]
    Left = 0,
    Right = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MonoOrStereoscopicEye {
    #[default]
    Left = 0,
    Right = 1,
    Mono = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SceneViewFilterMode {
    #[default]
    Off = 0,
    ShowFiltered = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderRequestMode {
    #[default]
    None = 0,
    ObjectId = 1,
    Depth = 2,
    VertexNormal = 3,
    WorldPosition = 4,
    EntityId = 5,
    BaseColor = 6,
    SpecularColor = 7,
    Metallic = 8,
    Emission = 9,
    Normal = 10,
    Smoothness = 11,
    Occlusion = 12,
    DiffuseColor = 13,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderRequestOutputSpace {
    ScreenSpace = -1,
    #[default]
    UV0 = 0,
    UV1 = 1,
    UV2 = 2,
    UV3 = 3,
    UV4 = 4,
    UV5 = 5,
    UV6 = 6,
    UV7 = 7,
    UV8 = 8,
}

#[unity_impl]
impl Camera {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Camera::get_nearClipPlane")]
    pub fn get_near_clip_plane(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_nearClipPlane(System.Single)")]
    pub fn set_near_clip_plane(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_farClipPlane")]
    pub fn get_far_clip_plane(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_farClipPlane(System.Single)")]
    pub fn set_far_clip_plane(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_fieldOfView")]
    pub fn get_field_of_view(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_fieldOfView(System.Single)")]
    pub fn set_field_of_view(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_renderingPath")]
    pub fn get_rendering_path(&self) -> RenderingPath {}

    #[unity_icall("UnityEngine.Camera::set_renderingPath(RenderingPath)")]
    pub fn set_rendering_path(&self, value: RenderingPath) {}

    #[unity_icall("UnityEngine.Camera::get_actualRenderingPath")]
    pub fn get_actual_rendering_path(&self) -> RenderingPath {}

    #[unity_icall("UnityEngine.Camera::get_allowHDR")]
    pub fn get_allow_hdr(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_allowHDR(System.Boolean)")]
    pub fn set_allow_hdr(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_allowMSAA")]
    pub fn get_allow_msaa(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_allowMSAA(System.Boolean)")]
    pub fn set_allow_msaa(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_allowDynamicResolution")]
    pub fn get_allow_dynamic_resolution(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_allowDynamicResolution(System.Boolean)")]
    pub fn set_allow_dynamic_resolution(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_forceIntoRenderTexture")]
    pub fn get_force_into_render_texture(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_forceIntoRenderTexture(System.Boolean)")]
    pub fn set_force_into_render_texture(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_orthographicSize")]
    pub fn get_orthographic_size(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_orthographicSize(System.Single)")]
    pub fn set_orthographic_size(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_orthographic")]
    pub fn get_orthographic(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_orthographic(System.Boolean)")]
    pub fn set_orthographic(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_opaqueSortMode")]
    pub fn get_opaque_sort_mode(&self) -> OpaqueSortMode {}

    #[unity_icall("UnityEngine.Camera::set_opaqueSortMode(OpaqueSortMode)")]
    pub fn set_opaque_sort_mode(&self, value: OpaqueSortMode) {}

    #[unity_icall("UnityEngine.Camera::get_transparencySortMode")]
    pub fn get_transparency_sort_mode(&self) -> TransparencySortMode {}

    #[unity_icall("UnityEngine.Camera::set_transparencySortMode(TransparencySortMode)")]
    pub fn set_transparency_sort_mode(&self, value: TransparencySortMode) {}

    #[unity_icall("UnityEngine.Camera::get_transparencySortAxis_Injected(Vector3&)")]
    pub fn get_transparency_sort_axis(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::set_transparencySortAxis_Injected(Vector3&)")]
    pub fn set_transparency_sort_axis(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::get_depth")]
    pub fn get_depth(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_depth(System.Single)")]
    pub fn set_depth(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_aspect")]
    pub fn get_aspect(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_aspect(System.Single)")]
    pub fn set_aspect(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_velocity_Injected(Vector3&)")]
    pub fn get_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::get_cullingMask")]
    pub fn get_culling_mask(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::set_cullingMask(System.Int32)")]
    pub fn set_culling_mask(&self, value: i32) {}

    #[unity_icall("UnityEngine.Camera::get_eventMask")]
    pub fn get_event_mask(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::set_eventMask(System.Int32)")]
    pub fn set_event_mask(&self, value: i32) {}

    #[unity_icall("UnityEngine.Camera::get_layerCullSpherical")]
    pub fn get_layer_cull_spherical(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_layerCullSpherical(System.Boolean)")]
    pub fn set_layer_cull_spherical(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_cameraType")]
    pub fn get_camera_type(&self) -> CameraType {}

    #[unity_icall("UnityEngine.Camera::set_cameraType(CameraType)")]
    pub fn set_camera_type(&self, value: CameraType) {}

    #[unity_icall("UnityEngine.Camera::get_overrideSceneCullingMask")]
    pub fn get_override_scene_culling_mask(&self) -> u64 {}

    #[unity_icall("UnityEngine.Camera::set_overrideSceneCullingMask(System.UInt64)")]
    pub fn set_override_scene_culling_mask(&self, value: u64) {}

    #[unity_method(name = "get_layerCullDistances")]
    pub fn get_layer_cull_distances(&self) -> Array<f32> {}

    #[unity_method(name = "set_layerCullDistances")]
    pub fn set_layer_cull_distances(&self, value: Array<f32>) {}

    #[unity_icall("UnityEngine.Camera::get_useOcclusionCulling")]
    pub fn get_use_occlusion_culling(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_useOcclusionCulling(System.Boolean)")]
    pub fn set_use_occlusion_culling(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_cullingMatrix_Injected(Matrix4x4&)")]
    pub fn get_culling_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::set_cullingMatrix_Injected(Matrix4x4&)")]
    pub fn set_culling_matrix(&self, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_backgroundColor_Injected(Color&)")]
    pub fn get_background_color(&self, ret: &mut Color) {}

    #[unity_icall("UnityEngine.Camera::set_backgroundColor_Injected(Color&)")]
    pub fn set_background_color(&self, value: &mut Color) {}

    #[unity_icall("UnityEngine.Camera::get_clearFlags")]
    pub fn get_clear_flags(&self) -> CameraClearFlags {}

    #[unity_icall("UnityEngine.Camera::set_clearFlags(CameraClearFlags)")]
    pub fn set_clear_flags(&self, value: CameraClearFlags) {}

    #[unity_icall("UnityEngine.Camera::get_depthTextureMode")]
    pub fn get_depth_texture_mode(&self) -> DepthTextureMode {}

    #[unity_icall("UnityEngine.Camera::set_depthTextureMode(DepthTextureMode)")]
    pub fn set_depth_texture_mode(&self, value: DepthTextureMode) {}

    #[unity_icall("UnityEngine.Camera::get_clearStencilAfterLightingPass")]
    pub fn get_clear_stencil_after_lighting_pass(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_clearStencilAfterLightingPass(System.Boolean)")]
    pub fn set_clear_stencil_after_lighting_pass(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_usePhysicalProperties")]
    pub fn get_use_physical_properties(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_usePhysicalProperties(System.Boolean)")]
    pub fn set_use_physical_properties(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_sensorSize_Injected(Vector2&)")]
    pub fn get_sensor_size(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::set_sensorSize_Injected(Vector2&)")]
    pub fn set_sensor_size(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::get_lensShift_Injected(Vector2&)")]
    pub fn get_lens_shift(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::set_lensShift_Injected(Vector2&)")]
    pub fn set_lens_shift(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::get_focalLength")]
    pub fn get_focal_length(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_focalLength(System.Single)")]
    pub fn set_focal_length(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_gateFit")]
    pub fn get_gate_fit(&self) -> GateFitMode {}

    #[unity_icall("UnityEngine.Camera::set_gateFit(Camera.GateFitMode)")]
    pub fn set_gate_fit(&self, value: GateFitMode) {}

    #[unity_icall("UnityEngine.Camera::get_rect_Injected(Rect&)")]
    pub fn get_rect(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Camera::set_rect_Injected(Rect&)")]
    pub fn set_rect(&self, value: &mut Rect) {}

    #[unity_icall("UnityEngine.Camera::get_pixelRect_Injected(Rect&)")]
    pub fn get_pixel_rect(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Camera::set_pixelRect_Injected(Rect&)")]
    pub fn set_pixel_rect(&self, value: &mut Rect) {}

    #[unity_icall("UnityEngine.Camera::get_pixelWidth")]
    pub fn get_pixel_width(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::get_pixelHeight")]
    pub fn get_pixel_height(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::get_scaledPixelWidth")]
    pub fn get_scaled_pixel_width(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::get_scaledPixelHeight")]
    pub fn get_scaled_pixel_height(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::get_targetTexture")]
    pub fn get_target_texture(&self) -> Option<RenderTexture> {}

    #[unity_icall("UnityEngine.Camera::set_targetTexture(RenderTexture)")]
    pub fn set_target_texture(&self, value: Option<RenderTexture>) {}

    #[unity_icall("UnityEngine.Camera::get_activeTexture")]
    pub fn get_active_texture(&self) -> Option<RenderTexture> {}

    #[unity_icall("UnityEngine.Camera::get_targetDisplay")]
    pub fn get_target_display(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::set_targetDisplay(System.Int32)")]
    pub fn set_target_display(&self, value: i32) {}

    #[unity_icall("UnityEngine.Camera::get_cameraToWorldMatrix_Injected(Matrix4x4&)")]
    pub fn get_camera_to_world_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_worldToCameraMatrix_Injected(Matrix4x4&)")]
    pub fn get_world_to_camera_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::set_worldToCameraMatrix_Injected(Matrix4x4&)")]
    pub fn set_world_to_camera_matrix(&self, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_projectionMatrix_Injected(Matrix4x4&)")]
    pub fn get_projection_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::set_projectionMatrix_Injected(Matrix4x4&)")]
    pub fn set_projection_matrix(&self, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_nonJitteredProjectionMatrix_Injected(Matrix4x4&)")]
    pub fn get_non_jittered_projection_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::set_nonJitteredProjectionMatrix_Injected(Matrix4x4&)")]
    pub fn set_non_jittered_projection_matrix(&self, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_useJitteredProjectionMatrixForTransparentRendering")]
    pub fn get_use_jittered_projection_matrix_for_transparent_rendering(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::set_useJitteredProjectionMatrixForTransparentRendering(System.Boolean)")]
    pub fn set_use_jittered_projection_matrix_for_transparent_rendering(&self, value: bool) {}

    #[unity_icall("UnityEngine.Camera::get_previousViewProjectionMatrix_Injected(Matrix4x4&)")]
    pub fn get_previous_view_projection_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::get_main")]
    pub fn get_main() -> Option<Camera> {}

    #[unity_icall("UnityEngine.Camera::get_current")]
    pub fn get_current() -> Option<Camera> {}

    #[unity_icall("UnityEngine.Camera::get_scene")]
    pub fn get_scene(&self) -> Scene {}

    #[unity_icall("UnityEngine.Camera::set_scene(Scene)")]
    pub fn set_scene(&self, value: Scene) {}

    #[unity_icall("UnityEngine.Camera::get_stereoEnabled")]
    pub fn get_stereo_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::get_stereoSeparation")]
    pub fn get_stereo_separation(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_stereoSeparation(System.Single)")]
    pub fn set_stereo_separation(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_stereoConvergence")]
    pub fn get_stereo_convergence(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::set_stereoConvergence(System.Single)")]
    pub fn set_stereo_convergence(&self, value: f32) {}

    #[unity_icall("UnityEngine.Camera::get_areVRStereoViewMatricesWithinSingleCullTolerance")]
    pub fn get_are_vr_stereo_view_matrices_within_single_cull_tolerance(&self) -> bool {}

    #[unity_icall("UnityEngine.Camera::get_stereoTargetEye")]
    pub fn get_stereo_target_eye(&self) -> StereoTargetEyeMask {}

    #[unity_icall("UnityEngine.Camera::set_stereoTargetEye(StereoTargetEyeMask)")]
    pub fn set_stereo_target_eye(&self, value: StereoTargetEyeMask) {}

    #[unity_icall("UnityEngine.Camera::get_stereoActiveEye")]
    pub fn get_stereo_active_eye(&self) -> MonoOrStereoscopicEye {}

    #[unity_method(name = "get_allCamerasCount", static)]
    pub fn get_all_cameras_count() -> i32 {}

    #[unity_method(name = "get_allCameras", static)]
    pub fn get_all_cameras() -> Array<Camera> {}

    #[unity_method(name = "get_sceneViewFilterMode")]
    pub fn get_scene_view_filter_mode(&self) -> SceneViewFilterMode {}

    #[unity_icall("UnityEngine.Camera::get_commandBufferCount")]
    pub fn get_command_buffer_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::Reset")]
    pub fn reset(&self) {}

    #[unity_icall("UnityEngine.Camera::ResetTransparencySortSettings")]
    pub fn reset_transparency_sort_settings(&self) {}

    #[unity_icall("UnityEngine.Camera::ResetAspect")]
    pub fn reset_aspect(&self) {}

    #[unity_icall("UnityEngine.Camera::ResetCullingMatrix")]
    pub fn reset_culling_matrix(&self) {}

    #[unity_icall("UnityEngine.Camera::SetReplacementShader(Shader,System.String)")]
    pub fn set_replacement_shader(&self, shader: Option<Shader>, replacement_tag: &str) {}

    #[unity_icall("UnityEngine.Camera::ResetReplacementShader")]
    pub fn reset_replacement_shader(&self) {}

    #[unity_icall("UnityEngine.Camera::GetGateFittedFieldOfView")]
    pub fn get_gate_fitted_field_of_view(&self) -> f32 {}

    #[unity_icall("UnityEngine.Camera::SetTargetBuffersImpl(RenderBuffer,RenderBuffer)")]
    pub fn set_target_buffers(&self, color: RenderBuffer, depth: RenderBuffer) {}

    #[unity_icall("UnityEngine.Camera::SetTargetBuffersMRTImpl(RenderBuffer[],RenderBuffer)")]
    pub fn set_target_buffers_1(&self, color: Array<RenderBuffer>, depth: RenderBuffer) {}

    #[unity_icall("UnityEngine.Camera::GetCameraBufferWarnings")]
    pub fn get_camera_buffer_warnings(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Camera::ResetWorldToCameraMatrix")]
    pub fn reset_world_to_camera_matrix(&self) {}

    #[unity_icall("UnityEngine.Camera::ResetProjectionMatrix")]
    pub fn reset_projection_matrix(&self) {}

    #[unity_icall("UnityEngine.Camera::CalculateObliqueMatrix(Vector4)")]
    pub fn calculate_oblique_matrix(&self, clip_plane: Vector4) -> Matrix4x4 {}

    #[unity_icall("UnityEngine.Camera::CalculateFrustumCornersInternal(Rect,System.Single,Camera.MonoOrStereoscopicEye,Vector3[])")]
    pub fn calculate_frustum_corners_internal(&self, viewport: Rect, z: f32, eye: MonoOrStereoscopicEye, out_corners: &mut Array<Vector3>) {}

    #[unity_icall("UnityEngine.Camera::CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected(Matrix4x4&,System.Single,Vector2&,Vector2&,System.Single,System.Single,System.Single,Camera.GateFitMode)")]
    pub fn calculate_projection_matrix_from_physical_properties(output: &mut Matrix4x4, focal_length: f32, sensor_size: &mut Vector2, lens_shift: &mut Vector2, near_clip: f32, far_clip: f32, gate_aspect: f32, gate_fit_mode: GateFitMode) {}

    #[unity_icall("UnityEngine.Camera::FocalLengthToFieldOfView(System.Single,System.Single)")]
    pub fn focal_length_to_field_of_view(focal_length: f32, sensor_size: f32) -> f32 {}

    #[unity_icall("UnityEngine.Camera::FieldOfViewToFocalLength(System.Single,System.Single)")]
    pub fn field_of_view_to_focal_length(field_of_view: f32, sensor_size: f32) -> f32 {}

    #[unity_icall("UnityEngine.Camera::HorizontalToVerticalFieldOfView(System.Single,System.Single)")]
    pub fn horizontal_to_vertical_field_of_view(horizontal_field_of_view: f32, aspect_ratio: f32) -> f32 {}

    #[unity_icall("UnityEngine.Camera::VerticalToHorizontalFieldOfView(System.Single,System.Single)")]
    pub fn vertical_to_horizontal_field_of_view(vertical_field_of_view: f32, aspect_ratio: f32) -> f32 {}

    #[unity_icall("UnityEngine.Camera::GetStereoViewMatrix(Camera.StereoscopicEye)")]
    pub fn get_stereo_view_matrix(&self, eye: StereoscopicEye) -> Matrix4x4 {}

    #[unity_icall("UnityEngine.Camera::CopyStereoDeviceProjectionMatrixToNonJittered(Camera.StereoscopicEye)")]
    pub fn copy_stereo_device_projection_matrix_to_non_jittered(&self, eye: StereoscopicEye) {}

    #[unity_icall("UnityEngine.Camera::GetStereoProjectionMatrix(Camera.StereoscopicEye)")]
    pub fn get_stereo_projection_matrix(&self, eye: StereoscopicEye) -> Matrix4x4 {}

    #[unity_icall("UnityEngine.Camera::SetStereoProjectionMatrix_Injected(Camera.StereoscopicEye,Matrix4x4&)")]
    pub fn set_stereo_projection_matrix(&self, eye: StereoscopicEye, matrix: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::ResetStereoProjectionMatrices")]
    pub fn reset_stereo_projection_matrices(&self) {}

    #[unity_icall("UnityEngine.Camera::SetStereoViewMatrix_Injected(Camera.StereoscopicEye,Matrix4x4&)")]
    pub fn set_stereo_view_matrix(&self, eye: StereoscopicEye, matrix: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::ResetStereoViewMatrices")]
    pub fn reset_stereo_view_matrices(&self) {}

    #[unity_icall("UnityEngine.Camera::GetAllCamerasImpl(Camera[])")]
    pub fn get_all_cameras_impl(cam: &mut Array<Camera>) -> i32 {}

    #[unity_icall("UnityEngine.Camera::RenderToCubemapImpl(Texture,System.Int32)")]
    pub fn render_to_cubemap_impl(&self, tex: Option<Texture>, face_mask: i32) -> bool {}

    #[unity_icall("UnityEngine.Camera::GetFilterMode")]
    pub fn get_filter_mode(&self) -> i32 {}

    #[unity_icall("UnityEngine.Camera::RenderToCubemapEyeImpl(RenderTexture,System.Int32,Camera.MonoOrStereoscopicEye)")]
    pub fn render_to_cubemap_eye_impl(&self, cubemap: Option<RenderTexture>, face_mask: i32, stereo_eye: MonoOrStereoscopicEye) -> bool {}

    #[unity_icall("UnityEngine.Camera::Render")]
    pub fn render(&self) {}

    #[unity_icall("UnityEngine.Camera::RenderWithShader(Shader,System.String)")]
    pub fn render_with_shader(&self, shader: Option<Shader>, replacement_tag: &str) {}

    #[unity_icall("UnityEngine.Camera::RenderDontRestore")]
    pub fn render_dont_restore(&self) {}

    #[unity_icall("UnityEngine.Camera::SubmitRenderRequestsInternal(System.Object)")]
    pub fn submit_render_requests_internal(&self, requests: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Camera::SetupCurrent(Camera)")]
    pub fn setup_current(cur: Option<Camera>) {}

    #[unity_icall("UnityEngine.Camera::CopyFrom(Camera)")]
    pub fn copy_from(&self, other: Option<Camera>) {}

    #[unity_icall("UnityEngine.Camera::RemoveCommandBuffers(CameraEvent)")]
    pub fn remove_command_buffers(&self, evt: CameraEvent) {}

    #[unity_icall("UnityEngine.Camera::RemoveAllCommandBuffers")]
    pub fn remove_all_command_buffers(&self) {}

    #[unity_icall("UnityEngine.Camera::AddCommandBufferImpl(CameraEvent,CommandBuffer)")]
    pub fn add_command_buffer_impl(&self, evt: CameraEvent, buffer: Option<CommandBuffer>) {}

    #[unity_icall("UnityEngine.Camera::AddCommandBufferAsyncImpl(CameraEvent,CommandBuffer,ComputeQueueType)")]
    pub fn add_command_buffer_async_impl(&self, evt: CameraEvent, buffer: Option<CommandBuffer>, queue_type: ComputeQueueType) {}

    #[unity_icall("UnityEngine.Camera::RemoveCommandBufferImpl(CameraEvent,CommandBuffer)")]
    pub fn remove_command_buffer_impl(&self, evt: CameraEvent, buffer: Option<CommandBuffer>) {}

    #[unity_icall("UnityEngine.Camera::GetCommandBuffers(CameraEvent)")]
    pub fn get_command_buffers(&self, evt: CameraEvent) -> Array<CommandBuffer> {}

    #[unity_icall("UnityEngine.Camera::GetCullingParameters_Internal(Camera,System.Boolean,ScriptableCullingParameters&,System.Int32)")]
    pub fn get_culling_parameters_internal(camera: Option<Camera>, stereo_aware: bool, culling_parameters: &mut ScriptableCullingParameters, managed_culling_parameters_size: i32) -> bool {}

    #[unity_icall("UnityEngine.Camera::GetGateFittedLensShift_Injected(Vector2&)")]
    pub fn get_gate_fitted_lens_shift(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::GetLocalSpaceAim_Injected(Vector3&)")]
    pub fn get_local_space_aim(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::SetTargetBuffersImpl_Injected(RenderBuffer&,RenderBuffer&)")]
    pub fn set_target_buffers_impl(&self, color: &mut RenderBuffer, depth: &mut RenderBuffer) {}

    #[unity_icall("UnityEngine.Camera::SetTargetBuffersMRTImpl_Injected(RenderBuffer[],RenderBuffer&)")]
    pub fn set_target_buffers_mrt_impl(&self, color: Array<RenderBuffer>, depth: &mut RenderBuffer) {}

    #[unity_icall("UnityEngine.Camera::CalculateObliqueMatrix_Injected(Vector4&,Matrix4x4&)")]
    pub fn calculate_oblique_matrix_1(&self, clip_plane: &mut Vector4, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::WorldToScreenPoint_Injected(Vector3&,Camera.MonoOrStereoscopicEye,Vector3&)")]
    pub fn world_to_screen_point(&self, position: &mut Vector3, eye: MonoOrStereoscopicEye, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::WorldToViewportPoint_Injected(Vector3&,Camera.MonoOrStereoscopicEye,Vector3&)")]
    pub fn world_to_viewport_point(&self, position: &mut Vector3, eye: MonoOrStereoscopicEye, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::ViewportToWorldPoint_Injected(Vector3&,Camera.MonoOrStereoscopicEye,Vector3&)")]
    pub fn viewport_to_world_point(&self, position: &mut Vector3, eye: MonoOrStereoscopicEye, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::ScreenToWorldPoint_Injected(Vector3&,Camera.MonoOrStereoscopicEye,Vector3&)")]
    pub fn screen_to_world_point(&self, position: &mut Vector3, eye: MonoOrStereoscopicEye, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::ScreenToViewportPoint_Injected(Vector3&,Vector3&)")]
    pub fn screen_to_viewport_point(&self, position: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::ViewportToScreenPoint_Injected(Vector3&,Vector3&)")]
    pub fn viewport_to_screen_point(&self, position: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Camera::GetFrustumPlaneSizeAt_Injected(System.Single,Vector2&)")]
    pub fn get_frustum_plane_size_at(&self, distance: f32, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Camera::ViewportPointToRay_Injected(Vector2&,Camera.MonoOrStereoscopicEye,Ray&)")]
    pub fn viewport_point_to_ray(&self, pos: &mut Vector2, eye: MonoOrStereoscopicEye, ret: &mut Ray) {}

    #[unity_icall("UnityEngine.Camera::ScreenPointToRay_Injected(Vector2&,Camera.MonoOrStereoscopicEye,Ray&)")]
    pub fn screen_point_to_ray(&self, pos: &mut Vector2, eye: MonoOrStereoscopicEye, ret: &mut Ray) {}

    #[unity_icall("UnityEngine.Camera::CalculateFrustumCornersInternal_Injected(Rect&,System.Single,Camera.MonoOrStereoscopicEye,Vector3[])")]
    pub fn calculate_frustum_corners_internal_1(&self, viewport: &mut Rect, z: f32, eye: MonoOrStereoscopicEye, out_corners: &mut Array<Vector3>) {}

    #[unity_icall("UnityEngine.Camera::get_scene_Injected(Scene&)")]
    pub fn get_scene_1(&self, ret: &mut Scene) {}

    #[unity_icall("UnityEngine.Camera::GetStereoNonJitteredProjectionMatrix_Injected(Camera.StereoscopicEye,Matrix4x4&)")]
    pub fn get_stereo_non_jittered_projection_matrix(&self, eye: StereoscopicEye, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::GetStereoViewMatrix_Injected(Camera.StereoscopicEye,Matrix4x4&)")]
    pub fn get_stereo_view_matrix_1(&self, eye: StereoscopicEye, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Camera::GetStereoProjectionMatrix_Injected(Camera.StereoscopicEye,Matrix4x4&)")]
    pub fn get_stereo_projection_matrix_1(&self, eye: StereoscopicEye, ret: &mut Matrix4x4) {}

}
