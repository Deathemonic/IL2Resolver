#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{List};
use super::base_event_data::BaseEventData;
use super::base_input_module::BaseInputModule;
use super::pointer_event_data::PointerEventData;
use super::raycast_result::RaycastResult;
use crate::core_module::GameObject;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "EventSystem", namespace = "UnityEngine.EventSystems", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct EventSystem(pub *mut c_void);

#[unity_impl]
impl EventSystem {
    #[unity_method(name = "get_current", static)]
    pub fn get_current() -> Option<EventSystem> {}

    #[unity_method(name = "set_current", static)]
    pub fn set_current(value: Option<EventSystem>) {}

    #[unity_method(name = "get_sendNavigationEvents")]
    pub fn get_send_navigation_events(&self) -> bool {}

    #[unity_method(name = "set_sendNavigationEvents")]
    pub fn set_send_navigation_events(&self, value: bool) {}

    #[unity_method(name = "get_pixelDragThreshold")]
    pub fn get_pixel_drag_threshold(&self) -> i32 {}

    #[unity_method(name = "set_pixelDragThreshold")]
    pub fn set_pixel_drag_threshold(&self, value: i32) {}

    #[unity_method(name = "get_currentInputModule")]
    pub fn get_current_input_module(&self) -> Option<BaseInputModule> {}

    #[unity_method(name = "get_firstSelectedGameObject")]
    pub fn get_first_selected_game_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_firstSelectedGameObject")]
    pub fn set_first_selected_game_object(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_currentSelectedGameObject")]
    pub fn get_current_selected_game_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "get_lastSelectedGameObject")]
    pub fn get_last_selected_game_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "get_isFocused")]
    pub fn get_is_focused(&self) -> bool {}

    #[unity_method(name = "get_alreadySelecting")]
    pub fn get_already_selecting(&self) -> bool {}

    #[unity_method(name = "UpdateModules")]
    pub fn update_modules(&self) {}

    #[unity_method(name = "SetSelectedGameObject")]
    pub fn set_selected_game_object(&self, selected: Option<GameObject>, pointer: Option<BaseEventData>) {}

    #[unity_method(name = "SetSelectedGameObject")]
    pub fn set_selected_game_object_1(&self, selected: Option<GameObject>) {}

    #[unity_method(name = "RaycastAll")]
    pub fn raycast_all(&self, event_data: Option<PointerEventData>, raycast_results: List<RaycastResult>) {}

    #[unity_method(name = "IsPointerOverGameObject")]
    pub fn is_pointer_over_game_object(&self) -> bool {}

    #[unity_method(name = "IsPointerOverGameObject")]
    pub fn is_pointer_over_game_object_1(&self, pointer_id: i32) -> bool {}

    #[unity_method(name = "SetUITookitEventSystemOverride", static)]
    pub fn set_ui_tookit_event_system_override(active_event_system: Option<EventSystem>, send_events: bool, create_panel_game_objects_on_start: bool) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
