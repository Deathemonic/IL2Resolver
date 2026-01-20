#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};
use super::coroutine::Coroutine;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "MonoBehaviour", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct MonoBehaviour(pub *mut c_void);

#[unity_impl]
impl MonoBehaviour {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.MonoBehaviour::get_useGUILayout")]
    pub fn get_use_gui_layout(&self) -> bool {}

    #[unity_icall("UnityEngine.MonoBehaviour::set_useGUILayout(System.Boolean)")]
    pub fn set_use_gui_layout(&self, value: bool) {}

    #[unity_icall("UnityEngine.MonoBehaviour::Internal_CancelInvokeAll(MonoBehaviour)")]
    pub fn cancel_invoke(this: Option<MonoBehaviour>) {}

    #[unity_icall("UnityEngine.MonoBehaviour::InvokeDelayed(MonoBehaviour,System.String,System.Single,System.Single)")]
    pub fn invoke(this: Option<MonoBehaviour>, method_name: &str, time: f32, repeat_rate: f32) {}

    #[unity_icall("UnityEngine.MonoBehaviour::CancelInvoke(MonoBehaviour,System.String)")]
    pub fn cancel_invoke_1(this: Option<MonoBehaviour>, method_name: &str) {}

    #[unity_icall("UnityEngine.MonoBehaviour::StopCoroutine(System.String)")]
    pub fn stop_coroutine(&self, method_name: &str) {}

    #[unity_icall("UnityEngine.MonoBehaviour::StopAllCoroutines")]
    pub fn stop_all_coroutines(&self) {}

    #[unity_method(name = "print", static)]
    pub fn print(message: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.MonoBehaviour::Internal_IsInvokingAll(MonoBehaviour)")]
    pub fn internal_is_invoking_all(this: Option<MonoBehaviour>) -> bool {}

    #[unity_icall("UnityEngine.MonoBehaviour::IsInvoking(MonoBehaviour,System.String)")]
    pub fn is_invoking(this: Option<MonoBehaviour>, method_name: &str) -> bool {}

    #[unity_icall("UnityEngine.MonoBehaviour::IsObjectMonoBehaviour(Object)")]
    pub fn is_object_mono_behaviour(obj: Option<Object>) -> bool {}

    #[unity_icall("UnityEngine.MonoBehaviour::StartCoroutineManaged(System.String,System.Object)")]
    pub fn start_coroutine_managed(&self, method_name: &str, value: Option<SystemObject>) -> Option<Coroutine> {}

    #[unity_icall("UnityEngine.MonoBehaviour::StartCoroutineManaged2(System.Collections.IEnumerator)")]
    pub fn start_coroutine_managed2(&self, enumerator: *mut c_void) -> Option<Coroutine> {}

    #[unity_icall("UnityEngine.MonoBehaviour::StopCoroutineManaged(Coroutine)")]
    pub fn stop_coroutine_managed(&self, routine: Option<Coroutine>) {}

    #[unity_icall("UnityEngine.MonoBehaviour::StopCoroutineFromEnumeratorManaged(System.Collections.IEnumerator)")]
    pub fn stop_coroutine_from_enumerator_managed(&self, routine: *mut c_void) {}

    #[unity_icall("UnityEngine.MonoBehaviour::GetScriptClassName")]
    pub fn get_script_class_name(&self) -> Option<SystemString> {}

}
