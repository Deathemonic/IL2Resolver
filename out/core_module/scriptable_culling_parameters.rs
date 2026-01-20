#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector3};
use crate::mscorlib::{SystemObject};
use super::camera_properties::CameraProperties;
use super::culling_options::CullingOptions;
use super::lod_parameters::LODParameters;
use super::plane::Plane;
use super::reflection_probe_sorting_criteria::ReflectionProbeSortingCriteria;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ScriptableCullingParameters", namespace = "UnityEngine.Rendering", value_type)]
pub struct ScriptableCullingParameters {
    pub m_is_orthographic: i32,
    pub m_lod_parameters: LODParameters,
    pub m_culling_planes: *mut c_void,
    pub m_culling_plane_count: i32,
    pub m_culling_mask: u32,
    pub m_scene_mask: u64,
    pub m_layer_far_cull_distances: *mut c_void,
    pub m_layer_cull: i32,
    pub m_culling_matrix: Matrix4x4,
    pub m_origin: Vector3,
    pub m_shadow_distance: f32,
    pub m_shadow_near_plane_offset: f32,
    pub m_culling_options: CullingOptions,
    pub m_reflection_probe_sorting_criteria: ReflectionProbeSortingCriteria,
    pub m_camera_properties: CameraProperties,
    pub m_accurate_occlusion_threshold: f32,
    pub m_maximum_portal_culling_jobs: i32,
    pub m_stereo_view_matrix: Matrix4x4,
    pub m_stereo_projection_matrix: Matrix4x4,
    pub m_stereo_separation_distance: f32,
    pub m_maximum_visible_lights: i32,
    pub m_conservative_enclosing_sphere: bool,
    pub m_num_iterations_enclosing_sphere: i32,
}

#[unity_impl]
impl ScriptableCullingParameters {
    #[unity_method(name = "get_maximumVisibleLights")]
    pub fn get_maximum_visible_lights(&self) -> i32 {}

    #[unity_method(name = "set_maximumVisibleLights")]
    pub fn set_maximum_visible_lights(&self, value: i32) {}

    #[unity_method(name = "get_conservativeEnclosingSphere")]
    pub fn get_conservative_enclosing_sphere(&self) -> bool {}

    #[unity_method(name = "set_conservativeEnclosingSphere")]
    pub fn set_conservative_enclosing_sphere(&self, value: bool) {}

    #[unity_method(name = "get_numIterationsEnclosingSphere")]
    pub fn get_num_iterations_enclosing_sphere(&self) -> i32 {}

    #[unity_method(name = "set_numIterationsEnclosingSphere")]
    pub fn set_num_iterations_enclosing_sphere(&self, value: i32) {}

    #[unity_method(name = "get_cullingPlaneCount")]
    pub fn get_culling_plane_count(&self) -> i32 {}

    #[unity_method(name = "set_cullingPlaneCount")]
    pub fn set_culling_plane_count(&self, value: i32) {}

    #[unity_method(name = "get_isOrthographic")]
    pub fn get_is_orthographic(&self) -> bool {}

    #[unity_method(name = "set_isOrthographic")]
    pub fn set_is_orthographic(&self, value: bool) {}

    #[unity_method(name = "get_lodParameters")]
    pub fn get_lod_parameters(&self) -> LODParameters {}

    #[unity_method(name = "set_lodParameters")]
    pub fn set_lod_parameters(&self, value: LODParameters) {}

    #[unity_method(name = "get_cullingMask")]
    pub fn get_culling_mask(&self) -> u32 {}

    #[unity_method(name = "set_cullingMask")]
    pub fn set_culling_mask(&self, value: u32) {}

    #[unity_method(name = "get_cullingMatrix")]
    pub fn get_culling_matrix(&self) -> Matrix4x4 {}

    #[unity_method(name = "set_cullingMatrix")]
    pub fn set_culling_matrix(&self, value: Matrix4x4) {}

