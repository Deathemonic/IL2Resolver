#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2, Vector3};
use crate::mscorlib::{SystemString};
use super::base_raycaster::BaseRaycaster;
use crate::core_module::GameObject;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "RaycastResult", namespace = "UnityEngine.EventSystems", value_type)]
pub struct RaycastResult {
    pub m_game_object: Option<GameObject>,
    pub module: Option<BaseRaycaster>,
    pub distance: f32,
    pub index: f32,
    pub depth: i32,
    pub sorting_group_id: i32,
    pub sorting_group_order: i32,
    pub sorting_layer: i32,
    pub sorting_order: i32,
    pub world_position: Vector3,
    pub world_normal: Vector3,
    pub screen_position: Vector2,
    pub display_index: i32,
}

#[unity_impl]
impl RaycastResult {
    #[unity_method(name = "get_gameObject")]
    pub fn get_game_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_gameObject")]
    pub fn set_game_object(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "Clear")]
    pub fn clear(&self) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
