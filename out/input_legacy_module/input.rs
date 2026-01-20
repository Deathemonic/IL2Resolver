#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector3};
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::acceleration_event::AccelerationEvent;
use super::compass::Compass;
use super::device_orientation::DeviceOrientation;
use super::gyroscope::Gyroscope;
use super::ime_composition_mode::IMECompositionMode;
use super::location_service::LocationService;
use super::touch::Touch;
use crate::core_module::KeyCode;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "Input", namespace = "UnityEngine")]
pub struct Input(pub *mut c_void);

#[unity_impl]
impl Input {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Input::get_simulateMouseWithTouches")]
    pub fn get_simulate_mouse_with_touches() -> bool {}

    #[unity_icall("UnityEngine.Input::set_simulateMouseWithTouches(System.Boolean)")]
    pub fn set_simulate_mouse_with_touches(value: bool) {}

    #[unity_icall("UnityEngine.Input::get_anyKey")]
    pub fn get_any_key() -> bool {}

    #[unity_icall("UnityEngine.Input::get_anyKeyDown")]
    pub fn get_any_key_down() -> bool {}

    #[unity_icall("UnityEngine.Input::get_inputString")]
    pub fn get_input_string() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Input::get_mousePosition_Injected(Vector3&)")]
    pub fn get_mouse_position(ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Input::get_mouseScrollDelta_Injected(Vector2&)")]
    pub fn get_mouse_scroll_delta(ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Input::get_imeCompositionMode")]
    pub fn get_ime_composition_mode() -> IMECompositionMode {}

    #[unity_icall("UnityEngine.Input::set_imeCompositionMode(IMECompositionMode)")]
    pub fn set_ime_composition_mode(value: IMECompositionMode) {}

    #[unity_icall("UnityEngine.Input::get_compositionString")]
    pub fn get_composition_string() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Input::get_imeIsSelected")]
    pub fn get_ime_is_selected() -> bool {}

    #[unity_icall("UnityEngine.Input::get_compositionCursorPos_Injected(Vector2&)")]
    pub fn get_composition_cursor_pos(ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Input::set_compositionCursorPos_Injected(Vector2&)")]
    pub fn set_composition_cursor_pos(value: &mut Vector2) {}

    #[unity_icall("UnityEngine.Input::get_eatKeyPressOnTextFieldFocus")]
    pub fn get_eat_key_press_on_text_field_focus() -> bool {}

    #[unity_icall("UnityEngine.Input::set_eatKeyPressOnTextFieldFocus(System.Boolean)")]
    pub fn set_eat_key_press_on_text_field_focus(value: bool) {}

    #[unity_icall("UnityEngine.Input::get_mousePresent")]
    pub fn get_mouse_present() -> bool {}

    #[unity_icall("UnityEngine.Input::get_touchCount")]
    pub fn get_touch_count() -> i32 {}

    #[unity_icall("UnityEngine.Input::get_touchPressureSupported")]
    pub fn get_touch_pressure_supported() -> bool {}

    #[unity_icall("UnityEngine.Input::get_stylusTouchSupported")]
    pub fn get_stylus_touch_supported() -> bool {}

    #[unity_icall("UnityEngine.Input::get_touchSupported")]
    pub fn get_touch_supported() -> bool {}

    #[unity_icall("UnityEngine.Input::get_multiTouchEnabled")]
    pub fn get_multi_touch_enabled() -> bool {}

    #[unity_icall("UnityEngine.Input::set_multiTouchEnabled(System.Boolean)")]
    pub fn set_multi_touch_enabled(value: bool) {}

    #[unity_icall("UnityEngine.Input::get_isGyroAvailable")]
    pub fn get_is_gyro_available() -> bool {}

    #[unity_icall("UnityEngine.Input::get_deviceOrientation")]
    pub fn get_device_orientation() -> DeviceOrientation {}

    #[unity_icall("UnityEngine.Input::get_acceleration")]
    pub fn get_acceleration() -> Vector3 {}

    #[unity_icall("UnityEngine.Input::get_compensateSensors")]
    pub fn get_compensate_sensors() -> bool {}

    #[unity_icall("UnityEngine.Input::set_compensateSensors(System.Boolean)")]
    pub fn set_compensate_sensors(value: bool) {}

    #[unity_icall("UnityEngine.Input::get_accelerationEventCount")]
    pub fn get_acceleration_event_count() -> i32 {}

    #[unity_icall("UnityEngine.Input::get_backButtonLeavesApp")]
    pub fn get_back_button_leaves_app() -> bool {}

    #[unity_icall("UnityEngine.Input::set_backButtonLeavesApp(System.Boolean)")]
    pub fn set_back_button_leaves_app(value: bool) {}

    #[unity_method(name = "get_location", static)]
    pub fn get_location() -> Option<LocationService> {}

    #[unity_method(name = "get_compass", static)]
    pub fn get_compass() -> Option<Compass> {}

    #[unity_method(name = "get_gyro", static)]
    pub fn get_gyro() -> Option<Gyroscope> {}

    #[unity_method(name = "get_touches", static)]
    pub fn get_touches() -> Array<Touch> {}

    #[unity_method(name = "get_accelerationEvents", static)]
    pub fn get_acceleration_events() -> Array<AccelerationEvent> {}

    #[unity_icall("UnityEngine.Input::GetKeyInt(KeyCode)")]
    pub fn get_key_int(key: KeyCode) -> bool {}

    #[unity_icall("UnityEngine.Input::GetKeyString(System.String)")]
    pub fn get_key_string(name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetKeyUpInt(KeyCode)")]
    pub fn get_key_up_int(key: KeyCode) -> bool {}

    #[unity_icall("UnityEngine.Input::GetKeyUpString(System.String)")]
    pub fn get_key_up_string(name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetKeyDownInt(KeyCode)")]
    pub fn get_key_down_int(key: KeyCode) -> bool {}

    #[unity_icall("UnityEngine.Input::GetKeyDownString(System.String)")]
    pub fn get_key_down_string(name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetAxis(System.String)")]
    pub fn get_axis(axis_name: &str) -> f32 {}

    #[unity_icall("UnityEngine.Input::GetAxisRaw(System.String)")]
    pub fn get_axis_raw(axis_name: &str) -> f32 {}

    #[unity_icall("UnityEngine.Input::GetButton(System.String)")]
    pub fn get_button(button_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetButtonDown(System.String)")]
    pub fn get_button_down(button_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetButtonUp(System.String)")]
    pub fn get_button_up(button_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetMouseButton(System.Int32)")]
    pub fn get_mouse_button(button: i32) -> bool {}

    #[unity_icall("UnityEngine.Input::GetMouseButtonDown(System.Int32)")]
    pub fn get_mouse_button_down(button: i32) -> bool {}

    #[unity_icall("UnityEngine.Input::GetMouseButtonUp(System.Int32)")]
    pub fn get_mouse_button_up(button: i32) -> bool {}

    #[unity_icall("UnityEngine.Input::ResetInputAxes")]
    pub fn reset_input_axes() {}

    #[unity_icall("UnityEngine.Input::IsJoystickPreconfigured(System.String)")]
    pub fn is_joystick_preconfigured(joystick_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Input::GetJoystickNames")]
    pub fn get_joystick_names() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Input::SimulateTouchInternal(Touch,System.Int64)")]
    pub fn simulate_touch_internal(touch: Touch, timestamp: i64) {}

    #[unity_icall("UnityEngine.Input::GetGyroInternal")]
    pub fn get_gyro_internal() -> i32 {}

    #[unity_icall("UnityEngine.Input::CheckDisabled")]
    pub fn check_disabled() -> bool {}

    #[unity_icall("UnityEngine.Input::GetTouch_Injected(System.Int32,Touch&)")]
    pub fn get_touch(index: i32, ret: &mut Touch) {}

    #[unity_icall("UnityEngine.Input::GetAccelerationEvent_Injected(System.Int32,AccelerationEvent&)")]
    pub fn get_acceleration_event(index: i32, ret: &mut AccelerationEvent) {}

    #[unity_icall("UnityEngine.Input::SimulateTouchInternal_Injected(Touch&,System.Int64)")]
    pub fn simulate_touch_internal_1(touch: &mut Touch, timestamp: i64) {}

    #[unity_icall("UnityEngine.Input::get_acceleration_Injected(Vector3&)")]
    pub fn get_acceleration_1(ret: &mut Vector3) {}

}
