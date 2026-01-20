#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector3, Vector4};
use crate::mscorlib::{SystemArray, SystemString};
use crate::mscorlib::collections::{Array, List};
use super::graphics_buffer;
use super::bone_weight::BoneWeight;
use super::bounds::Bounds;
use super::color::Color;
use super::color32::Color32;
use super::combine_instance::CombineInstance;
use super::graphics_buffer::GraphicsBuffer;
use super::index_format::IndexFormat;
use super::mesh_data_array::MeshDataArray;
use super::mesh_topology::MeshTopology;
use super::mesh_update_flags::MeshUpdateFlags;
use super::sub_mesh_descriptor::SubMeshDescriptor;
use super::vertex_attribute::VertexAttribute;
use super::vertex_attribute_descriptor::VertexAttributeDescriptor;
use super::vertex_attribute_format::VertexAttributeFormat;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Mesh", namespace = "UnityEngine", inherit = "Object")]
pub struct Mesh(pub *mut c_void);

#[unity_impl]
impl Mesh {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_vertices")]
    pub fn get_vertices(&self) -> Array<Vector3> {}

    #[unity_method(name = "set_vertices")]
    pub fn set_vertices(&self, value: Array<Vector3>) {}

    #[unity_method(name = "get_normals")]
    pub fn get_normals(&self) -> Array<Vector3> {}

    #[unity_method(name = "set_normals")]
    pub fn set_normals(&self, value: Array<Vector3>) {}

    #[unity_method(name = "get_tangents")]
    pub fn get_tangents(&self) -> Array<Vector4> {}

    #[unity_method(name = "set_tangents")]
    pub fn set_tangents(&self, value: Array<Vector4>) {}

