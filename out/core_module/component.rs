#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString, SystemType};
use super::game_object::GameObject;
use super::send_message_options::SendMessageOptions;
use super::transform::Transform;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Component", namespace = "UnityEngine", inherit = "Object")]
pub struct Component(pub *mut c_void);

#[unity_impl]
impl Component {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Component::get_transform")]
    pub fn get_transform(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Component::get_gameObject")]
    pub fn get_game_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "get_tag")]
    pub fn get_tag(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_tag")]
    pub fn set_tag(&self, value: &str) {}

    #[unity_icall("UnityEngine.Component::GetComponentFastPath(System.Type,System.IntPtr)")]
    pub fn get_component_fast_path(&self, type_ref: Option<SystemType>, one_further_than_result_value: isize) {}

    #[unity_icall("UnityEngine.Component::GetComponent(System.String)")]
    pub fn get_component(&self, type_ref: &str) -> Option<Component> {}

    #[unity_icall("UnityEngine.Component::get_gameObject")]
    pub fn get_components_in_children(&self) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.Component::get_gameObject")]
    pub fn get_components_in_parent(&self) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.Component::GetComponentsForListInternal(System.Type,System.Object)")]
    pub fn get_components(&self, search_type: Option<SystemType>, result_list: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Component::SendMessageUpwards(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_upwards(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::SendMessageUpwards(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_upwards_1(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::SendMessageUpwards(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_upwards_2(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::SendMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::SendMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_1(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::SendMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn send_message_2(&self, method_name: &str, value: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::BroadcastMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn broadcast_message(&self, method_name: &str, parameter: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::BroadcastMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn broadcast_message_1(&self, method_name: &str, parameter: Option<SystemObject>, options: SendMessageOptions) {}

    #[unity_icall("UnityEngine.Component::BroadcastMessage(System.String,System.Object,SendMessageOptions)")]
    pub fn broadcast_message_2(&self, method_name: &str, parameter: Option<SystemObject>, options: SendMessageOptions) {}

}
