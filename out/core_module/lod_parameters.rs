#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemObject};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "LODParameters", namespace = "UnityEngine.Rendering", value_type)]
pub struct LODParameters {
    pub m_is_orthographic: i32,
    pub m_camera_position: Vector3,
    pub m_field_of_view: f32,
    pub m_ortho_size: f32,
    pub m_camera_pixel_height: i32,
}

#[unity_impl]
impl LODParameters {
    #[unity_method(name = "get_isOrthographic")]
    pub fn get_is_orthographic(&self) -> bool {}

    #[unity_method(name = "set_isOrthographic")]
    pub fn set_is_orthographic(&self, value: bool) {}

    #[unity_method(name = "get_cameraPosition")]
    pub fn get_camera_position(&self) -> Vector3 {}

    #[unity_method(name = "set_cameraPosition")]
    pub fn set_camera_position(&self, value: Vector3) {}

    #[unity_method(name = "get_fieldOfView")]
    pub fn get_field_of_view(&self) -> f32 {}

    #[unity_method(name = "set_fieldOfView")]
    pub fn set_field_of_view(&self, value: f32) {}

    #[unity_method(name = "get_orthoSize")]
    pub fn get_ortho_size(&self) -> f32 {}

    #[unity_method(name = "set_orthoSize")]
    pub fn set_ortho_size(&self, value: f32) {}

    #[unity_method(name = "get_cameraPixelHeight")]
    pub fn get_camera_pixel_height(&self) -> i32 {}

    #[unity_method(name = "set_cameraPixelHeight")]
    pub fn set_camera_pixel_height(&self, value: i32) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: LODParameters) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
