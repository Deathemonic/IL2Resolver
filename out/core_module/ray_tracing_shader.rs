#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use crate::mscorlib::collections::{Array};
use super::camera::Camera;
use super::compute_buffer::ComputeBuffer;
use super::graphics_buffer::GraphicsBuffer;
use super::ray_tracing_acceleration_structure::RayTracingAccelerationStructure;
use super::texture::Texture;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RayTracingShader", namespace = "UnityEngine.Experimental.Rendering", inherit = "Object")]
pub struct RayTracingShader(pub *mut c_void);

#[unity_impl]
impl RayTracingShader {
    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::get_maxRecursionDepth")]
    pub fn get_max_recursion_depth(&self) -> f32 {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetFloat(System.Int32,System.Single)")]
    pub fn set_float(&self, name_id: i32, val: f32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetInt(System.Int32,System.Int32)")]
    pub fn set_int(&self, name_id: i32, val: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetVector(System.Int32,Vector4)")]
    pub fn set_vector(&self, name_id: i32, val: Vector4) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetMatrix(System.Int32,Matrix4x4)")]
    pub fn set_matrix(&self, name_id: i32, val: Matrix4x4) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetVectorArray(System.Int32,Vector4[])")]
    pub fn set_vector_array(&self, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetMatrixArray(System.Int32,Matrix4x4[])")]
    pub fn set_matrix_array(&self, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetFloatArray(System.Int32,System.Single[])")]
    pub fn set_floats(&self, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetFloatArray(System.Int32,System.Single[])")]
    pub fn set_floats_1(&self, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetIntArray(System.Int32,System.Int32[])")]
    pub fn set_ints(&self, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetIntArray(System.Int32,System.Int32[])")]
    pub fn set_ints_1(&self, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetTexture(System.Int32,Texture)")]
    pub fn set_texture(&self, name_id: i32, texture: Option<Texture>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetBuffer(System.Int32,ComputeBuffer)")]
    pub fn set_buffer(&self, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetGraphicsBuffer(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_1(&self, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetConstantComputeBuffer(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer(&self, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetConstantComputeBuffer(System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_1(&self, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetConstantGraphicsBuffer(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_2(&self, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetConstantGraphicsBuffer(System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_constant_buffer_3(&self, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetAccelerationStructure(System.Int32,RayTracingAccelerationStructure)")]
    pub fn set_acceleration_structure(&self, name_id: i32, acceleration_structure: Option<RayTracingAccelerationStructure>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetTextureFromGlobal(System.Int32,System.Int32)")]
    pub fn set_texture_from_global(&self, name_id: i32, global_texture_name_id: i32) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetShaderPass(System.String)")]
    pub fn set_shader_pass(&self, pass_name: &str) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::Dispatch(System.String,System.Int32,System.Int32,System.Int32,Camera)")]
    pub fn dispatch(&self, ray_gen_function_name: &str, width: i32, height: i32, depth: i32, camera: Option<Camera>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetGraphicsBuffer(System.Int32,GraphicsBuffer)")]
    pub fn set_buffer_2(&self, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetVector_Injected(System.Int32,Vector4&)")]
    pub fn set_vector_1(&self, name_id: i32, val: &mut Vector4) {}

    #[unity_icall("UnityEngine.Experimental.Rendering.RayTracingShader::SetMatrix_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_matrix_1(&self, name_id: i32, val: &mut Matrix4x4) {}

}
