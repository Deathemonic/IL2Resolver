#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::base_event_data::BaseEventData;
use super::canvas_update::CanvasUpdate;
use super::graphic::Graphic;
use super::pointer_event_data::PointerEventData;
use super::toggle_event::ToggleEvent;
use super::toggle_group::ToggleGroup;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Selectable, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Toggle", namespace = "UnityEngine.UI", inherit = "Selectable,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Toggle(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleTransition {
    #[default]
    None = 0,
    Fade = 1,
}

#[unity_impl]
impl Toggle {
    #[unity_method(name = "get_group")]
    pub fn get_group(&self) -> Option<ToggleGroup> {}

    #[unity_method(name = "set_group")]
    pub fn set_group(&self, value: Option<ToggleGroup>) {}

    #[unity_method(name = "get_isOn")]
    pub fn get_is_on(&self) -> bool {}

    #[unity_method(name = "set_isOn")]
    pub fn set_is_on(&self, value: bool) {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, executing: CanvasUpdate) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "SetIsOnWithoutNotify")]
    pub fn set_is_on_without_notify(&self, value: bool) {}

    #[unity_method(name = "OnPointerClick")]
    pub fn on_pointer_click(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSubmit")]
    pub fn on_submit(&self, event_data: Option<BaseEventData>) {}

}
