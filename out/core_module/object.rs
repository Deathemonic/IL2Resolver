#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::{SystemObject, SystemString, SystemType};
use crate::mscorlib::collections::{Array};
use super::find_objects_inactive::FindObjectsInactive;
use super::find_objects_sort_mode::FindObjectsSortMode;
use super::hide_flags::HideFlags;
use super::transform::Transform;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Object", namespace = "UnityEngine")]
pub struct Object(pub *mut c_void);

#[unity_impl]
impl Object {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_name")]
    pub fn set_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.Object::get_hideFlags")]
    pub fn get_hide_flags(&self) -> HideFlags {}

    #[unity_icall("UnityEngine.Object::set_hideFlags(HideFlags)")]
    pub fn set_hide_flags(&self, value: HideFlags) {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_icall("UnityEngine.Object::DestroyImmediate(Object,System.Boolean)")]
    pub fn destroy_immediate(obj: Option<Object>, allow_destroying_assets: bool) {}

    #[unity_icall("UnityEngine.Object::FindObjectsOfType(System.Type,System.Boolean)")]
    pub fn find_objects_of_type(type_ref: Option<SystemType>, include_inactive: bool) -> Array<Object> {}

    #[unity_icall("UnityEngine.Object::FindObjectsByType(System.Type,FindObjectsInactive,FindObjectsSortMode)")]
    pub fn find_objects_by_type(type_ref: Option<SystemType>, find_objects_inactive: FindObjectsInactive, sort_mode: FindObjectsSortMode) -> Array<Object> {}

    #[unity_icall("UnityEngine.Object::DontDestroyOnLoad(Object)")]
    pub fn dont_destroy_on_load(target: Option<Object>) {}

    #[unity_icall("UnityEngine.Object::Destroy(Object,System.Single)")]
    pub fn destroy_object(obj: Option<Object>, t: f32) {}

    #[unity_icall("UnityEngine.Object::FindObjectsOfTypeIncludingAssets(System.Type)")]
    pub fn find_objects_of_type_including_assets(type_ref: Option<SystemType>) -> Array<Object> {}

    #[unity_method(name = "FindObjectsOfTypeAll", static)]
    pub fn find_objects_of_type_all(type_ref: Option<SystemType>) -> Array<Object> {}

    #[unity_icall("UnityEngine.Object::GetOffsetOfInstanceIDInCPlusPlusObject")]
    pub fn get_offset_of_instance_id_in_c_plus_plus_object() -> i32 {}

    #[unity_icall("UnityEngine.Object::CurrentThreadIsMainThread")]
    pub fn current_thread_is_main_thread() -> bool {}

    #[unity_icall("UnityEngine.Object::Internal_CloneSingle(Object)")]
    pub fn internal_clone_single(data: Option<Object>) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::Internal_CloneSingleWithParent(Object,Transform,System.Boolean)")]
    pub fn internal_clone_single_with_parent(data: Option<Object>, parent: Option<Transform>, world_position_stays: bool) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::Internal_InstantiateSingle(Object,Vector3,Quaternion)")]
    pub fn internal_instantiate_single(data: Option<Object>, pos: Vector3, rot: Quaternion) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::Internal_InstantiateSingleWithParent(Object,Transform,Vector3,Quaternion)")]
    pub fn internal_instantiate_single_with_parent(data: Option<Object>, parent: Option<Transform>, pos: Vector3, rot: Quaternion) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::ToString(Object)")]
    pub fn to_string(obj: Option<Object>) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Object::IsPersistent(Object)")]
    pub fn is_persistent(obj: Option<Object>) -> bool {}

    #[unity_icall("UnityEngine.Object::DoesObjectWithInstanceIDExist(System.Int32)")]
    pub fn does_object_with_instance_id_exist(instance_id: i32) -> bool {}

    #[unity_icall("UnityEngine.Object::FindObjectFromInstanceID(System.Int32)")]
    pub fn find_object_from_instance_id(instance_id: i32) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::ForceLoadFromInstanceID(System.Int32)")]
    pub fn force_load_from_instance_id(instance_id: i32) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::Internal_InstantiateSingle_Injected(Object,Vector3&,Quaternion&)")]
    pub fn internal_instantiate_single_1(data: Option<Object>, pos: &mut Vector3, rot: &mut Quaternion) -> Option<Object> {}

    #[unity_icall("UnityEngine.Object::Internal_InstantiateSingleWithParent_Injected(Object,Transform,Vector3&,Quaternion&)")]
    pub fn internal_instantiate_single_with_parent_1(data: Option<Object>, parent: Option<Transform>, pos: &mut Vector3, rot: &mut Quaternion) -> Option<Object> {}

}
