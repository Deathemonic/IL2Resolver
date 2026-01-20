#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{List};
use super::pointer_event_data::PointerEventData;
use super::raycast_result::RaycastResult;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseRaycaster, PhysicsRaycaster, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Physics2DRaycaster", namespace = "UnityEngine.EventSystems", inherit = "PhysicsRaycaster,BaseRaycaster,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Physics2DRaycaster(pub *mut c_void);

#[unity_impl]
impl Physics2DRaycaster {
    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, event_data: Option<PointerEventData>, result_append_list: List<RaycastResult>) {}

}
