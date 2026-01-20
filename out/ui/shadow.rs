#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::vertex_helper::VertexHelper;
use crate::core_module::Color;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseMeshEffect, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Shadow", namespace = "UnityEngine.UI", inherit = "BaseMeshEffect,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Shadow(pub *mut c_void);

#[unity_impl]
impl Shadow {
    #[unity_method(name = "get_effectColor")]
    pub fn get_effect_color(&self) -> Color {}

    #[unity_method(name = "set_effectColor")]
    pub fn set_effect_color(&self, value: Color) {}

    #[unity_method(name = "get_effectDistance")]
    pub fn get_effect_distance(&self) -> Vector2 {}

    #[unity_method(name = "set_effectDistance")]
    pub fn set_effect_distance(&self, value: Vector2) {}

    #[unity_method(name = "get_useGraphicAlpha")]
    pub fn get_use_graphic_alpha(&self) -> bool {}

    #[unity_method(name = "set_useGraphicAlpha")]
    pub fn set_use_graphic_alpha(&self, value: bool) {}

    #[unity_method(name = "ModifyMesh")]
    pub fn modify_mesh(&self, vh: Option<VertexHelper>) {}

}
