#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::load_scene_mode::LoadSceneMode;
use super::local_physics_mode::LocalPhysicsMode;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "LoadSceneParameters", namespace = "UnityEngine.SceneManagement", value_type)]
pub struct LoadSceneParameters {
    pub m_load_scene_mode: LoadSceneMode,
    pub m_local_physics_mode: LocalPhysicsMode,
}

#[unity_impl]
impl LoadSceneParameters {
    #[unity_method(name = "get_loadSceneMode")]
    pub fn get_load_scene_mode(&self) -> LoadSceneMode {}

    #[unity_method(name = "set_loadSceneMode")]
    pub fn set_load_scene_mode(&self, value: LoadSceneMode) {}

    #[unity_method(name = "get_localPhysicsMode")]
    pub fn get_local_physics_mode(&self) -> LocalPhysicsMode {}

    #[unity_method(name = "set_localPhysicsMode")]
    pub fn set_local_physics_mode(&self, value: LocalPhysicsMode) {}

}
