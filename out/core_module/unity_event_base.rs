#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString, SystemType};
use crate::mscorlib::collections::{Array};
use crate::mscorlib::reflection::{MethodInfo};
use super::object::Object;
use super::unity_event_call_state::UnityEventCallState;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "UnityEventBase", namespace = "UnityEngine.Events")]
pub struct UnityEventBase(pub *mut c_void);

#[unity_impl]
impl UnityEventBase {
    #[unity_method(name = "GetPersistentEventCount")]
    pub fn get_persistent_event_count(&self) -> i32 {}

    #[unity_method(name = "GetPersistentTarget")]
    pub fn get_persistent_target(&self, index: i32) -> Option<Object> {}

    #[unity_method(name = "GetPersistentMethodName")]
    pub fn get_persistent_method_name(&self, index: i32) -> Option<SystemString> {}

    #[unity_method(name = "SetPersistentListenerState")]
    pub fn set_persistent_listener_state(&self, index: i32, state: UnityEventCallState) {}

    #[unity_method(name = "GetPersistentListenerState")]
    pub fn get_persistent_listener_state(&self, index: i32) -> UnityEventCallState {}

    #[unity_method(name = "RemoveAllListeners")]
    pub fn remove_all_listeners(&self) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "GetValidMethodInfo", static)]
    pub fn get_valid_method_info(obj: Option<SystemObject>, function_name: &str, argument_types: Array<SystemType>) -> Option<MethodInfo> {}

    #[unity_method(name = "GetValidMethodInfo", static)]
    pub fn get_valid_method_info_1(object_type: Option<SystemType>, function_name: &str, argument_types: Array<SystemType>) -> Option<MethodInfo> {}

}
