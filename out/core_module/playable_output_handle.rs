#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemType};
use crate::mscorlib::collections::{Array};
use super::object::Object;
use super::playable_handle::PlayableHandle;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PlayableOutputHandle", namespace = "UnityEngine.Playables", value_type)]
pub struct PlayableOutputHandle {
    pub m_handle: isize,
    pub m_version: u32,
}

#[unity_impl]
impl PlayableOutputHandle {
    #[unity_method(name = "get_Null", static)]
    pub fn get_null() -> PlayableOutputHandle {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, p: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetPlayableOutputType")]
    pub fn get_playable_output_type(&self) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetReferenceObject")]
    pub fn get_reference_object(&self) -> Option<Object> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetReferenceObject(Object)")]
    pub fn set_reference_object(&self, target: Option<Object>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetUserData")]
    pub fn get_user_data(&self) -> Option<Object> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetUserData(Object)")]
    pub fn set_user_data(&self, target: Option<Object>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetSourcePlayable")]
    pub fn get_source_playable(&self) -> PlayableHandle {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetSourcePlayable(PlayableHandle,System.Int32)")]
    pub fn set_source_playable(&self, target: PlayableHandle, port: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetSourceOutputPort")]
    pub fn get_source_output_port(&self) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetWeight")]
    pub fn get_weight(&self) -> f32 {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetWeight(System.Single)")]
    pub fn set_weight(&self, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::PushNotification(PlayableHandle,INotification,System.Object)")]
    pub fn push_notification(&self, origin: PlayableHandle, notification: *mut c_void, context: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetNotificationReceivers")]
    pub fn get_notification_receivers(&self) -> Array<*mut c_void> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::AddNotificationReceiver(INotificationReceiver)")]
    pub fn add_notification_receiver(&self, receiver: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::RemoveNotificationReceiver(INotificationReceiver)")]
    pub fn remove_notification_receiver(&self, receiver: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::IsNull_Injected(PlayableOutputHandle&)")]
    pub fn is_null(_unity_self: &mut PlayableOutputHandle) -> bool {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetPlayableOutputType_Injected(PlayableOutputHandle&)")]
    pub fn get_playable_output_type_1(_unity_self: &mut PlayableOutputHandle) -> Option<SystemType> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetReferenceObject_Injected(PlayableOutputHandle&)")]
    pub fn get_reference_object_1(_unity_self: &mut PlayableOutputHandle) -> Option<Object> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetReferenceObject_Injected(PlayableOutputHandle&,Object)")]
    pub fn set_reference_object_1(_unity_self: &mut PlayableOutputHandle, target: Option<Object>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetUserData_Injected(PlayableOutputHandle&)")]
    pub fn get_user_data_1(_unity_self: &mut PlayableOutputHandle) -> Option<Object> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetUserData_Injected(PlayableOutputHandle&,Object)")]
    pub fn set_user_data_1(_unity_self: &mut PlayableOutputHandle, target: Option<Object>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetSourcePlayable_Injected(PlayableOutputHandle&,PlayableHandle&)")]
    pub fn get_source_playable_1(_unity_self: &mut PlayableOutputHandle, ret: &mut PlayableHandle) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetSourcePlayable_Injected(PlayableOutputHandle&,PlayableHandle&,System.Int32)")]
    pub fn set_source_playable_1(_unity_self: &mut PlayableOutputHandle, target: &mut PlayableHandle, port: i32) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetSourceOutputPort_Injected(PlayableOutputHandle&)")]
    pub fn get_source_output_port_1(_unity_self: &mut PlayableOutputHandle) -> i32 {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetWeight_Injected(PlayableOutputHandle&)")]
    pub fn get_weight_1(_unity_self: &mut PlayableOutputHandle) -> f32 {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::SetWeight_Injected(PlayableOutputHandle&,System.Single)")]
    pub fn set_weight_1(_unity_self: &mut PlayableOutputHandle, weight: f32) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::PushNotification_Injected(PlayableOutputHandle&,PlayableHandle&,INotification,System.Object)")]
    pub fn push_notification_1(_unity_self: &mut PlayableOutputHandle, origin: &mut PlayableHandle, notification: *mut c_void, context: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::GetNotificationReceivers_Injected(PlayableOutputHandle&)")]
    pub fn get_notification_receivers_1(_unity_self: &mut PlayableOutputHandle) -> Array<*mut c_void> {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::AddNotificationReceiver_Injected(PlayableOutputHandle&,INotificationReceiver)")]
    pub fn add_notification_receiver_1(_unity_self: &mut PlayableOutputHandle, receiver: *mut c_void) {}

    #[unity_icall("UnityEngine.Playables.PlayableOutputHandle::RemoveNotificationReceiver_Injected(PlayableOutputHandle&,INotificationReceiver)")]
    pub fn remove_notification_receiver_1(_unity_self: &mut PlayableOutputHandle, receiver: *mut c_void) {}

}
