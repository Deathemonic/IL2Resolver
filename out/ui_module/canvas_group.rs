#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UIModule", class = "CanvasGroup", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct CanvasGroup(pub *mut c_void);

#[unity_impl]
impl CanvasGroup {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.CanvasGroup::get_alpha")]
    pub fn get_alpha(&self) -> f32 {}

    #[unity_icall("UnityEngine.CanvasGroup::set_alpha(System.Single)")]
    pub fn set_alpha(&self, value: f32) {}

    #[unity_icall("UnityEngine.CanvasGroup::get_interactable")]
    pub fn get_interactable(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasGroup::set_interactable(System.Boolean)")]
    pub fn set_interactable(&self, value: bool) {}

    #[unity_icall("UnityEngine.CanvasGroup::get_blocksRaycasts")]
    pub fn get_blocks_raycasts(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasGroup::set_blocksRaycasts(System.Boolean)")]
    pub fn set_blocks_raycasts(&self, value: bool) {}

    #[unity_icall("UnityEngine.CanvasGroup::get_ignoreParentGroups")]
    pub fn get_ignore_parent_groups(&self) -> bool {}

    #[unity_icall("UnityEngine.CanvasGroup::set_ignoreParentGroups(System.Boolean)")]
    pub fn set_ignore_parent_groups(&self, value: bool) {}

}
