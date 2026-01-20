#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::toggle::Toggle;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "ToggleGroup", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct ToggleGroup(pub *mut c_void);

#[unity_impl]
impl ToggleGroup {
    #[unity_method(name = "get_allowSwitchOff")]
    pub fn get_allow_switch_off(&self) -> bool {}

    #[unity_method(name = "set_allowSwitchOff")]
    pub fn set_allow_switch_off(&self, value: bool) {}

    #[unity_method(name = "NotifyToggleOn")]
    pub fn notify_toggle_on(&self, toggle: Option<Toggle>, send_callback: bool) {}

    #[unity_method(name = "UnregisterToggle")]
    pub fn unregister_toggle(&self, toggle: Option<Toggle>) {}

    #[unity_method(name = "RegisterToggle")]
    pub fn register_toggle(&self, toggle: Option<Toggle>) {}

    #[unity_method(name = "EnsureValidState")]
    pub fn ensure_valid_state(&self) {}

    #[unity_method(name = "AnyTogglesOn")]
    pub fn any_toggles_on(&self) -> bool {}

    #[unity_method(name = "GetFirstActiveToggle")]
    pub fn get_first_active_toggle(&self) -> Option<Toggle> {}

    #[unity_method(name = "SetAllTogglesOff")]
    pub fn set_all_toggles_off(&self, send_callback: bool) {}

}
