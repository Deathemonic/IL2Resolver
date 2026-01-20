#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString, SystemType};
use crate::mscorlib::collections::{Array};
use crate::mscorlib::io::{Stream};
use super::asset_bundle_create_request::AssetBundleCreateRequest;
use super::asset_bundle_recompress_operation::AssetBundleRecompressOperation;
use super::asset_bundle_request::AssetBundleRequest;
use super::build_compression::BuildCompression;
use crate::core_module::{AsyncOperation, ThreadPriority};
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "AssetBundle", namespace = "UnityEngine", inherit = "Object")]
pub struct AssetBundle(pub *mut c_void);

#[unity_impl]
impl AssetBundle {
    #[unity_method(name = "get_mainAsset")]
    pub fn get_main_asset(&self) -> Option<Object> {}

    #[unity_icall("UnityEngine.AssetBundle::get_isStreamedSceneAssetBundle")]
    pub fn get_is_streamed_scene_asset_bundle(&self) -> bool {}

    #[unity_method(name = "get_memoryBudgetKB", static)]
    pub fn get_memory_budget_kb() -> u32 {}

    #[unity_method(name = "set_memoryBudgetKB", static)]
    pub fn set_memory_budget_kb(value: u32) {}

    #[unity_icall("UnityEngine.AssetBundle::returnMainAsset(AssetBundle)")]
    pub fn return_main_asset(bundle: Option<AssetBundle>) -> Option<Object> {}

    #[unity_icall("UnityEngine.AssetBundle::UnloadAllAssetBundles(System.Boolean)")]
    pub fn unload_all_asset_bundles(unload_all_objects: bool) {}

    #[unity_icall("UnityEngine.AssetBundle::GetAllLoadedAssetBundles_Native")]
    pub fn get_all_loaded_asset_bundles_native() -> Array<AssetBundle> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromFileAsync_Internal(System.String,System.UInt32,System.UInt64)")]
    pub fn load_from_file_async_internal(path: &str, crc: u32, offset: u64) -> Option<AssetBundleCreateRequest> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromFile_Internal(System.String,System.UInt32,System.UInt64)")]
    pub fn load_from_file_internal(path: &str, crc: u32, offset: u64) -> Option<AssetBundle> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromMemoryAsync_Internal(System.Byte[],System.UInt32)")]
    pub fn load_from_memory_async_internal(binary: Array<u8>, crc: u32) -> Option<AssetBundleCreateRequest> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromMemory_Internal(System.Byte[],System.UInt32)")]
    pub fn load_from_memory_internal(binary: Array<u8>, crc: u32) -> Option<AssetBundle> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromStreamAsyncInternal(System.IO.Stream,System.UInt32,System.UInt32)")]
    pub fn load_from_stream_async_internal(stream: Option<Stream>, crc: u32, managed_read_buffer_size: u32) -> Option<AssetBundleCreateRequest> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadFromStreamInternal(System.IO.Stream,System.UInt32,System.UInt32)")]
    pub fn load_from_stream_internal(stream: Option<Stream>, crc: u32, managed_read_buffer_size: u32) -> Option<AssetBundle> {}

    #[unity_icall("UnityEngine.AssetBundle::Contains(System.String)")]
    pub fn contains(&self, name: &str) -> bool {}

    #[unity_method(name = "Load")]
    pub fn load(&self, name: &str) -> Option<Object> {}

    #[unity_method(name = "LoadAll")]
    pub fn load_all(&self) -> Array<Object> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadAsset_Internal(System.String,System.Type)")]
    pub fn load_asset_internal(&self, name: &str, type_ref: Option<SystemType>) -> Option<Object> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadAssetAsync_Internal(System.String,System.Type)")]
    pub fn load_asset_async_internal(&self, name: &str, type_ref: Option<SystemType>) -> Option<AssetBundleRequest> {}

    #[unity_icall("UnityEngine.AssetBundle::Unload(System.Boolean)")]
    pub fn unload(&self, unload_all_loaded_objects: bool) {}

    #[unity_icall("UnityEngine.AssetBundle::UnloadAsync(System.Boolean)")]
    pub fn unload_async(&self, unload_all_loaded_objects: bool) -> Option<AsyncOperation> {}

    #[unity_icall("UnityEngine.AssetBundle::GetAllAssetNames")]
    pub fn get_all_asset_names(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundle::GetAllScenePaths")]
    pub fn get_all_scene_paths(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadAssetWithSubAssets_Internal(System.String,System.Type)")]
    pub fn load_asset_with_sub_assets_internal(&self, name: &str, type_ref: Option<SystemType>) -> Array<Object> {}

    #[unity_icall("UnityEngine.AssetBundle::LoadAssetWithSubAssetsAsync_Internal(System.String,System.Type)")]
    pub fn load_asset_with_sub_assets_async_internal(&self, name: &str, type_ref: Option<SystemType>) -> Option<AssetBundleRequest> {}

    #[unity_icall("UnityEngine.AssetBundle::RecompressAssetBundleAsync_Internal(System.String,System.String,BuildCompression,System.UInt32,ThreadPriority)")]
    pub fn recompress_asset_bundle_async_internal(input_path: &str, output_path: &str, method: BuildCompression, expected_crc: u32, priority: ThreadPriority) -> Option<AssetBundleRecompressOperation> {}

    #[unity_icall("UnityEngine.AssetBundle::RecompressAssetBundleAsync_Internal_Injected(System.String,System.String,BuildCompression&,System.UInt32,ThreadPriority)")]
    pub fn recompress_asset_bundle_async_internal_1(input_path: &str, output_path: &str, method: &mut BuildCompression, expected_crc: u32, priority: ThreadPriority) -> Option<AssetBundleRecompressOperation> {}

}