    #[unity_method(name = "get_origin")]
    pub fn get_origin(&self) -> Vector3 {}

    #[unity_method(name = "set_origin")]
    pub fn set_origin(&self, value: Vector3) {}

    #[unity_method(name = "get_shadowDistance")]
    pub fn get_shadow_distance(&self) -> f32 {}

    #[unity_method(name = "set_shadowDistance")]
    pub fn set_shadow_distance(&self, value: f32) {}

    #[unity_method(name = "get_shadowNearPlaneOffset")]
    pub fn get_shadow_near_plane_offset(&self) -> f32 {}

    #[unity_method(name = "set_shadowNearPlaneOffset")]
    pub fn set_shadow_near_plane_offset(&self, value: f32) {}

    #[unity_method(name = "get_cullingOptions")]
    pub fn get_culling_options(&self) -> CullingOptions {}

    #[unity_method(name = "set_cullingOptions")]
    pub fn set_culling_options(&self, value: CullingOptions) {}

    #[unity_method(name = "get_reflectionProbeSortingCriteria")]
    pub fn get_reflection_probe_sorting_criteria(&self) -> ReflectionProbeSortingCriteria {}

    #[unity_method(name = "set_reflectionProbeSortingCriteria")]
    pub fn set_reflection_probe_sorting_criteria(&self, value: ReflectionProbeSortingCriteria) {}

    #[unity_method(name = "get_cameraProperties")]
    pub fn get_camera_properties(&self) -> CameraProperties {}

    #[unity_method(name = "set_cameraProperties")]
    pub fn set_camera_properties(&self, value: CameraProperties) {}

    #[unity_method(name = "get_stereoViewMatrix")]
    pub fn get_stereo_view_matrix(&self) -> Matrix4x4 {}

    #[unity_method(name = "set_stereoViewMatrix")]
    pub fn set_stereo_view_matrix(&self, value: Matrix4x4) {}

    #[unity_method(name = "get_stereoProjectionMatrix")]
    pub fn get_stereo_projection_matrix(&self) -> Matrix4x4 {}

    #[unity_method(name = "set_stereoProjectionMatrix")]
    pub fn set_stereo_projection_matrix(&self, value: Matrix4x4) {}

    #[unity_method(name = "get_stereoSeparationDistance")]
    pub fn get_stereo_separation_distance(&self) -> f32 {}

    #[unity_method(name = "set_stereoSeparationDistance")]
    pub fn set_stereo_separation_distance(&self, value: f32) {}

    #[unity_method(name = "get_accurateOcclusionThreshold")]
    pub fn get_accurate_occlusion_threshold(&self) -> f32 {}

    #[unity_method(name = "set_accurateOcclusionThreshold")]
    pub fn set_accurate_occlusion_threshold(&self, value: f32) {}

    #[unity_method(name = "get_maximumPortalCullingJobs")]
    pub fn get_maximum_portal_culling_jobs(&self) -> i32 {}

    #[unity_method(name = "set_maximumPortalCullingJobs")]
    pub fn set_maximum_portal_culling_jobs(&self, value: i32) {}

    #[unity_method(name = "get_cullingJobsLowerLimit", static)]
    pub fn get_culling_jobs_lower_limit() -> i32 {}

    #[unity_method(name = "get_cullingJobsUpperLimit", static)]
    pub fn get_culling_jobs_upper_limit() -> i32 {}

    #[unity_method(name = "GetLayerCullingDistance")]
    pub fn get_layer_culling_distance(&self, layer_index: i32) -> f32 {}

    #[unity_method(name = "SetLayerCullingDistance")]
    pub fn set_layer_culling_distance(&self, layer_index: i32, distance: f32) {}

    #[unity_method(name = "GetCullingPlane")]
    pub fn get_culling_plane(&self, index: i32) -> Plane {}

    #[unity_method(name = "SetCullingPlane")]
    pub fn set_culling_plane(&self, index: i32, plane: Plane) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: ScriptableCullingParameters) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
