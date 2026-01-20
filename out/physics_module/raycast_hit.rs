#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2, Vector3};
use super::articulation_body::ArticulationBody;
use super::collider::Collider;
use super::rigidbody::Rigidbody;
use crate::core_module::Transform;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "RaycastHit", namespace = "UnityEngine", value_type)]
pub struct RaycastHit {
    pub m_point: Vector3,
    pub m_normal: Vector3,
    pub m_face_id: u32,
    pub m_distance: f32,
    pub m_uv: Vector2,
    pub m_collider: i32,
}

#[unity_impl]
impl RaycastHit {
    #[unity_method(name = "get_textureCoord1")]
    pub fn get_texture_coord1(&self) -> Vector2 {}

    #[unity_method(name = "get_collider")]
    pub fn get_collider(&self) -> Option<Collider> {}

    #[unity_method(name = "get_colliderInstanceID")]
    pub fn get_collider_instance_id(&self) -> i32 {}

    #[unity_method(name = "get_point")]
    pub fn get_point(&self) -> Vector3 {}

    #[unity_method(name = "set_point")]
    pub fn set_point(&self, value: Vector3) {}

    #[unity_method(name = "get_normal")]
    pub fn get_normal(&self) -> Vector3 {}

    #[unity_method(name = "set_normal")]
    pub fn set_normal(&self, value: Vector3) {}

    #[unity_method(name = "get_barycentricCoordinate")]
    pub fn get_barycentric_coordinate(&self) -> Vector3 {}

    #[unity_method(name = "set_barycentricCoordinate")]
    pub fn set_barycentric_coordinate(&self, value: Vector3) {}

    #[unity_method(name = "get_distance")]
    pub fn get_distance(&self) -> f32 {}

    #[unity_method(name = "set_distance")]
    pub fn set_distance(&self, value: f32) {}

    #[unity_method(name = "get_triangleIndex")]
    pub fn get_triangle_index(&self) -> i32 {}

    #[unity_method(name = "get_textureCoord")]
    pub fn get_texture_coord(&self) -> Vector2 {}

    #[unity_method(name = "get_textureCoord2")]
    pub fn get_texture_coord2(&self) -> Vector2 {}

    #[unity_method(name = "get_transform")]
    pub fn get_transform(&self) -> Option<Transform> {}

    #[unity_method(name = "get_rigidbody")]
    pub fn get_rigidbody(&self) -> Option<Rigidbody> {}

    #[unity_method(name = "get_articulationBody")]
    pub fn get_articulation_body(&self) -> Option<ArticulationBody> {}

    #[unity_method(name = "get_lightmapCoord")]
    pub fn get_lightmap_coord(&self) -> Vector2 {}

    #[unity_icall("UnityEngine.RaycastHit::CalculateRaycastTexCoord(Collider,Vector2,Vector3,System.UInt32,System.Int32)")]
    pub fn calculate_raycast_tex_coord(collider: Option<Collider>, uv: Vector2, pos: Vector3, face: u32, textcoord: i32) -> Vector2 {}

    #[unity_icall("UnityEngine.RaycastHit::CalculateRaycastTexCoord_Injected(Collider,Vector2&,Vector3&,System.UInt32,System.Int32,Vector2&)")]
    pub fn calculate_raycast_tex_coord_1(collider: Option<Collider>, uv: &mut Vector2, pos: &mut Vector3, face: u32, textcoord: i32, ret: &mut Vector2) {}

}
