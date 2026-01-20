#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "ContentSizeFitter", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct ContentSizeFitter(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FitMode {
    #[default]
    Unconstrained = 0,
    MinSize = 1,
    PreferredSize = 2,
}

#[unity_impl]
impl ContentSizeFitter {
    #[unity_method(name = "get_horizontalFit")]
    pub fn get_horizontal_fit(&self) -> FitMode {}

    #[unity_method(name = "set_horizontalFit")]
    pub fn set_horizontal_fit(&self, value: FitMode) {}

    #[unity_method(name = "get_verticalFit")]
    pub fn get_vertical_fit(&self) -> FitMode {}

    #[unity_method(name = "set_verticalFit")]
    pub fn set_vertical_fit(&self, value: FitMode) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

}
