#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector3};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{List};
use super::event_system::EventSystem;
use super::raycast_result::RaycastResult;
use crate::core_module::{Camera, GameObject};
use crate::ui::{AbstractEventData, BaseEventData};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "PointerEventData", namespace = "UnityEngine.EventSystems", inherit = "BaseEventData,AbstractEventData")]
pub struct PointerEventData(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputButton {
    #[default]
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FramePressState {
    #[default]
    Pressed = 0,
    Released = 1,
    PressedAndReleased = 2,
    NotChanged = 3,
}

#[unity_impl]
impl PointerEventData {
    #[unity_ctor]
    pub fn new(event_system: Option<EventSystem>) -> Option<Self> {}

    #[unity_method(name = "get_pointerEnter")]
    pub fn get_pointer_enter(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_pointerEnter")]
    pub fn set_pointer_enter(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_lastPress")]
    pub fn get_last_press(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_lastPress")]
    pub fn set_last_press(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_rawPointerPress")]
    pub fn get_raw_pointer_press(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_rawPointerPress")]
    pub fn set_raw_pointer_press(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_pointerDrag")]
    pub fn get_pointer_drag(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_pointerDrag")]
    pub fn set_pointer_drag(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_pointerClick")]
    pub fn get_pointer_click(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_pointerClick")]
    pub fn set_pointer_click(&self, value: Option<GameObject>) {}

    #[unity_method(name = "get_pointerCurrentRaycast")]
    pub fn get_pointer_current_raycast(&self) -> RaycastResult {}

    #[unity_method(name = "set_pointerCurrentRaycast")]
    pub fn set_pointer_current_raycast(&self, value: RaycastResult) {}

    #[unity_method(name = "get_pointerPressRaycast")]
    pub fn get_pointer_press_raycast(&self) -> RaycastResult {}

    #[unity_method(name = "set_pointerPressRaycast")]
    pub fn set_pointer_press_raycast(&self, value: RaycastResult) {}

    #[unity_method(name = "get_eligibleForClick")]
    pub fn get_eligible_for_click(&self) -> bool {}

    #[unity_method(name = "set_eligibleForClick")]
    pub fn set_eligible_for_click(&self, value: bool) {}

    #[unity_method(name = "get_pointerId")]
    pub fn get_pointer_id(&self) -> i32 {}

    #[unity_method(name = "set_pointerId")]
    pub fn set_pointer_id(&self, value: i32) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Vector2 {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Vector2) {}

    #[unity_method(name = "get_delta")]
    pub fn get_delta(&self) -> Vector2 {}

    #[unity_method(name = "set_delta")]
    pub fn set_delta(&self, value: Vector2) {}

    #[unity_method(name = "get_pressPosition")]
    pub fn get_press_position(&self) -> Vector2 {}

    #[unity_method(name = "set_pressPosition")]
    pub fn set_press_position(&self, value: Vector2) {}

    #[unity_method(name = "get_worldPosition")]
    pub fn get_world_position(&self) -> Vector3 {}

    #[unity_method(name = "set_worldPosition")]
    pub fn set_world_position(&self, value: Vector3) {}

    #[unity_method(name = "get_worldNormal")]
    pub fn get_world_normal(&self) -> Vector3 {}

    #[unity_method(name = "set_worldNormal")]
    pub fn set_world_normal(&self, value: Vector3) {}

    #[unity_method(name = "get_clickTime")]
    pub fn get_click_time(&self) -> f32 {}

    #[unity_method(name = "set_clickTime")]
    pub fn set_click_time(&self, value: f32) {}

    #[unity_method(name = "get_clickCount")]
    pub fn get_click_count(&self) -> i32 {}

    #[unity_method(name = "set_clickCount")]
    pub fn set_click_count(&self, value: i32) {}

    #[unity_method(name = "get_scrollDelta")]
    pub fn get_scroll_delta(&self) -> Vector2 {}

    #[unity_method(name = "set_scrollDelta")]
    pub fn set_scroll_delta(&self, value: Vector2) {}

    #[unity_method(name = "get_useDragThreshold")]
    pub fn get_use_drag_threshold(&self) -> bool {}

    #[unity_method(name = "set_useDragThreshold")]
    pub fn set_use_drag_threshold(&self, value: bool) {}

    #[unity_method(name = "get_dragging")]
    pub fn get_dragging(&self) -> bool {}

    #[unity_method(name = "set_dragging")]
    pub fn set_dragging(&self, value: bool) {}

    #[unity_method(name = "get_button")]
    pub fn get_button(&self) -> InputButton {}

    #[unity_method(name = "set_button")]
    pub fn set_button(&self, value: InputButton) {}

    #[unity_method(name = "get_pressure")]
    pub fn get_pressure(&self) -> f32 {}

    #[unity_method(name = "set_pressure")]
    pub fn set_pressure(&self, value: f32) {}

    #[unity_method(name = "get_tangentialPressure")]
    pub fn get_tangential_pressure(&self) -> f32 {}

    #[unity_method(name = "set_tangentialPressure")]
    pub fn set_tangential_pressure(&self, value: f32) {}

    #[unity_method(name = "get_altitudeAngle")]
    pub fn get_altitude_angle(&self) -> f32 {}

    #[unity_method(name = "set_altitudeAngle")]
    pub fn set_altitude_angle(&self, value: f32) {}

    #[unity_method(name = "get_azimuthAngle")]
    pub fn get_azimuth_angle(&self) -> f32 {}

    #[unity_method(name = "set_azimuthAngle")]
    pub fn set_azimuth_angle(&self, value: f32) {}

    #[unity_method(name = "get_twist")]
    pub fn get_twist(&self) -> f32 {}

    #[unity_method(name = "set_twist")]
    pub fn set_twist(&self, value: f32) {}

    #[unity_method(name = "get_radius")]
    pub fn get_radius(&self) -> Vector2 {}

    #[unity_method(name = "set_radius")]
    pub fn set_radius(&self, value: Vector2) {}

    #[unity_method(name = "get_radiusVariance")]
    pub fn get_radius_variance(&self) -> Vector2 {}

    #[unity_method(name = "set_radiusVariance")]
    pub fn set_radius_variance(&self, value: Vector2) {}

    #[unity_method(name = "get_fullyExited")]
    pub fn get_fully_exited(&self) -> bool {}

    #[unity_method(name = "set_fullyExited")]
    pub fn set_fully_exited(&self, value: bool) {}

    #[unity_method(name = "get_reentered")]
    pub fn get_reentered(&self) -> bool {}

    #[unity_method(name = "set_reentered")]
    pub fn set_reentered(&self, value: bool) {}

    #[unity_method(name = "get_enterEventCamera")]
    pub fn get_enter_event_camera(&self) -> Option<Camera> {}

    #[unity_method(name = "get_pressEventCamera")]
    pub fn get_press_event_camera(&self) -> Option<Camera> {}

    #[unity_method(name = "get_pointerPress")]
    pub fn get_pointer_press(&self) -> Option<GameObject> {}

    #[unity_method(name = "set_pointerPress")]
    pub fn set_pointer_press(&self, value: Option<GameObject>) {}

    #[unity_method(name = "IsPointerMoving")]
    pub fn is_pointer_moving(&self) -> bool {}

    #[unity_method(name = "IsScrolling")]
    pub fn is_scrolling(&self) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
