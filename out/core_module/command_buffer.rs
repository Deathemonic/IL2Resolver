#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector3, Vector4};
use crate::mscorlib::{SystemArray, SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::camera::Camera;
use super::camera_late_latch_matrix_type::CameraLateLatchMatrixType;
use super::color::Color;
use super::command_buffer_execution_flags::CommandBufferExecutionFlags;
use super::compute_buffer::ComputeBuffer;
use super::compute_shader::ComputeShader;
use super::cubemap_face::CubemapFace;
use super::custom_sampler::CustomSampler;
use super::filter_mode::FilterMode;
use super::global_keyword::GlobalKeyword;
use super::gpu_fence::GPUFence;
use super::graphics_buffer::GraphicsBuffer;
use super::graphics_fence_type::GraphicsFenceType;
use super::graphics_format::GraphicsFormat;
use super::local_keyword::LocalKeyword;
use super::material::Material;
use super::material_property_block::MaterialPropertyBlock;
use super::mesh::Mesh;
use super::mesh_topology::MeshTopology;
use super::ray_tracing_acceleration_structure::RayTracingAccelerationStructure;
use super::ray_tracing_shader::RayTracingShader;
use super::rect::Rect;
use super::rect_int::RectInt;
use super::render_buffer_load_action::RenderBufferLoadAction;
use super::render_buffer_store_action::RenderBufferStoreAction;
use super::renderer::Renderer;
use super::renderer_list::RendererList;
use super::render_target_flags::RenderTargetFlags;
use super::render_target_identifier::RenderTargetIdentifier;
use super::render_texture::RenderTexture;
use super::render_texture_descriptor::RenderTextureDescriptor;
use super::render_texture_memoryless::RenderTextureMemoryless;
use super::render_texture_sub_element::RenderTextureSubElement;
use super::rt_clear_flags::RTClearFlags;
use super::shadow_sampling_mode::ShadowSamplingMode;
use super::single_pass_stereo_mode::SinglePassStereoMode;
use super::synchronisation_stage::SynchronisationStage;
use super::synchronisation_stage_flags::SynchronisationStageFlags;
use super::texture::Texture;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CommandBuffer", namespace = "UnityEngine.Rendering")]
pub struct CommandBuffer(pub *mut c_void);

#[unity_impl]
impl CommandBuffer {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::set_name(System.String)")]
    pub fn set_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::get_sizeInBytes")]
    pub fn get_size_in_bytes(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ReleaseBuffer")]
    pub fn dispose(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ReleaseBuffer")]
    pub fn release(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::WaitOnGPUFence_Internal(System.IntPtr,SynchronisationStageFlags)")]
    pub fn wait_on_async_graphics_fence(&self, fence_ptr: isize, stage: SynchronisationStageFlags) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::WaitOnGPUFence_Internal(System.IntPtr,SynchronisationStageFlags)")]
    pub fn wait_on_async_graphics_fence_1(&self, fence_ptr: isize, stage: SynchronisationStageFlags) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeFloatParam(ComputeShader,System.Int32,System.Single)")]
    pub fn set_compute_float_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: f32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeIntParam(ComputeShader,System.Int32,System.Int32)")]
    pub fn set_compute_int_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeVectorParam(ComputeShader,System.Int32,Vector4)")]
    pub fn set_compute_vector_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeVectorArrayParam(ComputeShader,System.Int32,Vector4[])")]
    pub fn set_compute_vector_array_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeMatrixParam(ComputeShader,System.Int32,Matrix4x4)")]
    pub fn set_compute_matrix_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeMatrixArrayParam(ComputeShader,System.Int32,Matrix4x4[])")]
    pub fn set_compute_matrix_array_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeFloats(ComputeShader,System.Int32,System.Single[])")]
    pub fn set_compute_float_params(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeFloats(ComputeShader,System.Int32,System.Single[])")]
    pub fn set_compute_float_params_1(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeInts(ComputeShader,System.Int32,System.Int32[])")]
    pub fn set_compute_int_params(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeInts(ComputeShader,System.Int32,System.Int32[])")]
    pub fn set_compute_int_params_1(&self, compute_shader: Option<ComputeShader>, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param_1(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param_2(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param_3(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param_4(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeTextureParam(ComputeShader,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,RenderTextureSubElement)")]
    pub fn set_compute_texture_param_5(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, rt: &mut RenderTargetIdentifier, mip_level: i32, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeBufferParam(ComputeShader,System.Int32,System.Int32,ComputeBuffer)")]
    pub fn set_compute_buffer_param(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeBufferParam(ComputeShader,System.Int32,System.Int32,ComputeBuffer)")]
    pub fn set_compute_buffer_param_1(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeGraphicsBufferParam(ComputeShader,System.Int32,System.Int32,GraphicsBuffer)")]
    pub fn set_compute_buffer_param_2(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeGraphicsBufferParam(ComputeShader,System.Int32,System.Int32,GraphicsBuffer)")]
    pub fn set_compute_buffer_param_3(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, name_id: i32, buffer: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeConstantComputeBufferParam(ComputeShader,System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_compute_constant_buffer_param(&self, compute_shader: Option<ComputeShader>, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeConstantComputeBufferParam(ComputeShader,System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_compute_constant_buffer_param_1(&self, compute_shader: Option<ComputeShader>, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeConstantGraphicsBufferParam(ComputeShader,System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_compute_constant_buffer_param_2(&self, compute_shader: Option<ComputeShader>, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetComputeConstantGraphicsBufferParam(ComputeShader,System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_compute_constant_buffer_param_3(&self, compute_shader: Option<ComputeShader>, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DispatchCompute(ComputeShader,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn dispatch_compute(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, thread_groups_x: i32, thread_groups_y: i32, thread_groups_z: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DispatchComputeIndirect(ComputeShader,System.Int32,ComputeBuffer,System.UInt32)")]
    pub fn dispatch_compute_1(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, indirect_buffer: Option<ComputeBuffer>, args_offset: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DispatchComputeIndirectGraphicsBuffer(ComputeShader,System.Int32,GraphicsBuffer,System.UInt32)")]
    pub fn dispatch_compute_2(&self, compute_shader: Option<ComputeShader>, kernel_index: i32, indirect_buffer: Option<GraphicsBuffer>, args_offset: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_BuildRayTracingAccelerationStructure(RayTracingAccelerationStructure,Vector3)")]
    pub fn build_ray_tracing_acceleration_structure(&self, acceleration_structure: Option<RayTracingAccelerationStructure>, relative_origin: Vector3) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingAccelerationStructure(RayTracingShader,System.Int32,RayTracingAccelerationStructure)")]
    pub fn set_ray_tracing_acceleration_structure(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, acceleration_structure: Option<RayTracingAccelerationStructure>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingAccelerationStructure(RayTracingShader,System.Int32,RayTracingAccelerationStructure)")]
    pub fn set_ray_tracing_acceleration_structure_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, acceleration_structure: Option<RayTracingAccelerationStructure>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingBufferParam(RayTracingShader,System.Int32,ComputeBuffer)")]
    pub fn set_ray_tracing_buffer_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingBufferParam(RayTracingShader,System.Int32,ComputeBuffer)")]
    pub fn set_ray_tracing_buffer_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingConstantComputeBufferParam(RayTracingShader,System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_constant_buffer_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingConstantComputeBufferParam(RayTracingShader,System.Int32,ComputeBuffer,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_constant_buffer_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<ComputeBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingConstantGraphicsBufferParam(RayTracingShader,System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_constant_buffer_param_2(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingConstantGraphicsBufferParam(RayTracingShader,System.Int32,GraphicsBuffer,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_constant_buffer_param_3(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, buffer: Option<GraphicsBuffer>, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingTextureParam(RayTracingShader,System.Int32,RenderTargetIdentifier&)")]
    pub fn set_ray_tracing_texture_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, rt: &mut RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingTextureParam(RayTracingShader,System.Int32,RenderTargetIdentifier&)")]
    pub fn set_ray_tracing_texture_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, rt: &mut RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingFloatParam(RayTracingShader,System.Int32,System.Single)")]
    pub fn set_ray_tracing_float_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: f32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingFloatParam(RayTracingShader,System.Int32,System.Single)")]
    pub fn set_ray_tracing_float_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: f32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingFloats(RayTracingShader,System.Int32,System.Single[])")]
    pub fn set_ray_tracing_float_params(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingFloats(RayTracingShader,System.Int32,System.Single[])")]
    pub fn set_ray_tracing_float_params_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingIntParam(RayTracingShader,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_int_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingIntParam(RayTracingShader,System.Int32,System.Int32)")]
    pub fn set_ray_tracing_int_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingInts(RayTracingShader,System.Int32,System.Int32[])")]
    pub fn set_ray_tracing_int_params(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingInts(RayTracingShader,System.Int32,System.Int32[])")]
    pub fn set_ray_tracing_int_params_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<i32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingVectorParam(RayTracingShader,System.Int32,Vector4)")]
    pub fn set_ray_tracing_vector_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingVectorParam(RayTracingShader,System.Int32,Vector4)")]
    pub fn set_ray_tracing_vector_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingVectorArrayParam(RayTracingShader,System.Int32,Vector4[])")]
    pub fn set_ray_tracing_vector_array_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingVectorArrayParam(RayTracingShader,System.Int32,Vector4[])")]
    pub fn set_ray_tracing_vector_array_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingMatrixParam(RayTracingShader,System.Int32,Matrix4x4)")]
    pub fn set_ray_tracing_matrix_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingMatrixParam(RayTracingShader,System.Int32,Matrix4x4)")]
    pub fn set_ray_tracing_matrix_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingMatrixArrayParam(RayTracingShader,System.Int32,Matrix4x4[])")]
    pub fn set_ray_tracing_matrix_array_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingMatrixArrayParam(RayTracingShader,System.Int32,Matrix4x4[])")]
    pub fn set_ray_tracing_matrix_array_param_1(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DispatchRays(RayTracingShader,System.String,System.UInt32,System.UInt32,System.UInt32,Camera)")]
    pub fn dispatch_rays(&self, ray_tracing_shader: Option<RayTracingShader>, ray_gen_shader_name: &str, width: u32, height: u32, depth: u32, camera: Option<Camera>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_mesh(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_mesh_1(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_mesh_2(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_renderer(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_renderer_1(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawRendererList(RendererList)")]
    pub fn draw_renderer_list(&self, renderer_list: RendererList) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural_1(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexed(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_2(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, index_count: i32, instance_count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexed(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_3(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, index_count: i32, instance_count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural_indirect(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural_indirect_1(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirect(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_indirect_2(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirect(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_indirect_3(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural_indirect_4(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_procedural_indirect_5(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirectGraphicsBuffer(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_indirect_6(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirectGraphicsBuffer(GraphicsBuffer,Matrix4x4,Material,System.Int32,MeshTopology,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_procedural_indirect_7(&self, index_buffer: Option<GraphicsBuffer>, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn draw_mesh_instanced(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstancedIndirect(Mesh,System.Int32,Material,System.Int32,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_mesh_instanced_indirect(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstancedIndirect(Mesh,System.Int32,Material,System.Int32,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_mesh_instanced_indirect_1(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstancedIndirectGraphicsBuffer(Mesh,System.Int32,Material,System.Int32,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_mesh_instanced_indirect_2(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstancedIndirectGraphicsBuffer(Mesh,System.Int32,Material,System.Int32,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn draw_mesh_instanced_indirect_3(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawOcclusionMesh(RectInt)")]
    pub fn draw_occlusion_mesh(&self, normalized_cam_viewport: RectInt) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn set_random_write_target(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ValidateAgainstExecutionFlags(CommandBufferExecutionFlags,CommandBufferExecutionFlags)")]
    pub fn set_random_write_target_1(&self, required_flags: CommandBufferExecutionFlags, invalid_flags: CommandBufferExecutionFlags) -> bool {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyCounterValueCC(ComputeBuffer,ComputeBuffer,System.UInt32)")]
    pub fn copy_counter_value(&self, src: Option<ComputeBuffer>, dst: Option<ComputeBuffer>, dst_offset_bytes: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyCounterValueGC(GraphicsBuffer,ComputeBuffer,System.UInt32)")]
    pub fn copy_counter_value_1(&self, src: Option<GraphicsBuffer>, dst: Option<ComputeBuffer>, dst_offset_bytes: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyCounterValueCG(ComputeBuffer,GraphicsBuffer,System.UInt32)")]
    pub fn copy_counter_value_2(&self, src: Option<ComputeBuffer>, dst: Option<GraphicsBuffer>, dst_offset_bytes: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyCounterValueGG(GraphicsBuffer,GraphicsBuffer,System.UInt32)")]
    pub fn copy_counter_value_3(&self, src: Option<GraphicsBuffer>, dst: Option<GraphicsBuffer>, dst_offset_bytes: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyTexture_Internal(RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn copy_texture(&self, src: &mut RenderTargetIdentifier, src_element: i32, src_mip: i32, src_x: i32, src_y: i32, src_width: i32, src_height: i32, dst: &mut RenderTargetIdentifier, dst_element: i32, dst_mip: i32, dst_x: i32, dst_y: i32, mode: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyTexture_Internal(RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn copy_texture_1(&self, src: &mut RenderTargetIdentifier, src_element: i32, src_mip: i32, src_x: i32, src_y: i32, src_width: i32, src_height: i32, dst: &mut RenderTargetIdentifier, dst_element: i32, dst_mip: i32, dst_x: i32, dst_y: i32, mode: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyTexture_Internal(RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn copy_texture_2(&self, src: &mut RenderTargetIdentifier, src_element: i32, src_mip: i32, src_x: i32, src_y: i32, src_width: i32, src_height: i32, dst: &mut RenderTargetIdentifier, dst_element: i32, dst_mip: i32, dst_x: i32, dst_y: i32, mode: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyTexture_Internal(RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,RenderTargetIdentifier&,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn copy_texture_3(&self, src: &mut RenderTargetIdentifier, src_element: i32, src_mip: i32, src_x: i32, src_y: i32, src_width: i32, src_height: i32, dst: &mut RenderTargetIdentifier, dst_element: i32, dst_mip: i32, dst_x: i32, dst_y: i32, mode: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalFloat(System.Int32,System.Single)")]
    pub fn set_global_float(&self, name_id: i32, value: f32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalInt(System.Int32,System.Int32)")]
    pub fn set_global_int(&self, name_id: i32, value: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalInteger(System.Int32,System.Int32)")]
    pub fn set_global_integer(&self, name_id: i32, value: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalVector(System.Int32,Vector4)")]
    pub fn set_global_vector(&self, name_id: i32, value: Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalColor(System.Int32,Color)")]
    pub fn set_global_color(&self, name_id: i32, value: Color) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalMatrix(System.Int32,Matrix4x4)")]
    pub fn set_global_matrix(&self, name_id: i32, value: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalFloatArrayListImpl(System.Int32,System.Object)")]
    pub fn set_global_float_array(&self, name_id: i32, values: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalFloatArray(System.Int32,System.Single[])")]
    pub fn set_global_float_array_1(&self, name_id: i32, values: Array<f32>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalVectorArrayListImpl(System.Int32,System.Object)")]
    pub fn set_global_vector_array(&self, name_id: i32, values: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalVectorArray(System.Int32,Vector4[])")]
    pub fn set_global_vector_array_1(&self, name_id: i32, values: Array<Vector4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalMatrixArrayListImpl(System.Int32,System.Object)")]
    pub fn set_global_matrix_array(&self, name_id: i32, values: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalMatrixArray(System.Int32,Matrix4x4[])")]
    pub fn set_global_matrix_array_1(&self, name_id: i32, values: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalTexture_Impl(System.Int32,RenderTargetIdentifier&,RenderTextureSubElement)")]
    pub fn set_global_texture(&self, name_id: i32, rt: &mut RenderTargetIdentifier, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalTexture_Impl(System.Int32,RenderTargetIdentifier&,RenderTextureSubElement)")]
    pub fn set_global_texture_1(&self, name_id: i32, rt: &mut RenderTargetIdentifier, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalTexture_Impl(System.Int32,RenderTargetIdentifier&,RenderTextureSubElement)")]
    pub fn set_global_texture_2(&self, name_id: i32, rt: &mut RenderTargetIdentifier, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalTexture_Impl(System.Int32,RenderTargetIdentifier&,RenderTextureSubElement)")]
    pub fn set_global_texture_3(&self, name_id: i32, rt: &mut RenderTargetIdentifier, element: RenderTextureSubElement) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalBufferInternal(System.Int32,ComputeBuffer)")]
    pub fn set_global_buffer(&self, name_id: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalBufferInternal(System.Int32,ComputeBuffer)")]
    pub fn set_global_buffer_1(&self, name_id: i32, value: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalGraphicsBufferInternal(System.Int32,GraphicsBuffer)")]
    pub fn set_global_buffer_2(&self, name_id: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalGraphicsBufferInternal(System.Int32,GraphicsBuffer)")]
    pub fn set_global_buffer_3(&self, name_id: i32, value: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalConstantBufferInternal(ComputeBuffer,System.Int32,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer(&self, buffer: Option<ComputeBuffer>, name_id: i32, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalConstantBufferInternal(ComputeBuffer,System.Int32,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer_1(&self, buffer: Option<ComputeBuffer>, name_id: i32, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalConstantGraphicsBufferInternal(GraphicsBuffer,System.Int32,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer_2(&self, buffer: Option<GraphicsBuffer>, name_id: i32, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalConstantGraphicsBufferInternal(GraphicsBuffer,System.Int32,System.Int32,System.Int32)")]
    pub fn set_global_constant_buffer_3(&self, buffer: Option<GraphicsBuffer>, name_id: i32, offset: i32, size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetSinglePassStereo(SinglePassStereoMode)")]
    pub fn set_single_pass_stereo(&self, mode: SinglePassStereoMode) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IssuePluginCustomTextureUpdateInternal(System.IntPtr,Texture,System.UInt32,System.Boolean)")]
    pub fn issue_plugin_custom_texture_update(&self, callback: isize, target_texture: Option<Texture>, user_data: u32, use_new_unity_rendering_ext_texture_update_params_v2: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IssuePluginCustomTextureUpdateInternal(System.IntPtr,Texture,System.UInt32,System.Boolean)")]
    pub fn issue_plugin_custom_texture_update_v1(&self, callback: isize, target_texture: Option<Texture>, user_data: u32, use_new_unity_rendering_ext_texture_update_params_v2: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CopyBufferImpl(GraphicsBuffer,GraphicsBuffer)")]
    pub fn copy_buffer(&self, source: Option<GraphicsBuffer>, dest: Option<GraphicsBuffer>) {}

    #[unity_method(name = "CreateGPUFence")]
    pub fn create_gpu_fence(&self, stage: SynchronisationStage) -> GPUFence {}

    #[unity_method(name = "CreateGPUFence")]
    pub fn create_gpu_fence_1(&self) -> GPUFence {}

    #[unity_method(name = "WaitOnGPUFence")]
    pub fn wait_on_gpu_fence(&self, fence: GPUFence, stage: SynchronisationStage) {}

    #[unity_method(name = "WaitOnGPUFence")]
    pub fn wait_on_gpu_fence_1(&self, fence: GPUFence) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetComputeBufferData(ComputeBuffer,System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_compute_buffer_data(&self, buffer: Option<ComputeBuffer>, data: Option<SystemArray>, managed_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetComputeBufferData(ComputeBuffer,System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_compute_buffer_data_1(&self, buffer: Option<ComputeBuffer>, data: Option<SystemArray>, managed_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetComputeBufferCounterValue(ComputeBuffer,System.UInt32)")]
    pub fn set_compute_buffer_counter_value(&self, buffer: Option<ComputeBuffer>, counter_value: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::WaitAllAsyncReadbackRequests")]
    pub fn wait_all_async_readback_requests(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_1(ComputeBuffer,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback1(&self, src: Option<ComputeBuffer>, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_2(ComputeBuffer,System.Int32,System.Int32,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback2(&self, src: Option<ComputeBuffer>, size: i32, offset: i32, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_3(Texture,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback3(&self, src: Option<Texture>, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_4(Texture,System.Int32,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback4(&self, src: Option<Texture>, mip_index: i32, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_5(Texture,System.Int32,GraphicsFormat,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback5(&self, src: Option<Texture>, mip_index: i32, dst_format: GraphicsFormat, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_6(Texture,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback6(&self, src: Option<Texture>, mip_index: i32, x: i32, width: i32, y: i32, height: i32, z: i32, depth: i32, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_7(Texture,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,GraphicsFormat,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback7(&self, src: Option<Texture>, mip_index: i32, x: i32, width: i32, y: i32, height: i32, z: i32, depth: i32, dst_format: GraphicsFormat, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_8(GraphicsBuffer,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback8(&self, src: Option<GraphicsBuffer>, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_RequestAsyncReadback_9(GraphicsBuffer,System.Int32,System.Int32,Action<AsyncGPUReadbackRequest>,AsyncRequestNativeArrayData*)")]
    pub fn internal_request_async_readback9(&self, src: Option<GraphicsBuffer>, size: i32, offset: i32, callback: *mut c_void, native_array_data: *mut *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetInvertCulling(System.Boolean)")]
    pub fn set_invert_culling(&self, invert_culling: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InitBuffer")]
    pub fn init_buffer() -> isize {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::CreateGPUFence_Internal(GraphicsFenceType,SynchronisationStageFlags)")]
    pub fn create_gpu_fence_internal(&self, fence_type: GraphicsFenceType, stage: SynchronisationStageFlags) -> isize {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRayTracingShaderPass(RayTracingShader,System.String)")]
    pub fn set_ray_tracing_shader_pass(&self, ray_tracing_shader: Option<RayTracingShader>, pass_name: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_GenerateMips(RenderTargetIdentifier)")]
    pub fn internal_generate_mips(&self, rt: RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_ResolveAntiAliasedSurface(RenderTexture,RenderTexture)")]
    pub fn internal_resolve_anti_aliased_surface(&self, rt: Option<RenderTexture>, target: Option<RenderTexture>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Clear")]
    pub fn clear(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMesh(Mesh,Matrix4x4,Material,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_mesh(&self, mesh: Option<Mesh>, matrix: Matrix4x4, material: Option<Material>, submesh_index: i32, shader_pass: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawRenderer(Renderer,Material,System.Int32,System.Int32)")]
    pub fn internal_draw_renderer(&self, renderer: Option<Renderer>, material: Option<Material>, submesh_index: i32, shader_pass: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProcedural(Matrix4x4,Material,System.Int32,MeshTopology,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural(&self, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, vertex_count: i32, instance_count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndirect(Matrix4x4,Material,System.Int32,MeshTopology,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indirect(&self, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndirectGraphicsBuffer(Matrix4x4,Material,System.Int32,MeshTopology,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indirect_graphics_buffer(&self, matrix: Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstanced(Mesh,System.Int32,Material,System.Int32,Matrix4x4[],System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_mesh_instanced(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, matrices: Array<Matrix4x4>, count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMeshInstancedProcedural(Mesh,System.Int32,Material,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_mesh_instanced_procedural(&self, mesh: Option<Mesh>, submesh_index: i32, material: Option<Material>, shader_pass: i32, count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRandomWriteTarget_Texture(System.Int32,RenderTargetIdentifier&)")]
    pub fn set_random_write_target_texture(&self, index: i32, rt: &mut RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRandomWriteTarget_Buffer(System.Int32,ComputeBuffer,System.Boolean)")]
    pub fn set_random_write_target_buffer(&self, index: i32, uav: Option<ComputeBuffer>, preserve_counter_value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRandomWriteTarget_GraphicsBuffer(System.Int32,GraphicsBuffer,System.Boolean)")]
    pub fn set_random_write_target_graphics_buffer(&self, index: i32, uav: Option<GraphicsBuffer>, preserve_counter_value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ClearRandomWriteTargets")]
    pub fn clear_random_write_targets(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewport(Rect)")]
    pub fn set_viewport(&self, pixel_rect: Rect) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableScissorRect(Rect)")]
    pub fn enable_scissor_rect(&self, scissor: Rect) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableScissorRect")]
    pub fn disable_scissor_rect(&self) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Blit_Texture(Texture,RenderTargetIdentifier&,Material,System.Int32,Vector2,Vector2,System.Int32,System.Int32)")]
    pub fn blit_texture(&self, source: Option<Texture>, dest: &mut RenderTargetIdentifier, mat: Option<Material>, pass: i32, scale: Vector2, offset: Vector2, source_depth_slice: i32, dest_depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Blit_Identifier(RenderTargetIdentifier&,RenderTargetIdentifier&,Material,System.Int32,Vector2,Vector2,System.Int32,System.Int32)")]
    pub fn blit_identifier(&self, source: &mut RenderTargetIdentifier, dest: &mut RenderTargetIdentifier, mat: Option<Material>, pass: i32, scale: Vector2, offset: Vector2, source_depth_slice: i32, dest_depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_1(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_2(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_3(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_4(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_5(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_6(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_7(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_8(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_9(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_10(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_11(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRT(System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,RenderTextureMemoryless,System.Boolean)")]
    pub fn get_temporary_rt_12(&self, name_id: i32, width: i32, height: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, memoryless_mode: RenderTextureMemoryless, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTWithDescriptor(System.Int32,RenderTextureDescriptor,FilterMode)")]
    pub fn get_temporary_rt_13(&self, name_id: i32, desc: RenderTextureDescriptor, filter: FilterMode) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTWithDescriptor(System.Int32,RenderTextureDescriptor,FilterMode)")]
    pub fn get_temporary_rt_14(&self, name_id: i32, desc: RenderTextureDescriptor, filter: FilterMode) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_1(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_2(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_3(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_4(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_5(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_6(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_7(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_8(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTArray(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,FilterMode,GraphicsFormat,System.Int32,System.Boolean,System.Boolean)")]
    pub fn get_temporary_rt_array_9(&self, name_id: i32, width: i32, height: i32, slices: i32, depth_buffer: i32, filter: FilterMode, format: GraphicsFormat, anti_aliasing: i32, enable_random_write: bool, use_dynamic_scale: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ReleaseTemporaryRT(System.Int32)")]
    pub fn release_temporary_rt(&self, name_id: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ClearRenderTarget(RTClearFlags,Color,System.Single,System.UInt32)")]
    pub fn clear_render_target(&self, clear_flags: RTClearFlags, background_color: Color, depth: f32, stencil: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableShaderKeyword(System.String)")]
    pub fn enable_shader_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableGlobalKeyword(GlobalKeyword)")]
    pub fn enable_global_keyword(&self, keyword: GlobalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableMaterialKeyword(Material,LocalKeyword)")]
    pub fn enable_material_keyword(&self, material: Option<Material>, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableComputeKeyword(ComputeShader,LocalKeyword)")]
    pub fn enable_compute_keyword(&self, compute_shader: Option<ComputeShader>, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableShaderKeyword(System.String)")]
    pub fn disable_shader_keyword(&self, keyword: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableGlobalKeyword(GlobalKeyword)")]
    pub fn disable_global_keyword(&self, keyword: GlobalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableMaterialKeyword(Material,LocalKeyword)")]
    pub fn disable_material_keyword(&self, material: Option<Material>, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableComputeKeyword(ComputeShader,LocalKeyword)")]
    pub fn disable_compute_keyword(&self, compute_shader: Option<ComputeShader>, keyword: LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalKeyword(GlobalKeyword,System.Boolean)")]
    pub fn set_global_keyword(&self, keyword: GlobalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetMaterialKeyword(Material,LocalKeyword,System.Boolean)")]
    pub fn set_material_keyword(&self, material: Option<Material>, keyword: LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeKeyword(ComputeShader,LocalKeyword,System.Boolean)")]
    pub fn set_compute_keyword(&self, compute_shader: Option<ComputeShader>, keyword: LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewMatrix(Matrix4x4)")]
    pub fn set_view_matrix(&self, view: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetProjectionMatrix(Matrix4x4)")]
    pub fn set_projection_matrix(&self, proj: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewProjectionMatrices(Matrix4x4,Matrix4x4)")]
    pub fn set_view_projection_matrices(&self, view: Matrix4x4, proj: Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalDepthBias(System.Single,System.Single)")]
    pub fn set_global_depth_bias(&self, bias: f32, slope_bias: f32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetExecutionFlags(CommandBufferExecutionFlags)")]
    pub fn set_execution_flags(&self, flags: CommandBufferExecutionFlags) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetLateLatchProjectionMatrices(Matrix4x4[])")]
    pub fn set_late_latch_projection_matrices(&self, projection_mat: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::MarkLateLatchMatrixShaderPropertyID(CameraLateLatchMatrixType,System.Int32)")]
    pub fn mark_late_latch_matrix_shader_property_id(&self, matrix_property_type: CameraLateLatchMatrixType, shader_property_id: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::UnmarkLateLatchMatrix(CameraLateLatchMatrixType)")]
    pub fn unmark_late_latch_matrix(&self, matrix_property_type: CameraLateLatchMatrixType) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetShadowSamplingMode_Impl(RenderTargetIdentifier&,ShadowSamplingMode)")]
    pub fn set_shadow_sampling_mode_impl(&self, shadowmap: &mut RenderTargetIdentifier, mode: ShadowSamplingMode) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IssuePluginEventInternal(System.IntPtr,System.Int32)")]
    pub fn issue_plugin_event_internal(&self, callback: isize, event_id: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::BeginSample(System.String)")]
    pub fn begin_sample(&self, name: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EndSample(System.String)")]
    pub fn end_sample(&self, name: &str) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::BeginSample_CustomSampler(CustomSampler)")]
    pub fn begin_sample_1(&self, sampler: Option<CustomSampler>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EndSample_CustomSampler(CustomSampler)")]
    pub fn end_sample_1(&self, sampler: Option<CustomSampler>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IssuePluginEventAndDataInternal(System.IntPtr,System.Int32,System.IntPtr)")]
    pub fn issue_plugin_event_and_data_internal(&self, callback: isize, event_id: i32, data: isize) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IssuePluginCustomBlitInternal(System.IntPtr,System.UInt32,RenderTargetIdentifier&,RenderTargetIdentifier&,System.UInt32,System.UInt32)")]
    pub fn issue_plugin_custom_blit_internal(&self, callback: isize, command: u32, source: &mut RenderTargetIdentifier, dest: &mut RenderTargetIdentifier, command_param: u32, command_flags: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IncrementUpdateCount(RenderTargetIdentifier)")]
    pub fn increment_update_count(&self, dest: RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetInstanceMultiplier(System.UInt32)")]
    pub fn set_instance_multiplier(&self, multiplier: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetWireframe(System.Boolean)")]
    pub fn set_wireframe(&self, enable: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_ProcessVTFeedback(RenderTargetIdentifier,System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_process_vt_feedback(&self, rt: RenderTargetIdentifier, resolver: isize, slice: i32, x: i32, width: i32, y: i32, height: i32, mip: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetComputeBufferNativeData(ComputeBuffer,System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_buffer_data(&self, buffer: Option<ComputeBuffer>, data: isize, native_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetComputeBufferCounterValue(ComputeBuffer,System.UInt32)")]
    pub fn set_buffer_counter_value(&self, buffer: Option<ComputeBuffer>, counter_value: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetGraphicsBufferNativeData(GraphicsBuffer,System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_buffer_data_1(&self, buffer: Option<GraphicsBuffer>, data: isize, native_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetGraphicsBufferCounterValue(GraphicsBuffer,System.UInt32)")]
    pub fn set_buffer_counter_value_1(&self, buffer: Option<GraphicsBuffer>, counter_value: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::InternalSetGraphicsBufferData(GraphicsBuffer,System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_set_graphics_buffer_data(&self, buffer: Option<GraphicsBuffer>, data: Option<SystemArray>, managed_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ConvertTexture_Internal_Injected(RenderTargetIdentifier&,System.Int32,RenderTargetIdentifier&,System.Int32)")]
    pub fn convert_texture_internal(&self, src: &mut RenderTargetIdentifier, src_element: i32, dst: &mut RenderTargetIdentifier, dst_element: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeVectorParam_Injected(ComputeShader,System.Int32,Vector4&)")]
    pub fn set_compute_vector_param_1(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: &mut Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeMatrixParam_Injected(ComputeShader,System.Int32,Matrix4x4&)")]
    pub fn set_compute_matrix_param_1(&self, compute_shader: Option<ComputeShader>, name_id: i32, val: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingVectorParam_Injected(RayTracingShader,System.Int32,Vector4&)")]
    pub fn internal_set_ray_tracing_vector_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: &mut Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_SetRayTracingMatrixParam_Injected(RayTracingShader,System.Int32,Matrix4x4&)")]
    pub fn internal_set_ray_tracing_matrix_param(&self, ray_tracing_shader: Option<RayTracingShader>, name_id: i32, val: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_BuildRayTracingAccelerationStructure_Injected(RayTracingAccelerationStructure,Vector3&)")]
    pub fn internal_build_ray_tracing_acceleration_structure(&self, acceleration_structure: Option<RayTracingAccelerationStructure>, relative_origin: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_GenerateMips_Injected(RenderTargetIdentifier&)")]
    pub fn internal_generate_mips_1(&self, rt: &mut RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawMesh_Injected(Mesh,Matrix4x4&,Material,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_mesh_1(&self, mesh: Option<Mesh>, matrix: &mut Matrix4x4, material: Option<Material>, submesh_index: i32, shader_pass: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawRendererList_Injected(RendererList&)")]
    pub fn internal_draw_renderer_list(&self, renderer_list: &mut RendererList) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProcedural_Injected(Matrix4x4&,Material,System.Int32,MeshTopology,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_1(&self, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, vertex_count: i32, instance_count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexed_Injected(GraphicsBuffer,Matrix4x4&,Material,System.Int32,MeshTopology,System.Int32,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indexed(&self, index_buffer: Option<GraphicsBuffer>, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, index_count: i32, instance_count: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndirect_Injected(Matrix4x4&,Material,System.Int32,MeshTopology,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indirect_1(&self, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirect_Injected(GraphicsBuffer,Matrix4x4&,Material,System.Int32,MeshTopology,ComputeBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indexed_indirect(&self, index_buffer: Option<GraphicsBuffer>, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<ComputeBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndirectGraphicsBuffer_Injected(Matrix4x4&,Material,System.Int32,MeshTopology,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indirect_graphics_buffer_1(&self, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawProceduralIndexedIndirectGraphicsBuffer_Injected(GraphicsBuffer,Matrix4x4&,Material,System.Int32,MeshTopology,GraphicsBuffer,System.Int32,MaterialPropertyBlock)")]
    pub fn internal_draw_procedural_indexed_indirect_graphics_buffer(&self, index_buffer: Option<GraphicsBuffer>, matrix: &mut Matrix4x4, material: Option<Material>, shader_pass: i32, topology: MeshTopology, buffer_with_args: Option<GraphicsBuffer>, args_offset: i32, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_DrawOcclusionMesh_Injected(RectInt&)")]
    pub fn internal_draw_occlusion_mesh(&self, normalized_cam_viewport: &mut RectInt) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewport_Injected(Rect&)")]
    pub fn set_viewport_1(&self, pixel_rect: &mut Rect) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableScissorRect_Injected(Rect&)")]
    pub fn enable_scissor_rect_1(&self, scissor: &mut Rect) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Blit_Texture_Injected(Texture,RenderTargetIdentifier&,Material,System.Int32,Vector2&,Vector2&,System.Int32,System.Int32)")]
    pub fn blit_texture_1(&self, source: Option<Texture>, dest: &mut RenderTargetIdentifier, mat: Option<Material>, pass: i32, scale: &mut Vector2, offset: &mut Vector2, source_depth_slice: i32, dest_depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Blit_Identifier_Injected(RenderTargetIdentifier&,RenderTargetIdentifier&,Material,System.Int32,Vector2&,Vector2&,System.Int32,System.Int32)")]
    pub fn blit_identifier_1(&self, source: &mut RenderTargetIdentifier, dest: &mut RenderTargetIdentifier, mat: Option<Material>, pass: i32, scale: &mut Vector2, offset: &mut Vector2, source_depth_slice: i32, dest_depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::GetTemporaryRTWithDescriptor_Injected(System.Int32,RenderTextureDescriptor&,FilterMode)")]
    pub fn get_temporary_rt_with_descriptor(&self, name_id: i32, desc: &mut RenderTextureDescriptor, filter: FilterMode) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::ClearRenderTarget_Injected(RTClearFlags,Color&,System.Single,System.UInt32)")]
    pub fn clear_render_target_1(&self, clear_flags: RTClearFlags, background_color: &mut Color, depth: f32, stencil: u32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalVector_Injected(System.Int32,Vector4&)")]
    pub fn set_global_vector_1(&self, name_id: i32, value: &mut Vector4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalColor_Injected(System.Int32,Color&)")]
    pub fn set_global_color_1(&self, name_id: i32, value: &mut Color) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalMatrix_Injected(System.Int32,Matrix4x4&)")]
    pub fn set_global_matrix_1(&self, name_id: i32, value: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableGlobalKeyword_Injected(GlobalKeyword&)")]
    pub fn enable_global_keyword_1(&self, keyword: &mut GlobalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableMaterialKeyword_Injected(Material,LocalKeyword&)")]
    pub fn enable_material_keyword_1(&self, material: Option<Material>, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::EnableComputeKeyword_Injected(ComputeShader,LocalKeyword&)")]
    pub fn enable_compute_keyword_1(&self, compute_shader: Option<ComputeShader>, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableGlobalKeyword_Injected(GlobalKeyword&)")]
    pub fn disable_global_keyword_1(&self, keyword: &mut GlobalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableMaterialKeyword_Injected(Material,LocalKeyword&)")]
    pub fn disable_material_keyword_1(&self, material: Option<Material>, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::DisableComputeKeyword_Injected(ComputeShader,LocalKeyword&)")]
    pub fn disable_compute_keyword_1(&self, compute_shader: Option<ComputeShader>, keyword: &mut LocalKeyword) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetGlobalKeyword_Injected(GlobalKeyword&,System.Boolean)")]
    pub fn set_global_keyword_1(&self, keyword: &mut GlobalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetMaterialKeyword_Injected(Material,LocalKeyword&,System.Boolean)")]
    pub fn set_material_keyword_1(&self, material: Option<Material>, keyword: &mut LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetComputeKeyword_Injected(ComputeShader,LocalKeyword&,System.Boolean)")]
    pub fn set_compute_keyword_1(&self, compute_shader: Option<ComputeShader>, keyword: &mut LocalKeyword, value: bool) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewMatrix_Injected(Matrix4x4&)")]
    pub fn set_view_matrix_1(&self, view: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetProjectionMatrix_Injected(Matrix4x4&)")]
    pub fn set_projection_matrix_1(&self, proj: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetViewProjectionMatrices_Injected(Matrix4x4&,Matrix4x4&)")]
    pub fn set_view_projection_matrices_1(&self, view: &mut Matrix4x4, proj: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::IncrementUpdateCount_Injected(RenderTargetIdentifier&)")]
    pub fn increment_update_count_1(&self, dest: &mut RenderTargetIdentifier) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRenderTargetSingle_Internal_Injected(RenderTargetIdentifier&,RenderBufferLoadAction,RenderBufferStoreAction,RenderBufferLoadAction,RenderBufferStoreAction)")]
    pub fn set_render_target_single_internal(&self, rt: &mut RenderTargetIdentifier, color_load_action: RenderBufferLoadAction, color_store_action: RenderBufferStoreAction, depth_load_action: RenderBufferLoadAction, depth_store_action: RenderBufferStoreAction) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRenderTargetColorDepth_Internal_Injected(RenderTargetIdentifier&,RenderTargetIdentifier&,RenderBufferLoadAction,RenderBufferStoreAction,RenderBufferLoadAction,RenderBufferStoreAction,RenderTargetFlags)")]
    pub fn set_render_target_color_depth_internal(&self, color: &mut RenderTargetIdentifier, depth: &mut RenderTargetIdentifier, color_load_action: RenderBufferLoadAction, color_store_action: RenderBufferStoreAction, depth_load_action: RenderBufferLoadAction, depth_store_action: RenderBufferStoreAction, flags: RenderTargetFlags) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRenderTargetMulti_Internal_Injected(RenderTargetIdentifier[],RenderTargetIdentifier&,RenderBufferLoadAction[],RenderBufferStoreAction[],RenderBufferLoadAction,RenderBufferStoreAction,RenderTargetFlags)")]
    pub fn set_render_target_multi_internal(&self, colors: Array<RenderTargetIdentifier>, depth: &mut RenderTargetIdentifier, color_load_actions: Array<RenderBufferLoadAction>, color_store_actions: Array<RenderBufferStoreAction>, depth_load_action: RenderBufferLoadAction, depth_store_action: RenderBufferStoreAction, flags: RenderTargetFlags) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRenderTargetColorDepthSubtarget_Injected(RenderTargetIdentifier&,RenderTargetIdentifier&,RenderBufferLoadAction,RenderBufferStoreAction,RenderBufferLoadAction,RenderBufferStoreAction,System.Int32,CubemapFace,System.Int32)")]
    pub fn set_render_target_color_depth_subtarget(&self, color: &mut RenderTargetIdentifier, depth: &mut RenderTargetIdentifier, color_load_action: RenderBufferLoadAction, color_store_action: RenderBufferStoreAction, depth_load_action: RenderBufferLoadAction, depth_store_action: RenderBufferStoreAction, mip_level: i32, cubemap_face: CubemapFace, depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::SetRenderTargetMultiSubtarget_Injected(RenderTargetIdentifier[],RenderTargetIdentifier&,RenderBufferLoadAction[],RenderBufferStoreAction[],RenderBufferLoadAction,RenderBufferStoreAction,System.Int32,CubemapFace,System.Int32)")]
    pub fn set_render_target_multi_subtarget(&self, colors: Array<RenderTargetIdentifier>, depth: &mut RenderTargetIdentifier, color_load_actions: Array<RenderBufferLoadAction>, color_store_actions: Array<RenderBufferStoreAction>, depth_load_action: RenderBufferLoadAction, depth_store_action: RenderBufferStoreAction, mip_level: i32, cubemap_face: CubemapFace, depth_slice: i32) {}

    #[unity_icall("UnityEngine.Rendering.CommandBuffer::Internal_ProcessVTFeedback_Injected(RenderTargetIdentifier&,System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_process_vt_feedback_1(&self, rt: &mut RenderTargetIdentifier, resolver: isize, slice: i32, x: i32, width: i32, y: i32, height: i32, mip: i32) {}

}
