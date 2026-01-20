#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector3};
use crate::mscorlib::collections::{Array};
use super::graphics_buffer::GraphicsBuffer;
use super::material::Material;
use super::ras_settings::RASSettings;
use super::ray_tracing_sub_mesh_flags::RayTracingSubMeshFlags;
use super::renderer::Renderer;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RayTracingAccelerationStructure", namespace = "UnityEngine.Experimental.Rendering")]
pub struct RayTracingAccelerationStructure(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RayTracingModeMask {
    #[default]
    Nothing = 0,
    Static = 2,
    DynamicTransform = 4,
    DynamicGeometry = 8,
    Everything = 14,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ManagementMode {
    #[default]
    Manual = 0,
    Automatic = 1,
}

#[unity_impl]
impl RayTracingAccelerationStructure {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(settings: RASSettings) -> Option<Self> {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Destroy(RayTracingAccelerationStructure)")]
    pub fn dispose(accel_struct: Option<RayTracingAccelerationStructure>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Create(RayTracingAccelerationStructure.RASSettings)")]
    pub fn create(desc: RASSettings) -> isize {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Destroy(RayTracingAccelerationStructure)")]
    pub fn release(accel_struct: Option<RayTracingAccelerationStructure>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Build(Vector3)")]
    pub fn build(&self, relative_origin: Vector3) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Build(Vector3)")]
    pub fn update(&self, relative_origin: Vector3) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Update(Vector3)")]
    pub fn update_1(&self, relative_origin: Vector3) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::AddInstance(Renderer,System.Boolean[],System.Boolean[],System.Boolean,System.Boolean,System.UInt32,System.UInt32)")]
    pub fn add_instance(&self, target_renderer: Option<Renderer>, sub_mesh_mask: Array<bool>, sub_mesh_transparency_flags: Array<bool>, enable_triangle_culling: bool, front_triangle_counter_clockwise: bool, mask: u32, id: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::AddInstanceSubMeshFlagsArray(Renderer,RayTracingSubMeshFlags[],System.Boolean,System.Boolean,System.UInt32,System.UInt32)")]
    pub fn add_instance_1(&self, target_renderer: Option<Renderer>, sub_mesh_flags: Array<RayTracingSubMeshFlags>, enable_triangle_culling: bool, front_triangle_counter_clockwise: bool, mask: u32, id: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::RemoveInstance(Renderer)")]
    pub fn remove_instance(&self, target_renderer: Option<Renderer>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::AddInstance_Procedural(GraphicsBuffer,System.UInt32,Material,Matrix4x4,System.Boolean,System.Boolean,System.Boolean,System.UInt32,System.Boolean,System.UInt32)")]
    pub fn add_instance_2(&self, aabb_buffer: Option<GraphicsBuffer>, num_elements: u32, material: Option<Material>, instance_transform: Matrix4x4, is_cut_off: bool, enable_triangle_culling: bool, front_triangle_counter_clockwise: bool, mask: u32, reuse_bounds: bool, id: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::AddInstance_Procedural(GraphicsBuffer,System.UInt32,Material,Matrix4x4,System.Boolean,System.Boolean,System.Boolean,System.UInt32,System.Boolean,System.UInt32)")]
    pub fn add_instance_3(&self, aabb_buffer: Option<GraphicsBuffer>, num_elements: u32, material: Option<Material>, instance_transform: Matrix4x4, is_cut_off: bool, enable_triangle_culling: bool, front_triangle_counter_clockwise: bool, mask: u32, reuse_bounds: bool, id: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::UpdateInstanceTransform(Renderer)")]
    pub fn update_instance_transform(&self, renderer: Option<Renderer>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::UpdateInstanceMask(Renderer,System.UInt32)")]
    pub fn update_instance_mask(&self, renderer: Option<Renderer>, mask: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::UpdateInstanceID(Renderer,System.UInt32)")]
    pub fn update_instance_id(&self, renderer: Option<Renderer>, instance_id: u32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::GetSize")]
    pub fn get_size(&self) -> u64 {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::GetInstanceCount")]
    pub fn get_instance_count(&self) -> u32 {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Create_Injected(RayTracingAccelerationStructure.RASSettings&)")]
    pub fn create_1(desc: &mut RASSettings) -> isize {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Build_Injected(Vector3&)")]
    pub fn build_1(&self, relative_origin: &mut Vector3) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::Update_Injected(Vector3&)")]
    pub fn update_2(&self, relative_origin: &mut Vector3) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingAccelerationStructure::AddInstance_Procedural_Injected(GraphicsBuffer,System.UInt32,Material,Matrix4x4&,System.Boolean,System.Boolean,System.Boolean,System.UInt32,System.Boolean,System.UInt32)")]
    pub fn add_instance_procedural(&self, aabb_buffer: Option<GraphicsBuffer>, num_elements: u32, material: Option<Material>, instance_transform: &mut Matrix4x4, is_cut_off: bool, enable_triangle_culling: bool, front_triangle_counter_clockwise: bool, mask: u32, reuse_bounds: bool, id: u32) {}

}
