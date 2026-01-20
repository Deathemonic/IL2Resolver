#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::bounds::Bounds;
use super::mesh_topology::MeshTopology;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "SubMeshDescriptor", namespace = "UnityEngine.Rendering", value_type)]
pub struct SubMeshDescriptor {
    pub bounds: Bounds,
    pub topology: MeshTopology,
    pub index_start: i32,
    pub index_count: i32,
    pub base_vertex: i32,
    pub first_vertex: i32,
    pub vertex_count: i32,
}

#[unity_impl]
impl SubMeshDescriptor {
    #[unity_method(name = "get_bounds")]
    pub fn get_bounds(&self) -> Bounds {}

    #[unity_method(name = "set_bounds")]
    pub fn set_bounds(&self, value: Bounds) {}

    #[unity_method(name = "get_topology")]
    pub fn get_topology(&self) -> MeshTopology {}

    #[unity_method(name = "set_topology")]
    pub fn set_topology(&self, value: MeshTopology) {}

    #[unity_method(name = "get_indexStart")]
    pub fn get_index_start(&self) -> i32 {}

    #[unity_method(name = "set_indexStart")]
    pub fn set_index_start(&self, value: i32) {}

    #[unity_method(name = "get_indexCount")]
    pub fn get_index_count(&self) -> i32 {}

    #[unity_method(name = "set_indexCount")]
    pub fn set_index_count(&self, value: i32) {}

    #[unity_method(name = "get_baseVertex")]
    pub fn get_base_vertex(&self) -> i32 {}

    #[unity_method(name = "set_baseVertex")]
    pub fn set_base_vertex(&self, value: i32) {}

    #[unity_method(name = "get_firstVertex")]
    pub fn get_first_vertex(&self) -> i32 {}

    #[unity_method(name = "set_firstVertex")]
    pub fn set_first_vertex(&self, value: i32) {}

    #[unity_method(name = "get_vertexCount")]
    pub fn get_vertex_count(&self) -> i32 {}

    #[unity_method(name = "set_vertexCount")]
    pub fn set_vertex_count(&self, value: i32) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
