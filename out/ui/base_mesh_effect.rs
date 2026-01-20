#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::vertex_helper::VertexHelper;
use crate::core_module::Mesh;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "BaseMeshEffect", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct BaseMeshEffect(pub *mut c_void);

#[unity_impl]
impl BaseMeshEffect {
    #[unity_method(name = "ModifyMesh")]
    pub fn modify_mesh(&self, mesh: Option<Mesh>) {}

    #[unity_method(name = "ModifyMesh")]
    pub fn modify_mesh_1(&self, vh: Option<VertexHelper>) {}

}
