#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{LayoutGroup, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "HorizontalOrVerticalLayoutGroup", namespace = "UnityEngine.UI", inherit = "LayoutGroup,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct HorizontalOrVerticalLayoutGroup(pub *mut c_void);

#[unity_impl]
impl HorizontalOrVerticalLayoutGroup {
    #[unity_method(name = "get_spacing")]
    pub fn get_spacing(&self) -> f32 {}

    #[unity_method(name = "set_spacing")]
    pub fn set_spacing(&self, value: f32) {}

    #[unity_method(name = "get_childForceExpandWidth")]
    pub fn get_child_force_expand_width(&self) -> bool {}

    #[unity_method(name = "set_childForceExpandWidth")]
    pub fn set_child_force_expand_width(&self, value: bool) {}

    #[unity_method(name = "get_childForceExpandHeight")]
    pub fn get_child_force_expand_height(&self) -> bool {}

    #[unity_method(name = "set_childForceExpandHeight")]
    pub fn set_child_force_expand_height(&self, value: bool) {}

    #[unity_method(name = "get_childControlWidth")]
    pub fn get_child_control_width(&self) -> bool {}

    #[unity_method(name = "set_childControlWidth")]
    pub fn set_child_control_width(&self, value: bool) {}

    #[unity_method(name = "get_childControlHeight")]
    pub fn get_child_control_height(&self) -> bool {}

    #[unity_method(name = "set_childControlHeight")]
    pub fn set_child_control_height(&self, value: bool) {}

    #[unity_method(name = "get_childScaleWidth")]
    pub fn get_child_scale_width(&self) -> bool {}

    #[unity_method(name = "set_childScaleWidth")]
    pub fn set_child_scale_width(&self, value: bool) {}

    #[unity_method(name = "get_childScaleHeight")]
    pub fn get_child_scale_height(&self) -> bool {}

    #[unity_method(name = "set_childScaleHeight")]
    pub fn set_child_scale_height(&self, value: bool) {}

    #[unity_method(name = "get_reverseArrangement")]
    pub fn get_reverse_arrangement(&self) -> bool {}

    #[unity_method(name = "set_reverseArrangement")]
    pub fn set_reverse_arrangement(&self, value: bool) {}

}
