#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemType};
use crate::mscorlib::collections::{Array, List};
use super::async_operation::AsyncOperation;
use super::object::Object;
use super::resource_request::ResourceRequest;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Resources", namespace = "UnityEngine")]
pub struct Resources(pub *mut c_void);

#[unity_impl]
impl Resources {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "FindObjectsOfTypeAll", static)]
    pub fn find_objects_of_type_all(type_ref: Option<SystemType>) -> Array<Object> {}

    #[unity_method(name = "Load", static)]
    pub fn load(path: &str) -> Option<Object> {}

    #[unity_method(name = "Load", static)]
    pub fn load_1(path: &str, system_type_instance: Option<SystemType>) -> Option<Object> {}

    #[unity_method(name = "LoadAsync", static)]
    pub fn load_async(path: &str) -> Option<ResourceRequest> {}

    #[unity_method(name = "LoadAsync", static)]
    pub fn load_async_1(path: &str, type_ref: Option<SystemType>) -> Option<ResourceRequest> {}

    #[unity_method(name = "LoadAll", static)]
    pub fn load_all(path: &str, system_type_instance: Option<SystemType>) -> Array<Object> {}

    #[unity_method(name = "LoadAll", static)]
    pub fn load_all_1(path: &str) -> Array<Object> {}

    #[unity_icall("UnityEngine.Resources::GetBuiltinResource(System.Type,System.String)")]
    pub fn get_builtin_resource(type_ref: Option<SystemType>, path: &str) -> Option<Object> {}

    #[unity_method(name = "UnloadAsset", static)]
    pub fn unload_asset(asset_to_unload: Option<Object>) {}

    #[unity_icall("UnityEngine.Resources::UnloadAssetImplResourceManager(Object)")]
    pub fn unload_asset_impl_resource_manager(asset_to_unload: Option<Object>) {}

    #[unity_icall("UnityEngine.Resources::UnloadUnusedAssets")]
    pub fn unload_unused_assets() -> Option<AsyncOperation> {}

    #[unity_icall("UnityEngine.Resources::InstanceIDToObject(System.Int32)")]
    pub fn instance_id_to_object(instance_id: i32) -> Option<Object> {}

    #[unity_icall("UnityEngine.Resources::InstanceIDToObjectList(System.IntPtr,System.Int32,List<Object>)")]
    pub fn instance_id_to_object_list(instance_i_ds: isize, instance_count: i32, objects: List<Object>) {}

}
