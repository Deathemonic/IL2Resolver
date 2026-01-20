#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Scene", namespace = "UnityEngine.SceneManagement", value_type)]
pub struct Scene {
    pub m_handle: i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadingState {
    #[default]
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Unloading = 3,
}

#[unity_impl]
impl Scene {
    #[unity_method(name = "get_handle")]
    pub fn get_handle(&self) -> i32 {}

    #[unity_method(name = "get_path")]
    pub fn get_path(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_name")]
    pub fn set_name(&self, value: &str) {}

    #[unity_method(name = "get_isLoaded")]
    pub fn get_is_loaded(&self) -> bool {}

    #[unity_method(name = "get_buildIndex")]
    pub fn get_build_index(&self) -> i32 {}

    #[unity_method(name = "get_isDirty")]
    pub fn get_is_dirty(&self) -> bool {}

    #[unity_method(name = "get_rootCount")]
    pub fn get_root_count(&self) -> i32 {}

    #[unity_method(name = "get_isSubScene")]
    pub fn get_is_sub_scene(&self) -> bool {}

    #[unity_method(name = "set_isSubScene")]
    pub fn set_is_sub_scene(&self, value: bool) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::IsValidInternal(System.Int32)")]
    pub fn is_valid_internal(scene_handle: i32) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetPathInternal(System.Int32)")]
    pub fn get_path_internal(scene_handle: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetNameInternal(System.Int32)")]
    pub fn get_name_internal(scene_handle: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::SetNameInternal(System.Int32,System.String)")]
    pub fn set_name_internal(scene_handle: i32, name: &str) {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetGUIDInternal(System.Int32)")]
    pub fn get_guid_internal(scene_handle: i32) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::IsSubScene(System.Int32)")]
    pub fn is_sub_scene(scene_handle: i32) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetIsLoadedInternal(System.Int32)")]
    pub fn get_is_loaded_internal(scene_handle: i32) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetLoadingStateInternal(System.Int32)")]
    pub fn get_loading_state_internal(scene_handle: i32) -> *mut c_void {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetIsDirtyInternal(System.Int32)")]
    pub fn get_is_dirty_internal(scene_handle: i32) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetDirtyID(System.Int32)")]
    pub fn get_dirty_id(scene_handle: i32) -> i32 {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetBuildIndexInternal(System.Int32)")]
    pub fn get_build_index_internal(scene_handle: i32) -> i32 {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetRootCountInternal(System.Int32)")]
    pub fn get_root_count_internal(scene_handle: i32) -> i32 {}

    #[unity_icall("UnityEngine.SceneManagement.Scene::GetRootGameObjectsInternal(System.Int32,System.Object)")]
    pub fn get_root_game_objects_internal(scene_handle: i32, result_root_list: Option<SystemObject>) {}

}
