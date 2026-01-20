#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::axis_event_data::AxisEventData;
use super::canvas_update::CanvasUpdate;
use super::pointer_event_data::PointerEventData;
use super::slider_event::SliderEvent;
use crate::core_module::RectTransform;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Selectable, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Slider", namespace = "UnityEngine.UI", inherit = "Selectable,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Slider(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    LeftToRight = 0,
    RightToLeft = 1,
    BottomToTop = 2,
    TopToBottom = 3,
}

#[unity_impl]
impl Slider {
    #[unity_method(name = "get_fillRect")]
    pub fn get_fill_rect(&self) -> Option<RectTransform> {}

    #[unity_method(name = "set_fillRect")]
    pub fn set_fill_rect(&self, value: Option<RectTransform>) {}

    #[unity_method(name = "get_handleRect")]
    pub fn get_handle_rect(&self) -> Option<RectTransform> {}

    #[unity_method(name = "set_handleRect")]
    pub fn set_handle_rect(&self, value: Option<RectTransform>) {}

    #[unity_method(name = "get_direction")]
    pub fn get_direction(&self) -> Direction {}

    #[unity_method(name = "set_direction")]
    pub fn set_direction(&self, value: Direction) {}

    #[unity_method(name = "get_minValue")]
    pub fn get_min_value(&self) -> f32 {}

    #[unity_method(name = "set_minValue")]
    pub fn set_min_value(&self, value: f32) {}

    #[unity_method(name = "get_maxValue")]
    pub fn get_max_value(&self) -> f32 {}

    #[unity_method(name = "set_maxValue")]
    pub fn set_max_value(&self, value: f32) {}

    #[unity_method(name = "get_wholeNumbers")]
    pub fn get_whole_numbers(&self) -> bool {}

    #[unity_method(name = "set_wholeNumbers")]
    pub fn set_whole_numbers(&self, value: bool) {}

    #[unity_method(name = "get_value")]
    pub fn get_value(&self) -> f32 {}

    #[unity_method(name = "set_value")]
    pub fn set_value(&self, value: f32) {}

    #[unity_method(name = "get_normalizedValue")]
    pub fn get_normalized_value(&self) -> f32 {}

    #[unity_method(name = "set_normalizedValue")]
    pub fn set_normalized_value(&self, value: f32) {}

    #[unity_method(name = "get_onValueChanged")]
    pub fn get_on_value_changed(&self) -> Option<SliderEvent> {}

    #[unity_method(name = "set_onValueChanged")]
    pub fn set_on_value_changed(&self, value: Option<SliderEvent>) {}

    #[unity_method(name = "SetValueWithoutNotify")]
    pub fn set_value_without_notify(&self, input: f32) {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, executing: CanvasUpdate) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "OnPointerDown")]
    pub fn on_pointer_down(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnDrag")]
    pub fn on_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnMove")]
    pub fn on_move(&self, event_data: Option<AxisEventData>) {}

    #[unity_method(name = "FindSelectableOnLeft")]
    pub fn find_selectable_on_left(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnRight")]
    pub fn find_selectable_on_right(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnUp")]
    pub fn find_selectable_on_up(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnDown")]
    pub fn find_selectable_on_down(&self) -> Option<Selectable> {}

    #[unity_method(name = "OnInitializePotentialDrag")]
    pub fn on_initialize_potential_drag(&self, event_data: Option<PointerEventData>) {}

}
