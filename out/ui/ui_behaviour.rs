#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "UIBehaviour", namespace = "UnityEngine.EventSystems", inherit = "MonoBehaviour,Behaviour,Component,Object")]
pub struct UIBehaviour(pub *mut c_void);

#[unity_impl]
impl UIBehaviour {
    #[unity_method(name = "IsActive")]
    pub fn is_active(&self) -> bool {}

    #[unity_method(name = "IsDestroyed")]
    pub fn is_destroyed(&self) -> bool {}

}
