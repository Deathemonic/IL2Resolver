#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Quaternion, Vector3};
use super::color::Color;
use super::material::Material;
use super::mesh::Mesh;
use super::rect::Rect;
use super::texture::Texture;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Gizmos", namespace = "UnityEngine")]
pub struct Gizmos(pub *mut c_void);

#[unity_impl]
impl Gizmos {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Gizmos::get_color_Injected(Color&)")]
    pub fn get_color(ret: &mut Color) {}

    #[unity_icall("UnityEngine.Gizmos::set_color_Injected(Color&)")]
    pub fn set_color(value: &mut Color) {}

    #[unity_icall("UnityEngine.Gizmos::get_matrix_Injected(Matrix4x4&)")]
    pub fn get_matrix(ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Gizmos::set_matrix_Injected(Matrix4x4&)")]
    pub fn set_matrix(value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Gizmos::get_exposure")]
    pub fn get_exposure() -> Option<Texture> {}

    #[unity_icall("UnityEngine.Gizmos::set_exposure(Texture)")]
    pub fn set_exposure(value: Option<Texture>) {}

    #[unity_icall("UnityEngine.Gizmos::get_probeSize")]
    pub fn get_probe_size() -> f32 {}

    #[unity_icall("UnityEngine.Gizmos::DrawLine_Injected(Vector3&,Vector3&)")]
    pub fn draw_ray(from: &mut Vector3, to: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawLine_Injected(Vector3&,Vector3&)")]
    pub fn draw_ray_1(from: &mut Vector3, to: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawMesh_Injected(Mesh,System.Int32,Vector3&,Quaternion&,Vector3&)")]
    pub fn draw_mesh(mesh: Option<Mesh>, submesh_index: i32, position: &mut Vector3, rotation: &mut Quaternion, scale: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawWireMesh_Injected(Mesh,System.Int32,Vector3&,Quaternion&,Vector3&)")]
    pub fn draw_wire_mesh(mesh: Option<Mesh>, submesh_index: i32, position: &mut Vector3, rotation: &mut Quaternion, scale: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawGUITexture_Injected(Rect&,Texture,System.Int32,System.Int32,System.Int32,System.Int32,Material)")]
    pub fn draw_gui_texture(screen_rect: &mut Rect, texture: Option<Texture>, left_border: i32, right_border: i32, top_border: i32, bottom_border: i32, mat: Option<Material>) {}

    #[unity_icall("UnityEngine.Gizmos::DrawLine_Injected(Vector3&,Vector3&)")]
    pub fn draw_line(from: &mut Vector3, to: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawWireSphere_Injected(Vector3&,System.Single)")]
    pub fn draw_wire_sphere(center: &mut Vector3, radius: f32) {}

    #[unity_icall("UnityEngine.Gizmos::DrawSphere_Injected(Vector3&,System.Single)")]
    pub fn draw_sphere(center: &mut Vector3, radius: f32) {}

    #[unity_icall("UnityEngine.Gizmos::DrawWireCube_Injected(Vector3&,Vector3&)")]
    pub fn draw_wire_cube(center: &mut Vector3, size: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawCube_Injected(Vector3&,Vector3&)")]
    pub fn draw_cube(center: &mut Vector3, size: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawMesh_Injected(Mesh,System.Int32,Vector3&,Quaternion&,Vector3&)")]
    pub fn draw_mesh_1(mesh: Option<Mesh>, submesh_index: i32, position: &mut Vector3, rotation: &mut Quaternion, scale: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawWireMesh_Injected(Mesh,System.Int32,Vector3&,Quaternion&,Vector3&)")]
    pub fn draw_wire_mesh_1(mesh: Option<Mesh>, submesh_index: i32, position: &mut Vector3, rotation: &mut Quaternion, scale: &mut Vector3) {}

    #[unity_icall("UnityEngine.Gizmos::DrawIcon_Injected(Vector3&,System.String,System.Boolean,Color&)")]
    pub fn draw_icon(center: &mut Vector3, name: &str, allow_scaling: bool, tint: &mut Color) {}

    #[unity_icall("UnityEngine.Gizmos::DrawIcon_Injected(Vector3&,System.String,System.Boolean,Color&)")]
    pub fn draw_icon_1(center: &mut Vector3, name: &str, allow_scaling: bool, tint: &mut Color) {}

    #[unity_icall("UnityEngine.Gizmos::DrawGUITexture_Injected(Rect&,Texture,System.Int32,System.Int32,System.Int32,System.Int32,Material)")]
    pub fn draw_gui_texture_1(screen_rect: &mut Rect, texture: Option<Texture>, left_border: i32, right_border: i32, top_border: i32, bottom_border: i32, mat: Option<Material>) {}

    #[unity_icall("UnityEngine.Gizmos::DrawFrustum_Injected(Vector3&,System.Single,System.Single,System.Single,System.Single)")]
    pub fn draw_frustum(center: &mut Vector3, fov: f32, max_range: f32, min_range: f32, aspect: f32) {}

}
