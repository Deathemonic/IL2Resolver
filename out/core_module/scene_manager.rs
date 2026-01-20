#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::async_operation::AsyncOperation;
use super::create_scene_parameters::CreateSceneParameters;
use super::game_object::GameObject;
use super::load_scene_mode::LoadSceneMode;
use super::load_scene_parameters::LoadSceneParameters;
use super::scene::Scene;
use super::unload_scene_options::UnloadSceneOptions;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "SceneManager", namespace = "UnityEngine.SceneManagement")]
pub struct SceneManager(pub *mut c_void);

#[unity_impl]
impl SceneManager {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::get_sceneCount")]
    pub fn get_scene_count() -> i32 {}

    #[unity_method(name = "get_sceneCountInBuildSettings", static)]
    pub fn get_scene_count_in_build_settings() -> i32 {}

    #[unity_method(name = "add_sceneLoaded", static)]
    pub fn add_scene_loaded(value: *mut c_void) {}

    #[unity_method(name = "remove_sceneLoaded", static)]
    pub fn remove_scene_loaded(value: *mut c_void) {}

    #[unity_method(name = "add_sceneUnloaded", static)]
    pub fn add_scene_unloaded(value: *mut c_void) {}

    #[unity_method(name = "remove_sceneUnloaded", static)]
    pub fn remove_scene_unloaded(value: *mut c_void) {}

    #[unity_method(name = "add_activeSceneChanged", static)]
    pub fn add_active_scene_changed(value: *mut c_void) {}

    #[unity_method(name = "remove_activeSceneChanged", static)]
    pub fn remove_active_scene_changed(value: *mut c_void) {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async(scene_build_index: i32, mode: LoadSceneMode) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async_1(scene_build_index: i32) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async_2(scene_build_index: i32, parameters: LoadSceneParameters) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async_3(scene_name: &str, mode: LoadSceneMode) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async_4(scene_name: &str) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadSceneAsync", static)]
    pub fn load_scene_async_5(scene_name: &str, parameters: LoadSceneParameters) -> Option<AsyncOperation> {}

    #[unity_method(name = "UnloadScene", static)]
    pub fn unload_scene(scene_build_index: i32) -> bool {}

    #[unity_method(name = "UnloadScene", static)]
    pub fn unload_scene_1(scene_name: &str) -> bool {}

    #[unity_method(name = "UnloadSceneAsync", static)]
    pub fn unload_scene_async(scene_build_index: i32) -> Option<AsyncOperation> {}

    #[unity_method(name = "UnloadSceneAsync", static)]
    pub fn unload_scene_async_1(scene_name: &str) -> Option<AsyncOperation> {}

    #[unity_method(name = "UnloadSceneAsync", static)]
    pub fn unload_scene_async_2(scene_build_index: i32, options: UnloadSceneOptions) -> Option<AsyncOperation> {}

    #[unity_method(name = "UnloadSceneAsync", static)]
    pub fn unload_scene_async_3(scene_name: &str, options: UnloadSceneOptions) -> Option<AsyncOperation> {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::SetActiveScene_Injected(Scene&)")]
    pub fn set_active_scene(scene: &mut Scene) -> bool {}

    #[unity_method(name = "GetSceneByBuildIndex", static)]
    pub fn get_scene_by_build_index(build_index: i32) -> Scene {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::MergeScenes_Injected(Scene&,Scene&)")]
    pub fn merge_scenes(source_scene: &mut Scene, destination_scene: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::MoveGameObjectToScene_Injected(GameObject,Scene&)")]
    pub fn move_game_object_to_scene(go: Option<GameObject>, scene: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::GetActiveScene_Injected(Scene&)")]
    pub fn get_active_scene(ret: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::GetSceneByPath_Injected(System.String,Scene&)")]
    pub fn get_scene_by_path(scene_path: &str, ret: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::GetSceneByName_Injected(System.String,Scene&)")]
    pub fn get_scene_by_name(name: &str, ret: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::GetSceneAt_Injected(System.Int32,Scene&)")]
    pub fn get_scene_at(index: i32, ret: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::CreateScene_Injected(System.String,CreateSceneParameters&,Scene&)")]
    pub fn create_scene(scene_name: &str, parameters: &mut CreateSceneParameters, ret: &mut Scene) {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::UnloadSceneInternal_Injected(Scene&,UnloadSceneOptions)")]
    pub fn unload_scene_internal(scene: &mut Scene, options: UnloadSceneOptions) -> bool {}

    #[unity_icall("UnityEngine.SceneManagement.SceneManager::UnloadSceneAsyncInternal_Injected(Scene&,UnloadSceneOptions)")]
    pub fn unload_scene_async_internal(scene: &mut Scene, options: UnloadSceneOptions) -> Option<AsyncOperation> {}

}