    #[unity_method(name = "get_uv")]
    pub fn get_uv(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv")]
    pub fn set_uv(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv2")]
    pub fn get_uv2(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv2")]
    pub fn set_uv2(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv3")]
    pub fn get_uv3(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv3")]
    pub fn set_uv3(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv4")]
    pub fn get_uv4(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv4")]
    pub fn set_uv4(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv5")]
    pub fn get_uv5(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv5")]
    pub fn set_uv5(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv6")]
    pub fn get_uv6(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv6")]
    pub fn set_uv6(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv7")]
    pub fn get_uv7(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv7")]
    pub fn set_uv7(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_uv8")]
    pub fn get_uv8(&self) -> Array<Vector2> {}

    #[unity_method(name = "set_uv8")]
    pub fn set_uv8(&self, value: Array<Vector2>) {}

    #[unity_method(name = "get_colors")]
    pub fn get_colors(&self) -> Array<Color> {}

    #[unity_method(name = "set_colors")]
    pub fn set_colors(&self, value: Array<Color>) {}

    #[unity_method(name = "get_colors32")]
    pub fn get_colors32(&self) -> Array<Color32> {}

    #[unity_method(name = "set_colors32")]
    pub fn set_colors32(&self, value: Array<Color32>) {}

    #[unity_method(name = "get_vertexAttributeCount")]
    pub fn get_vertex_attribute_count(&self) -> i32 {}

    #[unity_method(name = "get_triangles")]
    pub fn get_triangles(&self) -> Array<i32> {}

    #[unity_method(name = "set_triangles")]
    pub fn set_triangles(&self, value: Array<i32>) {}

    #[unity_method(name = "get_boneWeights")]
    pub fn get_bone_weights(&self) -> Array<BoneWeight> {}

    #[unity_method(name = "set_boneWeights")]
    pub fn set_bone_weights(&self, value: Array<BoneWeight>) {}

    #[unity_icall("UnityEngine.Mesh::get_indexFormat")]
    pub fn get_index_format(&self) -> IndexFormat {}

    #[unity_icall("UnityEngine.Mesh::set_indexFormat(IndexFormat)")]
    pub fn set_index_format(&self, value: IndexFormat) {}

    #[unity_icall("UnityEngine.Mesh::get_vertexBufferCount")]
    pub fn get_vertex_buffer_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::get_vertexBufferTarget")]
    pub fn get_vertex_buffer_target(&self) -> graphics_buffer::Target {}

    #[unity_icall("UnityEngine.Mesh::set_vertexBufferTarget(GraphicsBuffer.Target)")]
    pub fn set_vertex_buffer_target(&self, value: graphics_buffer::Target) {}

    #[unity_icall("UnityEngine.Mesh::get_indexBufferTarget")]
    pub fn get_index_buffer_target(&self) -> graphics_buffer::Target {}

    #[unity_icall("UnityEngine.Mesh::set_indexBufferTarget(GraphicsBuffer.Target)")]
    pub fn set_index_buffer_target(&self, value: graphics_buffer::Target) {}

    #[unity_icall("UnityEngine.Mesh::get_blendShapeCount")]
    pub fn get_blend_shape_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::get_bindposes")]
    pub fn get_bindposes(&self) -> Array<Matrix4x4> {}

    #[unity_icall("UnityEngine.Mesh::set_bindposes(Matrix4x4[])")]
    pub fn set_bindposes(&self, value: Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Mesh::get_isReadable")]
    pub fn get_is_readable(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_vertexCount")]
    pub fn get_vertex_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::get_subMeshCount")]
    pub fn get_sub_mesh_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::set_subMeshCount(System.Int32)")]
    pub fn set_sub_mesh_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.Mesh::get_bounds_Injected(Bounds&)")]
    pub fn get_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Mesh::set_bounds_Injected(Bounds&)")]
    pub fn set_bounds(&self, value: &mut Bounds) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs(&self, channel: i32, uvs: List<Vector2>) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_1(&self, channel: i32, uvs: List<Vector3>) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_2(&self, channel: i32, uvs: List<Vector4>) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_3(&self, channel: i32, uvs: List<Vector2>, start: i32, length: i32) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_4(&self, channel: i32, uvs: List<Vector2>, start: i32, length: i32, flags: MeshUpdateFlags) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_5(&self, channel: i32, uvs: List<Vector3>, start: i32, length: i32) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_6(&self, channel: i32, uvs: List<Vector3>, start: i32, length: i32, flags: MeshUpdateFlags) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_7(&self, channel: i32, uvs: List<Vector4>, start: i32, length: i32) {}

    #[unity_method(name = "SetUVs")]
    pub fn set_u_vs_8(&self, channel: i32, uvs: List<Vector4>, start: i32, length: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_9(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_10(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_11(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_12(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_13(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_14(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_15(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_16(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_u_vs_17(&self) -> bool {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs(&self, channel: i32, uvs: List<Vector2>) {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs_1(&self, channel: i32, uvs: List<Vector3>) {}

    #[unity_method(name = "GetUVs")]
    pub fn get_u_vs_2(&self, channel: i32, uvs: List<Vector4>) {}

    #[unity_icall("UnityEngine.Mesh::SetVertexBufferParamsFromArray(System.Int32,VertexAttributeDescriptor[])")]
    pub fn set_vertex_buffer_params(&self, vertex_count: i32, attributes: Array<VertexAttributeDescriptor>) {}

    #[unity_icall("UnityEngine.Mesh::SetVertexBufferParamsFromPtr(System.Int32,System.IntPtr,System.Int32)")]
    pub fn set_vertex_buffer_params_1(&self, vertex_count: i32, attributes_ptr: isize, attributes_count: i32) {}

    #[unity_method(name = "AcquireReadOnlyMeshData", static)]
    pub fn acquire_read_only_mesh_data(mesh: Option<Mesh>) -> MeshDataArray {}

    #[unity_method(name = "AcquireReadOnlyMeshData", static)]
    pub fn acquire_read_only_mesh_data_1(meshes: Array<Mesh>) -> MeshDataArray {}

    #[unity_method(name = "AcquireReadOnlyMeshData", static)]
    pub fn acquire_read_only_mesh_data_2(meshes: List<Mesh>) -> MeshDataArray {}

    #[unity_method(name = "AllocateWritableMeshData", static)]
    pub fn allocate_writable_mesh_data(mesh_count: i32) -> MeshDataArray {}

    #[unity_method(name = "ApplyAndDisposeWritableMeshData", static)]
    pub fn apply_and_dispose_writable_mesh_data(data: MeshDataArray, mesh: Option<Mesh>, flags: MeshUpdateFlags) {}

    #[unity_method(name = "ApplyAndDisposeWritableMeshData", static)]
    pub fn apply_and_dispose_writable_mesh_data_1(data: MeshDataArray, meshes: Array<Mesh>, flags: MeshUpdateFlags) {}

    #[unity_method(name = "ApplyAndDisposeWritableMeshData", static)]
    pub fn apply_and_dispose_writable_mesh_data_2(data: MeshDataArray, meshes: List<Mesh>, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::get_subMeshCount")]
    pub fn get_indices(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices_1(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices_2(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices_3(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices_4(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn set_indices_5(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::SetAllSubMeshesAtOnceFromArray(SubMeshDescriptor[],System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn set_sub_meshes(&self, desc: Array<SubMeshDescriptor>, start: i32, count: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::ClearImpl(System.Boolean)")]
    pub fn clear(&self, keep_vertex_layout: bool) {}

    #[unity_icall("UnityEngine.Mesh::ClearImpl(System.Boolean)")]
    pub fn clear_1(&self, keep_vertex_layout: bool) {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn recalculate_bounds(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn recalculate_normals(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::get_canAccess")]
    pub fn recalculate_tangents(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::CombineMeshesImpl(CombineInstance[],System.Boolean,System.Boolean,System.Boolean)")]
    pub fn combine_meshes(&self, combine: Array<CombineInstance>, merge_sub_meshes: bool, use_matrices: bool, has_lightmap_data: bool) {}

    #[unity_icall("UnityEngine.Mesh::CombineMeshesImpl(CombineInstance[],System.Boolean,System.Boolean,System.Boolean)")]
    pub fn combine_meshes_1(&self, combine: Array<CombineInstance>, merge_sub_meshes: bool, use_matrices: bool, has_lightmap_data: bool) {}

    #[unity_icall("UnityEngine.Mesh::CombineMeshesImpl(CombineInstance[],System.Boolean,System.Boolean,System.Boolean)")]
    pub fn combine_meshes_2(&self, combine: Array<CombineInstance>, merge_sub_meshes: bool, use_matrices: bool, has_lightmap_data: bool) {}

    #[unity_icall("UnityEngine.Mesh::CombineMeshesImpl(CombineInstance[],System.Boolean,System.Boolean,System.Boolean)")]
    pub fn combine_meshes_3(&self, combine: Array<CombineInstance>, merge_sub_meshes: bool, use_matrices: bool, has_lightmap_data: bool) {}

    #[unity_icall("UnityEngine.Mesh::Internal_Create(Mesh)")]
    pub fn internal_create(mono: Option<Mesh>) {}

    #[unity_icall("UnityEngine.Mesh::FromInstanceID(System.Int32)")]
    pub fn from_instance_id(id: i32) -> Option<Mesh> {}

    #[unity_icall("UnityEngine.Mesh::GetTotalIndexCount")]
    pub fn get_total_index_count(&self) -> u32 {}

    #[unity_icall("UnityEngine.Mesh::SetIndexBufferParams(System.Int32,IndexFormat)")]
    pub fn set_index_buffer_params(&self, index_count: i32, format: IndexFormat) {}

    #[unity_icall("UnityEngine.Mesh::InternalSetIndexBufferData(System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn internal_set_index_buffer_data(&self, data: isize, data_start: i32, mesh_buffer_start: i32, count: i32, elem_size: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::InternalSetIndexBufferDataFromArray(System.Array,System.Int32,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn internal_set_index_buffer_data_from_array(&self, data: Option<SystemArray>, data_start: i32, mesh_buffer_start: i32, count: i32, elem_size: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::InternalSetVertexBufferData(System.Int32,System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn internal_set_vertex_buffer_data(&self, stream: i32, data: isize, data_start: i32, mesh_buffer_start: i32, count: i32, elem_size: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::InternalSetVertexBufferDataFromArray(System.Int32,System.Array,System.Int32,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn internal_set_vertex_buffer_data_from_array(&self, stream: i32, data: Option<SystemArray>, data_start: i32, mesh_buffer_start: i32, count: i32, elem_size: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributesAlloc")]
    pub fn get_vertex_attributes_alloc(&self) -> Option<SystemArray> {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributesArray(VertexAttributeDescriptor[])")]
    pub fn get_vertex_attributes_array(&self, attributes: Array<VertexAttributeDescriptor>) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributesList(List<VertexAttributeDescriptor>)")]
    pub fn get_vertex_attributes_list(&self, attributes: List<VertexAttributeDescriptor>) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributeCountImpl")]
    pub fn get_vertex_attribute_count_impl(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttribute(System.Int32)")]
    pub fn get_vertex_attribute(&self, index: i32) -> VertexAttributeDescriptor {}

    #[unity_icall("UnityEngine.Mesh::GetIndexStartImpl(System.Int32)")]
    pub fn get_index_start_impl(&self, submesh: i32) -> u32 {}

    #[unity_icall("UnityEngine.Mesh::GetIndexCountImpl(System.Int32)")]
    pub fn get_index_count_impl(&self, submesh: i32) -> u32 {}

    #[unity_icall("UnityEngine.Mesh::GetTrianglesCountImpl(System.Int32)")]
    pub fn get_triangles_count_impl(&self, submesh: i32) -> u32 {}

    #[unity_icall("UnityEngine.Mesh::GetBaseVertexImpl(System.Int32)")]
    pub fn get_base_vertex_impl(&self, submesh: i32) -> u32 {}

    #[unity_icall("UnityEngine.Mesh::GetTrianglesImpl(System.Int32,System.Boolean)")]
    pub fn get_triangles_impl(&self, submesh: i32, apply_base_vertex: bool) -> Array<i32> {}

    #[unity_icall("UnityEngine.Mesh::GetIndicesImpl(System.Int32,System.Boolean)")]
    pub fn get_indices_impl(&self, submesh: i32, apply_base_vertex: bool) -> Array<i32> {}

    #[unity_icall("UnityEngine.Mesh::SetIndicesImpl(System.Int32,MeshTopology,IndexFormat,System.Array,System.Int32,System.Int32,System.Boolean,System.Int32)")]
    pub fn set_indices_impl(&self, submesh: i32, topology: MeshTopology, indices_format: IndexFormat, indices: Option<SystemArray>, array_start: i32, array_size: i32, calculate_bounds: bool, base_vertex: i32) {}

    #[unity_icall("UnityEngine.Mesh::SetIndicesNativeArrayImpl(System.Int32,MeshTopology,IndexFormat,System.IntPtr,System.Int32,System.Int32,System.Boolean,System.Int32)")]
    pub fn set_indices_native_array_impl(&self, submesh: i32, topology: MeshTopology, indices_format: IndexFormat, indices: isize, array_start: i32, array_size: i32, calculate_bounds: bool, base_vertex: i32) {}

    #[unity_icall("UnityEngine.Mesh::GetTrianglesNonAllocImpl(System.Int32[],System.Int32,System.Boolean)")]
    pub fn get_triangles_non_alloc_impl(&self, values: &mut Array<i32>, submesh: i32, apply_base_vertex: bool) {}

    #[unity_icall("UnityEngine.Mesh::GetTrianglesNonAllocImpl16(System.UInt16[],System.Int32,System.Boolean)")]
    pub fn get_triangles_non_alloc_impl16(&self, values: &mut Array<u16>, submesh: i32, apply_base_vertex: bool) {}

    #[unity_icall("UnityEngine.Mesh::GetIndicesNonAllocImpl(System.Int32[],System.Int32,System.Boolean)")]
    pub fn get_indices_non_alloc_impl(&self, values: &mut Array<i32>, submesh: i32, apply_base_vertex: bool) {}

    #[unity_icall("UnityEngine.Mesh::GetIndicesNonAllocImpl16(System.UInt16[],System.Int32,System.Boolean)")]
    pub fn get_indices_non_alloc_impl16(&self, values: &mut Array<u16>, submesh: i32, apply_base_vertex: bool) {}

    #[unity_icall("UnityEngine.Mesh::PrintErrorCantAccessChannel(VertexAttribute)")]
    pub fn print_error_cant_access_channel(&self, ch: VertexAttribute) {}

    #[unity_icall("UnityEngine.Mesh::HasVertexAttribute(VertexAttribute)")]
    pub fn has_vertex_attribute(&self, attr: VertexAttribute) -> bool {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributeDimension(VertexAttribute)")]
    pub fn get_vertex_attribute_dimension(&self, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributeFormat(VertexAttribute)")]
    pub fn get_vertex_attribute_format(&self, attr: VertexAttribute) -> VertexAttributeFormat {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributeStream(VertexAttribute)")]
    pub fn get_vertex_attribute_stream(&self, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttributeOffset(VertexAttribute)")]
    pub fn get_vertex_attribute_offset(&self, attr: VertexAttribute) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::SetArrayForChannelImpl(VertexAttribute,VertexAttributeFormat,System.Int32,System.Array,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn set_array_for_channel_impl(&self, channel: VertexAttribute, format: VertexAttributeFormat, dim: i32, values: Option<SystemArray>, array_size: i32, values_start: i32, values_count: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::SetNativeArrayForChannelImpl(VertexAttribute,VertexAttributeFormat,System.Int32,System.IntPtr,System.Int32,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn set_native_array_for_channel_impl(&self, channel: VertexAttribute, format: VertexAttributeFormat, dim: i32, values: isize, array_size: i32, values_start: i32, values_count: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::GetAllocArrayFromChannelImpl(VertexAttribute,VertexAttributeFormat,System.Int32)")]
    pub fn get_alloc_array_from_channel_impl(&self, channel: VertexAttribute, format: VertexAttributeFormat, dim: i32) -> Option<SystemArray> {}

    #[unity_icall("UnityEngine.Mesh::GetArrayFromChannelImpl(VertexAttribute,VertexAttributeFormat,System.Int32,System.Array)")]
    pub fn get_array_from_channel_impl(&self, channel: VertexAttribute, format: VertexAttributeFormat, dim: i32, values: Option<SystemArray>) {}

    #[unity_icall("UnityEngine.Mesh::GetVertexBufferStride(System.Int32)")]
    pub fn get_vertex_buffer_stride(&self, stream: i32) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetNativeVertexBufferPtr(System.Int32)")]
    pub fn get_native_vertex_buffer_ptr(&self, index: i32) -> isize {}

    #[unity_icall("UnityEngine.Mesh::GetNativeIndexBufferPtr")]
    pub fn get_native_index_buffer_ptr(&self) -> isize {}

    #[unity_icall("UnityEngine.Mesh::GetVertexBufferImpl(System.Int32)")]
    pub fn get_vertex_buffer_impl(&self, index: i32) -> Option<GraphicsBuffer> {}

    #[unity_icall("UnityEngine.Mesh::GetIndexBufferImpl")]
    pub fn get_index_buffer_impl(&self) -> Option<GraphicsBuffer> {}

    #[unity_icall("UnityEngine.Mesh::ClearBlendShapes")]
    pub fn clear_blend_shapes(&self) {}

    #[unity_icall("UnityEngine.Mesh::GetBlendShapeName(System.Int32)")]
    pub fn get_blend_shape_name(&self, shape_index: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Mesh::GetBlendShapeIndex(System.String)")]
    pub fn get_blend_shape_index(&self, blend_shape_name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetBlendShapeFrameCount(System.Int32)")]
    pub fn get_blend_shape_frame_count(&self, shape_index: i32) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetBlendShapeFrameWeight(System.Int32,System.Int32)")]
    pub fn get_blend_shape_frame_weight(&self, shape_index: i32, frame_index: i32) -> f32 {}

    #[unity_icall("UnityEngine.Mesh::GetBlendShapeFrameVertices(System.Int32,System.Int32,Vector3[],Vector3[],Vector3[])")]
    pub fn get_blend_shape_frame_vertices(&self, shape_index: i32, frame_index: i32, delta_vertices: Array<Vector3>, delta_normals: Array<Vector3>, delta_tangents: Array<Vector3>) {}

    #[unity_icall("UnityEngine.Mesh::AddBlendShapeFrame(System.String,System.Single,Vector3[],Vector3[],Vector3[])")]
    pub fn add_blend_shape_frame(&self, shape_name: &str, frame_weight: f32, delta_vertices: Array<Vector3>, delta_normals: Array<Vector3>, delta_tangents: Array<Vector3>) {}

    #[unity_icall("UnityEngine.Mesh::HasBoneWeights")]
    pub fn has_bone_weights(&self) -> bool {}

    #[unity_icall("UnityEngine.Mesh::GetBoneWeightsImpl")]
    pub fn get_bone_weights_impl(&self) -> Array<BoneWeight> {}

    #[unity_icall("UnityEngine.Mesh::SetBoneWeightsImpl(BoneWeight[])")]
    pub fn set_bone_weights_impl(&self, weights: Array<BoneWeight>) {}

    #[unity_icall("UnityEngine.Mesh::GetAllBoneWeightsArraySize")]
    pub fn get_all_bone_weights_array_size(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetAllBoneWeightsArray")]
    pub fn get_all_bone_weights_array(&self) -> isize {}

    #[unity_icall("UnityEngine.Mesh::GetBonesPerVertexArray")]
    pub fn get_bones_per_vertex_array(&self) -> isize {}

    #[unity_icall("UnityEngine.Mesh::GetBindposeCount")]
    pub fn get_bindpose_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Mesh::GetBoneWeightsNonAllocImpl(BoneWeight[])")]
    pub fn get_bone_weights_non_alloc_impl(&self, values: &mut Array<BoneWeight>) {}

    #[unity_icall("UnityEngine.Mesh::GetBindposesNonAllocImpl(Matrix4x4[])")]
    pub fn get_bindposes_non_alloc_impl(&self, values: &mut Array<Matrix4x4>) {}

    #[unity_icall("UnityEngine.Mesh::SetSubMesh(System.Int32,SubMeshDescriptor,MeshUpdateFlags)")]
    pub fn set_sub_mesh(&self, index: i32, desc: SubMeshDescriptor, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::GetSubMesh(System.Int32)")]
    pub fn get_sub_mesh(&self, index: i32) -> SubMeshDescriptor {}

    #[unity_icall("UnityEngine.Mesh::SetAllSubMeshesAtOnceFromNativeArray(System.IntPtr,System.Int32,System.Int32,MeshUpdateFlags)")]
    pub fn set_all_sub_meshes_at_once_from_native_array(&self, desc: isize, start: i32, count: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::RecalculateBoundsImpl(MeshUpdateFlags)")]
    pub fn recalculate_bounds_impl(&self, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::RecalculateNormalsImpl(MeshUpdateFlags)")]
    pub fn recalculate_normals_impl(&self, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::RecalculateTangentsImpl(MeshUpdateFlags)")]
    pub fn recalculate_tangents_impl(&self, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::MarkDynamicImpl")]
    pub fn mark_dynamic_impl(&self) {}

    #[unity_icall("UnityEngine.Mesh::MarkModified")]
    pub fn mark_modified(&self) {}

    #[unity_icall("UnityEngine.Mesh::UploadMeshDataImpl(System.Boolean)")]
    pub fn upload_mesh_data_impl(&self, mark_no_longer_readable: bool) {}

    #[unity_icall("UnityEngine.Mesh::GetTopologyImpl(System.Int32)")]
    pub fn get_topology_impl(&self, submesh: i32) -> MeshTopology {}

    #[unity_icall("UnityEngine.Mesh::RecalculateUVDistributionMetricImpl(System.Int32,System.Single)")]
    pub fn recalculate_uv_distribution_metric_impl(&self, uv_set_index: i32, uv_area_threshold: f32) {}

    #[unity_icall("UnityEngine.Mesh::RecalculateUVDistributionMetricsImpl(System.Single)")]
    pub fn recalculate_uv_distribution_metrics_impl(&self, uv_area_threshold: f32) {}

    #[unity_icall("UnityEngine.Mesh::GetUVDistributionMetric(System.Int32)")]
    pub fn get_uv_distribution_metric(&self, uv_set_index: i32) -> f32 {}

    #[unity_icall("UnityEngine.Mesh::OptimizeImpl")]
    pub fn optimize_impl(&self) {}

    #[unity_icall("UnityEngine.Mesh::OptimizeIndexBuffersImpl")]
    pub fn optimize_index_buffers_impl(&self) {}

    #[unity_icall("UnityEngine.Mesh::OptimizeReorderVertexBufferImpl")]
    pub fn optimize_reorder_vertex_buffer_impl(&self) {}

    #[unity_icall("UnityEngine.Mesh::GetVertexAttribute_Injected(System.Int32,VertexAttributeDescriptor&)")]
    pub fn get_vertex_attribute_1(&self, index: i32, ret: &mut VertexAttributeDescriptor) {}

    #[unity_icall("UnityEngine.Mesh::SetSubMesh_Injected(System.Int32,SubMeshDescriptor&,MeshUpdateFlags)")]
    pub fn set_sub_mesh_1(&self, index: i32, desc: &mut SubMeshDescriptor, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.Mesh::GetSubMesh_Injected(System.Int32,SubMeshDescriptor&)")]
    pub fn get_sub_mesh_1(&self, index: i32, ret: &mut SubMeshDescriptor) {}

}
