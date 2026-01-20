#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemArray, SystemObject, SystemString, SystemType};
use crate::mscorlib::collections::{Array};
use super::component::Component;
use super::primitive_type::PrimitiveType;
use super::scene::Scene;
use super::send_message_options::SendMessageOptions;
use super::transform::Transform;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GameObject", namespace = "UnityEngine", component_host, inherit = "Object")]
pub struct GameObject(pub *mut c_void);

#[unity_impl]
impl GameObject {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(name: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(name: &str, components: Array<SystemType>) -> Option<Self> {}

    #[unity_icall("UnityEngine.GameObject::get_transform")]
    pub fn get_transform(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.GameObject::get_layer")]
    pub fn get_layer(&self) -> i32 {}

    #[unity_icall("UnityEngine.GameObject::set_layer(System.Int32)")]
    pub fn set_layer(&self, value: i32) {}

    #[unity_icall("UnityEngine.GameObject::get_active")]
    pub fn get_active(&self) -> bool {}

    #[unity_icall("UnityEngine.GameObject::set_active(System.Boolean)")]
    pub fn set_active(&self, value: bool) {}

    #[unity_icall("UnityEngine.GameObject::get_activeSelf")]
    pub fn get_active_self(&self) -> bool {}

    #[unity_icall("UnityEngine.GameObject::get_activeInHierarchy")]
    pub fn get_active_in_hierarchy(&self) -> bool {}

    #[unity_icall("UnityEngine.GameObject::get_isStatic")]
    pub fn get_is_static(&self) -> bool {}

    #[unity_icall("UnityEngine.GameObject::set_isStatic(System.Boolean)")]
    pub fn set_is_static(&self, value: bool) {}

    #[unity_icall("UnityEngine.GameObject::get_tag")]
    pub fn get_tag(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GameObject::set_tag(System.String)")]
    pub fn set_tag(&self, value: &str) {}

    #[unity_icall("UnityEngine.GameObject::get_scene")]
    pub fn get_scene(&self) -> Scene {}

    #[unity_icall("UnityEngine.GameObject::get_sceneCullingMask")]
    pub fn get_scene_culling_mask(&self) -> u64 {}

    #[unity_method(name = "get_gameObject")]
    pub fn get_game_object(&self) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.GameObject::CreatePrimitive(PrimitiveType)")]
    pub fn create_primitive(type_ref: PrimitiveType) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.GameObject::GetComponent(System.Type)")]
    pub fn get_component(&self, type_ref: Option<SystemType>) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::GetComponentFastPath(System.Type,System.IntPtr)")]
    pub fn get_component_fast_path(&self, type_ref: Option<SystemType>, one_further_than_result_value: isize) {}

    #[unity_icall("UnityEngine.GameObject::GetComponentByName(System.String)")]
    pub fn get_component_by_name(&self, type_ref: &str) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::GetComponentInChildren(System.Type,System.Boolean)")]
    pub fn get_component_in_children(&self, type_ref: Option<SystemType>, include_inactive: bool) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::GetComponentInParent(System.Type,System.Boolean)")]
    pub fn get_component_in_parent(&self, type_ref: Option<SystemType>, include_inactive: bool) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::GetComponentsInternal(System.Type,System.Boolean,System.Boolean,System.Boolean,System.Boolean,System.Object)")]
    pub fn get_components_internal(&self, type_ref: Option<SystemType>, use_search_type_as_array_return_type: bool, recursive: bool, include_inactive: bool, reverse: bool, result_list: Option<SystemObject>) -> Option<SystemArray> {}

    #[unity_icall("UnityEngine.GameObject::TryGetComponentInternal(System.Type)")]
    pub fn try_get_component_internal(&self, type_ref: Option<SystemType>) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::TryGetComponentFastPath(System.Type,System.IntPtr)")]
    pub fn try_get_component_fast_path(&self, type_ref: Option<SystemType>, one_further_than_result_value: isize) {}

    #[unity_icall("UnityEngine.GameObject::SendMessageUpwards(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_upwards(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.GameObject::SendMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.GameObject::BroadcastMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn broadcast_message(&self, method_name: &str, parameter: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.GameObject::AddComponentInternal(System.String)")]
    pub fn add_component_internal(&self, class_name: &str) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::Internal_AddComponentWithType(System.Type)")]
    pub fn internal_add_component_with_type(&self, component_type: Option<SystemType>) -> Option<Component> {}

    #[unity_icall("UnityEngine.GameObject::SetActiveRecursively(System.Boolean)")]
    pub fn set_active_recursively(&self, state: bool) {}

    #[unity_icall("UnityEngine.GameObject::CompareTag(System.String)")]
    pub fn compare_tag(&self, tag: &str) -> bool {}

    #[unity_icall("UnityEngine.GameObject::FindGameObjectWithTag(System.String)")]
    pub fn find_game_object_with_tag(tag: &str) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.GameObject::FindGameObjectsWithTag(System.String)")]
    pub fn find_game_objects_with_tag(tag: &str) -> Array<GameObject> {}

    #[unity_icall("UnityEngine.GameObject::Internal_CreateGameObject(GameObject,System.String)")]
    pub fn internal_create_game_object(this: Option<GameObject>, name: &str) {}

    #[unity_icall("UnityEngine.GameObject::Find(System.String)")]
    pub fn find(name: &str) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.GameObject::get_scene_Injected(Scene&)")]
    pub fn get_scene_1(&self, ret: &mut Scene) {}

}
