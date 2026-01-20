#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{BaseInputModule, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "PointerInputModule", namespace = "UnityEngine.EventSystems", inherit = "BaseInputModule,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct PointerInputModule(pub *mut c_void);

#[unity_impl]
impl PointerInputModule {
    #[unity_method(name = "IsPointerOverGameObject")]
    pub fn is_pointer_over_game_object(&self, pointer_id: i32) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
