#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::event_system::EventSystem;
use super::move_direction::MoveDirection;
use crate::ui::{AbstractEventData, BaseEventData};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "AxisEventData", namespace = "UnityEngine.EventSystems", inherit = "BaseEventData,AbstractEventData")]
pub struct AxisEventData(pub *mut c_void);

#[unity_impl]
impl AxisEventData {
    #[unity_ctor]
    pub fn new(event_system: Option<EventSystem>) -> Option<Self> {}

    #[unity_method(name = "get_moveVector")]
    pub fn get_move_vector(&self) -> Vector2 {}

    #[unity_method(name = "set_moveVector")]
    pub fn set_move_vector(&self, value: Vector2) {}

    #[unity_method(name = "get_moveDir")]
    pub fn get_move_dir(&self) -> MoveDirection {}

    #[unity_method(name = "set_moveDir")]
    pub fn set_move_dir(&self, value: MoveDirection) {}

}
