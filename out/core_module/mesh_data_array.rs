#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::mesh::Mesh;
use super::mesh_data::MeshData;
use super::mesh_update_flags::MeshUpdateFlags;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "MeshDataArray", namespace = "UnityEngine", value_type)]
pub struct MeshDataArray {
    pub m_ptrs: *mut isize,
    pub m_length: i32,
}

#[unity_impl]
impl MeshDataArray {
    #[unity_method(name = "get_Length")]
    pub fn get_length(&self) -> i32 {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> MeshData {}

    #[unity_icall("UnityEngine.MeshDataArray::AcquireReadOnlyMeshData(Mesh,System.IntPtr*)")]
    pub fn acquire_read_only_mesh_data(mesh: Option<Mesh>, datas: *mut isize) {}

    #[unity_icall("UnityEngine.MeshDataArray::AcquireReadOnlyMeshDatas(Mesh[],System.IntPtr*,System.Int32)")]
    pub fn acquire_read_only_mesh_datas(meshes: Array<Mesh>, datas: *mut isize, count: i32) {}

    #[unity_icall("UnityEngine.MeshDataArray::ReleaseMeshDatas(System.IntPtr*,System.Int32)")]
    pub fn release_mesh_datas(datas: *mut isize, count: i32) {}

    #[unity_icall("UnityEngine.MeshDataArray::CreateNewMeshDatas(System.IntPtr*,System.Int32)")]
    pub fn create_new_mesh_datas(datas: *mut isize, count: i32) {}

    #[unity_icall("UnityEngine.MeshDataArray::ApplyToMeshesImpl(Mesh[],System.IntPtr*,System.Int32,MeshUpdateFlags)")]
    pub fn apply_to_meshes_impl(meshes: Array<Mesh>, datas: *mut isize, count: i32, flags: MeshUpdateFlags) {}

    #[unity_icall("UnityEngine.MeshDataArray::ApplyToMeshImpl(Mesh,System.IntPtr,MeshUpdateFlags)")]
    pub fn apply_to_mesh_impl(mesh: Option<Mesh>, data: isize, flags: MeshUpdateFlags) {}

}
