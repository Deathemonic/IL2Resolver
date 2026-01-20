#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::base_input::BaseInput;
use super::pointer_event_data::PointerEventData;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "BaseInputModule", namespace = "UnityEngine.EventSystems", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct BaseInputModule(pub *mut c_void);

#[unity_impl]
impl BaseInputModule {
    #[unity_method(name = "get_input")]
    pub fn get_input(&self) -> Option<BaseInput> {}

    #[unity_method(name = "get_inputOverride")]
    pub fn get_input_override(&self) -> Option<BaseInput> {}

    #[unity_method(name = "set_inputOverride")]
    pub fn set_input_override(&self, value: Option<BaseInput>) {}

    #[unity_method(name = "Process")]
    pub fn process(&self) {}

    #[unity_method(name = "IsPointerOverGameObject")]
    pub fn is_pointer_over_game_object(&self, pointer_id: i32) -> bool {}

    #[unity_method(name = "ShouldActivateModule")]
    pub fn should_activate_module(&self) -> bool {}

    #[unity_method(name = "DeactivateModule")]
    pub fn deactivate_module(&self) {}

    #[unity_method(name = "ActivateModule")]
    pub fn activate_module(&self) {}

    #[unity_method(name = "UpdateModule")]
    pub fn update_module(&self) {}

    #[unity_method(name = "IsModuleSupported")]
    pub fn is_module_supported(&self) -> bool {}

    #[unity_method(name = "ConvertUIToolkitPointerId")]
    pub fn convert_ui_toolkit_pointer_id(&self, source_pointer_data: Option<PointerEventData>) -> i32 {}

}
