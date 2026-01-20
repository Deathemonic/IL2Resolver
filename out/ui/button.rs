#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::base_event_data::BaseEventData;
use super::button_clicked_event::ButtonClickedEvent;
use super::pointer_event_data::PointerEventData;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Selectable, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Button", namespace = "UnityEngine.UI", inherit = "Selectable,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Button(pub *mut c_void);

#[unity_impl]
impl Button {
    #[unity_method(name = "get_onClick")]
    pub fn get_on_click(&self) -> Option<ButtonClickedEvent> {}

    #[unity_method(name = "set_onClick")]
    pub fn set_on_click(&self, value: Option<ButtonClickedEvent>) {}

    #[unity_method(name = "OnPointerClick")]
    pub fn on_pointer_click(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSubmit")]
    pub fn on_submit(&self, event_data: Option<BaseEventData>) {}

}
