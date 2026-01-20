#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::canvas_update::CanvasUpdate;
use super::pointer_event_data::PointerEventData;
use super::scrollbar::Scrollbar;
use super::scroll_rect_event::ScrollRectEvent;
use crate::core_module::RectTransform;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "ScrollRect", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct ScrollRect(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementType {
    #[default]
    Unrestricted = 0,
    Elastic = 1,
    Clamped = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarVisibility {
    #[default]
    Permanent = 0,
    AutoHide = 1,
    AutoHideAndExpandViewport = 2,
}

#[unity_impl]
impl ScrollRect {
    #[unity_method(name = "get_content")]
    pub fn get_content(&self) -> Option<RectTransform> {}

    #[unity_method(name = "set_content")]
    pub fn set_content(&self, value: Option<RectTransform>) {}

    #[unity_method(name = "get_horizontal")]
    pub fn get_horizontal(&self) -> bool {}

    #[unity_method(name = "set_horizontal")]
    pub fn set_horizontal(&self, value: bool) {}

    #[unity_method(name = "get_vertical")]
    pub fn get_vertical(&self) -> bool {}

    #[unity_method(name = "set_vertical")]
    pub fn set_vertical(&self, value: bool) {}

    #[unity_method(name = "get_movementType")]
    pub fn get_movement_type(&self) -> MovementType {}

    #[unity_method(name = "set_movementType")]
    pub fn set_movement_type(&self, value: MovementType) {}

    #[unity_method(name = "get_elasticity")]
    pub fn get_elasticity(&self) -> f32 {}

    #[unity_method(name = "set_elasticity")]
    pub fn set_elasticity(&self, value: f32) {}

    #[unity_method(name = "get_inertia")]
    pub fn get_inertia(&self) -> bool {}

    #[unity_method(name = "set_inertia")]
    pub fn set_inertia(&self, value: bool) {}

    #[unity_method(name = "get_decelerationRate")]
    pub fn get_deceleration_rate(&self) -> f32 {}

    #[unity_method(name = "set_decelerationRate")]
    pub fn set_deceleration_rate(&self, value: f32) {}

    #[unity_method(name = "get_scrollSensitivity")]
    pub fn get_scroll_sensitivity(&self) -> f32 {}

    #[unity_method(name = "set_scrollSensitivity")]
    pub fn set_scroll_sensitivity(&self, value: f32) {}

    #[unity_method(name = "get_viewport")]
    pub fn get_viewport(&self) -> Option<RectTransform> {}

    #[unity_method(name = "set_viewport")]
    pub fn set_viewport(&self, value: Option<RectTransform>) {}

    #[unity_method(name = "get_horizontalScrollbar")]
    pub fn get_horizontal_scrollbar(&self) -> Option<Scrollbar> {}

    #[unity_method(name = "set_horizontalScrollbar")]
    pub fn set_horizontal_scrollbar(&self, value: Option<Scrollbar>) {}

    #[unity_method(name = "get_verticalScrollbar")]
    pub fn get_vertical_scrollbar(&self) -> Option<Scrollbar> {}

    #[unity_method(name = "set_verticalScrollbar")]
    pub fn set_vertical_scrollbar(&self, value: Option<Scrollbar>) {}

    #[unity_method(name = "get_horizontalScrollbarVisibility")]
    pub fn get_horizontal_scrollbar_visibility(&self) -> ScrollbarVisibility {}

    #[unity_method(name = "set_horizontalScrollbarVisibility")]
    pub fn set_horizontal_scrollbar_visibility(&self, value: ScrollbarVisibility) {}

    #[unity_method(name = "get_verticalScrollbarVisibility")]
    pub fn get_vertical_scrollbar_visibility(&self) -> ScrollbarVisibility {}

    #[unity_method(name = "set_verticalScrollbarVisibility")]
    pub fn set_vertical_scrollbar_visibility(&self, value: ScrollbarVisibility) {}

    #[unity_method(name = "get_horizontalScrollbarSpacing")]
    pub fn get_horizontal_scrollbar_spacing(&self) -> f32 {}

    #[unity_method(name = "set_horizontalScrollbarSpacing")]
    pub fn set_horizontal_scrollbar_spacing(&self, value: f32) {}

    #[unity_method(name = "get_verticalScrollbarSpacing")]
    pub fn get_vertical_scrollbar_spacing(&self) -> f32 {}

    #[unity_method(name = "set_verticalScrollbarSpacing")]
    pub fn set_vertical_scrollbar_spacing(&self, value: f32) {}

    #[unity_method(name = "get_onValueChanged")]
    pub fn get_on_value_changed(&self) -> Option<ScrollRectEvent> {}

    #[unity_method(name = "set_onValueChanged")]
    pub fn set_on_value_changed(&self, value: Option<ScrollRectEvent>) {}

    #[unity_method(name = "get_velocity")]
    pub fn get_velocity(&self) -> Vector2 {}

    #[unity_method(name = "set_velocity")]
    pub fn set_velocity(&self, value: Vector2) {}

    #[unity_method(name = "get_normalizedPosition")]
    pub fn get_normalized_position(&self) -> Vector2 {}

    #[unity_method(name = "set_normalizedPosition")]
    pub fn set_normalized_position(&self, value: Vector2) {}

    #[unity_method(name = "get_horizontalNormalizedPosition")]
    pub fn get_horizontal_normalized_position(&self) -> f32 {}

    #[unity_method(name = "set_horizontalNormalizedPosition")]
    pub fn set_horizontal_normalized_position(&self, value: f32) {}

    #[unity_method(name = "get_verticalNormalizedPosition")]
    pub fn get_vertical_normalized_position(&self) -> f32 {}

    #[unity_method(name = "set_verticalNormalizedPosition")]
    pub fn set_vertical_normalized_position(&self, value: f32) {}

    #[unity_method(name = "get_minWidth")]
    pub fn get_min_width(&self) -> f32 {}

    #[unity_method(name = "get_preferredWidth")]
    pub fn get_preferred_width(&self) -> f32 {}

    #[unity_method(name = "get_flexibleWidth")]
    pub fn get_flexible_width(&self) -> f32 {}

    #[unity_method(name = "get_minHeight")]
    pub fn get_min_height(&self) -> f32 {}

    #[unity_method(name = "get_preferredHeight")]
    pub fn get_preferred_height(&self) -> f32 {}

    #[unity_method(name = "get_flexibleHeight")]
    pub fn get_flexible_height(&self) -> f32 {}

    #[unity_method(name = "get_layoutPriority")]
    pub fn get_layout_priority(&self) -> i32 {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, executing: CanvasUpdate) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "IsActive")]
    pub fn is_active(&self) -> bool {}

    #[unity_method(name = "StopMovement")]
    pub fn stop_movement(&self) {}

    #[unity_method(name = "OnScroll")]
    pub fn on_scroll(&self, data: Option<PointerEventData>) {}

    #[unity_method(name = "OnInitializePotentialDrag")]
    pub fn on_initialize_potential_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnBeginDrag")]
    pub fn on_begin_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnEndDrag")]
    pub fn on_end_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnDrag")]
    pub fn on_drag(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "SetLayoutHorizontal")]
    pub fn set_layout_horizontal(&self) {}

    #[unity_method(name = "SetLayoutVertical")]
    pub fn set_layout_vertical(&self) {}

}
