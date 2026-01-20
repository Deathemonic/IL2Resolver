#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Behaviour", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct Behaviour(pub *mut c_void);

#[unity_impl]
impl Behaviour {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Behaviour::get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.Behaviour::set_enabled(System.Boolean)")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_icall("UnityEngine.Behaviour::get_isActiveAndEnabled")]
    pub fn get_is_active_and_enabled(&self) -> bool {}

}
