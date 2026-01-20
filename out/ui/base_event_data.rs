#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::base_input_module::BaseInputModule;
use super::event_system::EventSystem;
use crate::core_module::GameObject;
use crate::ui::AbstractEventData;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "BaseEventData", namespace = "UnityEngine.EventSystems", inherit = "AbstractEventData")]
pub struct BaseEventData(pub *mut c_void);

#[unity_impl]
impl BaseEventData {
    #[unity_ctor]
    pub fn new(event_system: Option<EventSystem>) -> Option<Self> {}

    #[unity_method(name = "get_currentInputModule")]
    pub fn get_current_input_module(&self) -> Option<BaseInputModule> {}

    #[unity_method(name = "get_selectedObject")]
    pub fn get_selected_object(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_selectedObject")]
    pub fn set_selected_object(&self, value: Option<GameObject>) {}

}
