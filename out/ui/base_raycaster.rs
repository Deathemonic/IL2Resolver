#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{List};
use super::pointer_event_data::PointerEventData;
use super::raycast_result::RaycastResult;
use crate::core_module::Camera;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "BaseRaycaster", namespace = "UnityEngine.EventSystems", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct BaseRaycaster(pub *mut c_void);

#[unity_impl]
impl BaseRaycaster {
    #[unity_method(name = "get_eventCamera")]
    pub fn get_event_camera(&self) -> Option<Camera> {}

    #[unity_method(name = "get_priority")]
    pub fn get_priority(&self) -> i32 {}

    #[unity_method(name = "get_sortOrderPriority")]
    pub fn get_sort_order_priority(&self) -> i32 {}

    #[unity_method(name = "get_renderOrderPriority")]
    pub fn get_render_order_priority(&self) -> i32 {}

    #[unity_method(name = "get_rootRaycaster")]
    pub fn get_root_raycaster(&self) -> Option<BaseRaycaster> {}

    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, event_data: Option<PointerEventData>, result_append_list: List<RaycastResult>) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
