#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::index_format::IndexFormat;
use super::mesh_update_flags::MeshUpdateFlags;
use super::sub_mesh_descriptor::SubMeshDescriptor;
use super::vertex_attribute::VertexAttribute;
use super::vertex_attribute_descriptor::VertexAttributeDescriptor;
use super::vertex_attribute_format::VertexAttributeFormat;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "MeshData", namespace = "UnityEngine", value_type)]
pub struct MeshData {
    pub m_ptr: isize,
}

#[unity_impl]
impl MeshData {
    #[unity_method(name = "get_vertexCount")]
    pub fn get_vertex_count(&self) -> i32 {}

    #[unity_method(name = "get_vertexBufferCount")]
    pub fn get_vertex_buffer_count(&self) -> i32 {}

    #[unity_method(name = "get_indexFormat")]
    pub fn get_index_format(&self) -> IndexFormat {}

    #[unity_method(name = "get_subMeshCount")]
    pub fn get_sub_mesh_count(&self) -> i32 {}

    #[unity_method(name = "set_subMeshCount")]
    pub fn set_sub_mesh_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.MeshData::HasVertexAttribute(System.IntPtr,VertexAttribute)")]
    pub fn has_vertex_attribute(this: isize, attr: VertexAttribute) -> bool {}

    #[unity_icall("UnityEngine.MeshData::GetVertexAttributeDimension(System.IntPtr,VertexAttribute)")]
    pub fn get_vertex_attribute_dimension(this: isize, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.MeshData::GetVertexAttributeFormat(System.IntPtr,VertexAttribute)")]
    pub fn get_vertex_attribute_format(this: isize, attr: VertexAttribute) -> VertexAttributeFormat {}

    #[unity_icall("UnityEngine.MeshData::GetVertexAttributeStream(System.IntPtr,VertexAttribute)")]
    pub fn get_vertex_attribute_stream(this: isize, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.MeshData::GetVertexAttributeOffset(System.IntPtr,VertexAttribute)")]
    pub fn get_vertex_attribute_offset(this: isize, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.MeshData::GetVertexDataPtr(System.IntPtr,System.Int32)")]
    pub fn get_vertex_data_ptr(this: isize, stream: i32) -> isize {}

    #[unity_icall("UnityEngine.MeshData::GetVertexDataSize(System.IntPtr,System.Int32)")]
    pub fn get_vertex_data_size(this: isize, stream: i32) -> u64 {}

    #[unity_icall("UnityEngine.MeshData::GetVertexBufferStride(System.IntPtr,System.Int32)")]
    pub fn get_vertex_buffer_stride(this: isize, stream: i32) -> i32 {}

    #[unity_icall("UnityEngine.MeshData::CopyAttributeIntoPtr(System.IntPtr,VertexAttribute,VertexAttributeFormat,System.Int32,System.IntPtr)")]
    pub fn copy_attribute_into_ptr(this: isize, attr: VertexAttribute, format: VertexAttributeFormat, dim: i32, dst: isize) {}

    #[unity_icall("UnityEngine.MeshData::CopyIndicesIntoPtr(System.IntPtr,System.Int32,System.Boolean,System.Int32,System.IntPtr)")]
    pub fn copy_indices_into_ptr(this: isize, submesh: i32, apply_base_vertex: bool, dst_stride: i32, dst: isize) {}

    #[unity_icall("UnityEngine.MeshData::GetIndexCount(System.IntPtr,System.Int32)")]
    pub fn get_index_count(this: isize, submesh: i32) -> i32 {}

    #[unity_icall("UnityEngine.MeshData::GetIndexDataPtr(System.IntPtr)")]
    pub fn get_index_data_ptr(this: isize) -> isize {}

    #[unity_icall("UnityEngine.MeshData::GetIndexDataSize(System.IntPtr)")]
    pub fn get_index_data_size(this: isize) -> u64 {}

    #[unity_icall("UnityEngine.MeshData::GetSubMesh(System.IntPtr,System.Int32)")]
    pub fn get_sub_mesh(this: isize, index: i32) -> SubMeshDescriptor {}

    #[unity_method(name = "GetVertices")]
    pub fn get_vertices(&self, out_vertices: *mut c_void) {}

    #[unity_method(name = "GetNormals")]
    pub fn get_normals(&self, out_normals: *mut c_void) {}

    #[unity_method(name = "GetTangents")]
    pub fn get_tangents(&self, out_tangents: *mut c_void) {}

    #[unity_method(name = "GetColors")]
    pub fn get_colors(&self, out_colors: *mut c_void) {}

    #[unity_method(name = "GetColors")]
    pub fn get_colors_1(&self, out_colors: *mut c_void) {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs(&self, channel: i32, out_u_vs: *mut c_void) {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs_1(&self, channel: i32, out_u_vs: *mut c_void) {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs_2(&self, channel: i32, out_u_vs: *mut c_void) {}

    #[unity_icall("UnityEngine.MeshData::SetVertexBufferParamsFromArray(System.IntPtr,System.Int32,VertexAttributeDescriptor[])")]
    pub fn set_vertex_buffer_params(this: isize, vertex_count: i32, attributes: Array<VertexAttributeDescriptor>) {}

    #[unity_icall("UnityEngine.MeshData::SetVertexBufferParamsFromPtr(System.IntPtr,System.Int32,System.IntPtr,System.Int32)")]
    pub fn set_vertex_buffer_params_1(this: isize, vertex_count: i32, attributes_ptr: isize, attributes_count: i32) {}

    #[unity_icall("UnityEngine.MeshData::SetIndexBufferParamsImpl(System.IntPtr,System.Int32,IndexFormat)")]
    pub fn set_index_buffer_params(this: isize, index_count: i32, index_format: IndexFormat) {}

    #[unity_icall("UnityEngine.MeshData::SetSubMeshImpl(System.IntPtr,System.Int32,SubMeshDescriptor,MeshUpdateFlags)")]
    pub fn set_sub_mesh(this: isize, index: i32, desc: SubMeshDescriptor, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.MeshData::GetSubMesh_Injected(System.IntPtr,System.Int32,SubMeshDescriptor&)")]
    pub fn get_sub_mesh_1(this: isize, index: i32, ret: &mut SubMeshDescriptor) {}

    #[unity_icall("UnityEngine.MeshData::SetSubMeshImpl_Injected(System.IntPtr,System.Int32,SubMeshDescriptor&,MeshUpdateFlags)")]
    pub fn set_sub_mesh_impl(this: isize, index: i32, desc: &mut SubMeshDescriptor, flags: MeshUpdateFlags) {}

}
