#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{List};
use super::axis_event_data::AxisEventData;
use super::base_event_data::BaseEventData;
use super::entry::Entry;
use super::pointer_event_data::PointerEventData;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "EventTrigger", namespace = "UnityEngine.EventSystems", inherit = "MonoBehaviour,Behaviour,Component,Object")]
pub struct EventTrigger(pub *mut c_void);

#[unity_impl]
impl EventTrigger {
    #[unity_method(name = "get_delegates")]
    pub fn get_delegates(&self) -> List<Entry> {}

    #[unity_method(name = "set_delegates")]
    pub fn set_delegates(&self, value: List<Entry>) {}

    #[unity_method(name = "get_triggers")]
    pub fn get_triggers(&self) -> List<Entry> {}

    #[unity_method(name = "set_triggers")]
    pub fn set_triggers(&self, value: List<Entry>) {}

    #[unity_method(name = "OnPointerEnter")]
    pub fn on_pointer_enter(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerExit")]
    pub fn on_pointer_exit(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnDrag")]
    pub fn on_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnDrop")]
    pub fn on_drop(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerDown")]
    pub fn on_pointer_down(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerUp")]
    pub fn on_pointer_up(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerClick")]
    pub fn on_pointer_click(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSelect")]
    pub fn on_select(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnDeselect")]
    pub fn on_deselect(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnScroll")]
    pub fn on_scroll(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnMove")]
    pub fn on_move(&self, event_data: Option<AxisEventData>) {}

    #[unity_method(name = "OnUpdateSelected")]
    pub fn on_update_selected(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnInitializePotentialDrag")]
    pub fn on_initialize_potential_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnBeginDrag")]
    pub fn on_begin_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnEndDrag")]
    pub fn on_end_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSubmit")]
    pub fn on_submit(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnCancel")]
    pub fn on_cancel(&self, event_data: Option<BaseEventData>) {}

}
