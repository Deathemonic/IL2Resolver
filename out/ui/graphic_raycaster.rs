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
#[unity(assembly = "UnityEngine.UI", class = "GraphicRaycaster", namespace = "UnityEngine.UI", inherit = "BaseRaycaster,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct GraphicRaycaster(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockingObjects {
    #[default]
    None = 0,
    TwoD = 1,
    ThreeD = 2,
    All = 3,
}

#[unity_impl]
impl GraphicRaycaster {
    #[unity_method(name = "get_sortOrderPriority")]
    pub fn get_sort_order_priority(&self) -> i32 {}

    #[unity_method(name = "get_renderOrderPriority")]
    pub fn get_render_order_priority(&self) -> i32 {}

    #[unity_method(name = "get_ignoreReversedGraphics")]
    pub fn get_ignore_reversed_graphics(&self) -> bool {}

    #[unity_method(name = "set_ignoreReversedGraphics")]
    pub fn set_ignore_reversed_graphics(&self, value: bool) {}

    #[unity_method(name = "get_blockingObjects")]
    pub fn get_blocking_objects(&self) -> BlockingObjects {}

    #[unity_method(name = "set_blockingObjects")]
    pub fn set_blocking_objects(&self, value: BlockingObjects) {}

    #[unity_method(name = "get_blockingMask")]
    pub fn get_blocking_mask(&self) -> LayerMask {}

    #[unity_method(name = "set_blockingMask")]
    pub fn set_blocking_mask(&self, value: LayerMask) {}

    #[unity_method(name = "get_eventCamera")]
    pub fn get_event_camera(&self) -> Option<Camera> {}

    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, event_data: Option<PointerEventData>, result_append_list: List<RaycastResult>) {}

}
