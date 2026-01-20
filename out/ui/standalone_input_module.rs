#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseInputModule, PointerInputModule, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "StandaloneInputModule", namespace = "UnityEngine.EventSystems", inherit = "PointerInputModule,BaseInputModule,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct StandaloneInputModule(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Mouse = 0,
    Buttons = 1,
}

#[unity_impl]
impl StandaloneInputModule {
    #[unity_method(name = "get_inputMode")]
    pub fn get_input_mode(&self) -> InputMode {}

    #[unity_method(name = "get_allowActivationOnMobileDevice")]
    pub fn get_allow_activation_on_mobile_device(&self) -> bool {}

    #[unity_method(name = "set_allowActivationOnMobileDevice")]
    pub fn set_allow_activation_on_mobile_device(&self, value: bool) {}

    #[unity_method(name = "get_forceModuleActive")]
    pub fn get_force_module_active(&self) -> bool {}

    #[unity_method(name = "set_forceModuleActive")]
    pub fn set_force_module_active(&self, value: bool) {}

    #[unity_method(name = "get_inputActionsPerSecond")]
    pub fn get_input_actions_per_second(&self) -> f32 {}

    #[unity_method(name = "set_inputActionsPerSecond")]
    pub fn set_input_actions_per_second(&self, value: f32) {}

    #[unity_method(name = "get_repeatDelay")]
    pub fn get_repeat_delay(&self) -> f32 {}

    #[unity_method(name = "set_repeatDelay")]
    pub fn set_repeat_delay(&self, value: f32) {}

    #[unity_method(name = "get_horizontalAxis")]
    pub fn get_horizontal_axis(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_horizontalAxis")]
    pub fn set_horizontal_axis(&self, value: &str) {}

    #[unity_method(name = "get_verticalAxis")]
    pub fn get_vertical_axis(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_verticalAxis")]
    pub fn set_vertical_axis(&self, value: &str) {}

    #[unity_method(name = "get_submitButton")]
    pub fn get_submit_button(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_submitButton")]
    pub fn set_submit_button(&self, value: &str) {}

    #[unity_method(name = "get_cancelButton")]
    pub fn get_cancel_button(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_cancelButton")]
    pub fn set_cancel_button(&self, value: &str) {}

    #[unity_method(name = "UpdateModule")]
    pub fn update_module(&self) {}

    #[unity_method(name = "ShouldActivateModule")]
    pub fn should_activate_module(&self) -> bool {}

    #[unity_method(name = "ActivateModule")]
    pub fn activate_module(&self) {}

    #[unity_method(name = "DeactivateModule")]
    pub fn deactivate_module(&self) {}

    #[unity_method(name = "Process")]
    pub fn process(&self) {}

}
