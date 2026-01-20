#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::local_physics_mode::LocalPhysicsMode;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "CreateSceneParameters", namespace = "UnityEngine.SceneManagement", value_type)]
pub struct CreateSceneParameters {
    pub m_local_physics_mode: LocalPhysicsMode,
}

#[unity_impl]
impl CreateSceneParameters {
    #[unity_method(name = "get_localPhysicsMode")]
    pub fn get_local_physics_mode(&self) -> LocalPhysicsMode {}

    #[unity_method(name = "set_localPhysicsMode")]
    pub fn set_local_physics_mode(&self, value: LocalPhysicsMode) {}

}
