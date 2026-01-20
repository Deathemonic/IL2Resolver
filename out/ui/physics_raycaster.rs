#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{List};
use super::pointer_event_data::PointerEventData;
use super::raycast_result::RaycastResult;
use crate::core_module::{Camera, LayerMask};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseRaycaster, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "PhysicsRaycaster", namespace = "UnityEngine.EventSystems", inherit = "BaseRaycaster,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct PhysicsRaycaster(pub *mut c_void);

#[unity_impl]
impl PhysicsRaycaster {
    #[unity_method(name = "get_eventCamera")]
    pub fn get_event_camera(&self) -> Option<Camera> {}

    #[unity_method(name = "get_depth")]
    pub fn get_depth(&self) -> i32 {}

    #[unity_method(name = "get_finalEventMask")]
    pub fn get_final_event_mask(&self) -> i32 {}

    #[unity_method(name = "get_eventMask")]
    pub fn get_event_mask(&self) -> LayerMask {}

    #[unity_method(name = "set_eventMask")]
    pub fn set_event_mask(&self, value: LayerMask) {}

    #[unity_method(name = "get_maxRayIntersections")]
    pub fn get_max_ray_intersections(&self) -> i32 {}

    #[unity_method(name = "set_maxRayIntersections")]
    pub fn set_max_ray_intersections(&self, value: i32) {}

    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, event_data: Option<PointerEventData>, result_append_list: List<RaycastResult>) {}

}
