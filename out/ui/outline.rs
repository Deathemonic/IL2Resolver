#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::vertex_helper::VertexHelper;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseMeshEffect, Shadow, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Outline", namespace = "UnityEngine.UI", inherit = "Shadow,BaseMeshEffect,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Outline(pub *mut c_void);

#[unity_impl]
impl Outline {
    #[unity_method(name = "ModifyMesh")]
    pub fn modify_mesh(&self, vh: Option<VertexHelper>) {}

}
