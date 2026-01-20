#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{HorizontalOrVerticalLayoutGroup, LayoutGroup, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "VerticalLayoutGroup", namespace = "UnityEngine.UI", inherit = "HorizontalOrVerticalLayoutGroup,LayoutGroup,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct VerticalLayoutGroup(pub *mut c_void);

#[unity_impl]
impl VerticalLayoutGroup {
    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

}
