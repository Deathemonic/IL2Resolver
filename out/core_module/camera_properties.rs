#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector3};
use crate::mscorlib::{SystemObject};
use super::plane::Plane;
use super::rect::Rect;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CameraProperties", namespace = "UnityEngine.Rendering", value_type)]
pub struct CameraProperties {
    pub screen_rect: Rect,
    pub view_dir: Vector3,
    pub projection_near: f32,
    pub projection_far: f32,
    pub camera_near: f32,
    pub camera_far: f32,
    pub camera_aspect: f32,
    pub camera_to_world: Matrix4x4,
    pub actual_world_to_clip: Matrix4x4,
    pub camera_clip_to_world: Matrix4x4,
    pub camera_world_to_clip: Matrix4x4,
    pub implicit_projection: Matrix4x4,
    pub stereo_world_to_clip_left: Matrix4x4,
    pub stereo_world_to_clip_right: Matrix4x4,
    pub world_to_camera: Matrix4x4,
    pub up: Vector3,
    pub right: Vector3,
    pub transform_direction: Vector3,
    pub camera_euler: Vector3,
    pub velocity: Vector3,
    pub far_plane_world_space_length: f32,
    pub renderer_count: u32,
    pub m_shadow_cull_planes: *mut c_void,
    pub m_camera_cull_planes: *mut c_void,
    pub base_far_distance: f32,
    pub shadow_cull_center: Vector3,
    pub layer_cull_distances: *mut c_void,
    pub layer_cull_spherical: i32,
    pub core_camera_values: *mut c_void,
    pub camera_type: u32,
    pub projection_is_oblique: i32,
    pub is_implicit_projection_matrix: i32,
}

#[unity_impl]
impl CameraProperties {
    #[unity_method(name = "GetShadowCullingPlane")]
    pub fn get_shadow_culling_plane(&self, index: i32) -> Plane {}

    #[unity_method(name = "SetShadowCullingPlane")]
    pub fn set_shadow_culling_plane(&self, index: i32, plane: Plane) {}

    #[unity_method(name = "GetCameraCullingPlane")]
    pub fn get_camera_culling_plane(&self, index: i32) -> Plane {}

    #[unity_method(name = "SetCameraCullingPlane")]
    pub fn set_camera_culling_plane(&self, index: i32, plane: Plane) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: CameraProperties) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
