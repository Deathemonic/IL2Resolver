#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use super::event_modifiers::EventModifiers;
use super::event_type::EventType;
use super::pointer_type::PointerType;
use crate::core_module::{KeyCode, Ray};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "Event", namespace = "UnityEngine")]
pub struct Event(pub *mut c_void);

#[unity_impl]
impl Event {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(display_index: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(other: Option<Event>) -> Option<Self> {}

    #[unity_method(name = "get_mouseRay")]
    pub fn get_mouse_ray(&self) -> Ray {}

    #[unity_method(name = "set_mouseRay")]
    pub fn set_mouse_ray(&self, value: Ray) {}

    #[unity_method(name = "get_shift")]
    pub fn get_shift(&self) -> bool {}

    #[unity_method(name = "set_shift")]
    pub fn set_shift(&self, value: bool) {}

    #[unity_method(name = "get_control")]
    pub fn get_control(&self) -> bool {}

    #[unity_method(name = "set_control")]
    pub fn set_control(&self, value: bool) {}

    #[unity_method(name = "get_alt")]
    pub fn get_alt(&self) -> bool {}

    #[unity_method(name = "set_alt")]
    pub fn set_alt(&self, value: bool) {}

    #[unity_method(name = "get_command")]
    pub fn get_command(&self) -> bool {}

    #[unity_method(name = "set_command")]
    pub fn set_command(&self, value: bool) {}

    #[unity_method(name = "get_capsLock")]
    pub fn get_caps_lock(&self) -> bool {}

    #[unity_method(name = "set_capsLock")]
    pub fn set_caps_lock(&self, value: bool) {}

    #[unity_method(name = "get_numeric")]
    pub fn get_numeric(&self) -> bool {}

    #[unity_method(name = "set_numeric")]
    pub fn set_numeric(&self, value: bool) {}

    #[unity_method(name = "get_functionKey")]
    pub fn get_function_key(&self) -> bool {}

    #[unity_method(name = "get_current", static)]
    pub fn get_current() -> Option<Event> {}

    #[unity_method(name = "set_current", static)]
    pub fn set_current(value: Option<Event>) {}

    #[unity_method(name = "get_isKey")]
    pub fn get_is_key(&self) -> bool {}

    #[unity_method(name = "get_isMouse")]
    pub fn get_is_mouse(&self) -> bool {}

    #[unity_method(name = "get_isScrollWheel")]
    pub fn get_is_scroll_wheel(&self) -> bool {}

    #[unity_icall("UnityEngine.Event::get_rawType")]
    pub fn get_raw_type(&self) -> EventType {}

    #[unity_icall("UnityEngine.Event::get_mousePosition_Injected(Vector2&)")]
    pub fn get_mouse_position(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Event::set_mousePosition_Injected(Vector2&)")]
    pub fn set_mouse_position(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.Event::get_delta_Injected(Vector2&)")]
    pub fn get_delta(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Event::set_delta_Injected(Vector2&)")]
    pub fn set_delta(&self, value: &mut Vector2) {}

    #[unity_icall("UnityEngine.Event::get_pointerType")]
    pub fn get_pointer_type(&self) -> PointerType {}

    #[unity_icall("UnityEngine.Event::set_pointerType(PointerType)")]
    pub fn set_pointer_type(&self, value: PointerType) {}

    #[unity_icall("UnityEngine.Event::get_button")]
    pub fn get_button(&self) -> i32 {}

    #[unity_icall("UnityEngine.Event::set_button(System.Int32)")]
    pub fn set_button(&self, value: i32) {}

    #[unity_icall("UnityEngine.Event::get_modifiers")]
    pub fn get_modifiers(&self) -> EventModifiers {}

    #[unity_icall("UnityEngine.Event::set_modifiers(EventModifiers)")]
    pub fn set_modifiers(&self, value: EventModifiers) {}

    #[unity_icall("UnityEngine.Event::get_pressure")]
    pub fn get_pressure(&self) -> f32 {}

    #[unity_icall("UnityEngine.Event::set_pressure(System.Single)")]
    pub fn set_pressure(&self, value: f32) {}

    #[unity_icall("UnityEngine.Event::get_clickCount")]
    pub fn get_click_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Event::set_clickCount(System.Int32)")]
    pub fn set_click_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.Event::get_character")]
    pub fn get_character(&self) -> u16 {}

    #[unity_icall("UnityEngine.Event::set_character(System.Char)")]
    pub fn set_character(&self, value: u16) {}

    #[unity_icall("UnityEngine.Event::get_keyCode")]
    pub fn get_key_code(&self) -> KeyCode {}

    #[unity_icall("UnityEngine.Event::set_keyCode(KeyCode)")]
    pub fn set_key_code(&self, value: KeyCode) {}

    #[unity_icall("UnityEngine.Event::get_displayIndex")]
    pub fn get_display_index(&self) -> i32 {}

    #[unity_icall("UnityEngine.Event::set_displayIndex(System.Int32)")]
    pub fn set_display_index(&self, value: i32) {}

    #[unity_icall("UnityEngine.Event::get_type")]
    pub fn get_type(&self) -> EventType {}

    #[unity_icall("UnityEngine.Event::set_type(EventType)")]
    pub fn set_type(&self, value: EventType) {}

    #[unity_icall("UnityEngine.Event::get_commandName")]
    pub fn get_command_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Event::set_commandName(System.String)")]
    pub fn set_command_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.Event::Internal_Use")]
    pub fn internal_use(&self) {}

    #[unity_icall("UnityEngine.Event::Internal_Create(System.Int32)")]
    pub fn internal_create(display_index: i32) -> isize {}

    #[unity_icall("UnityEngine.Event::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.Event::Internal_Copy(System.IntPtr)")]
    pub fn internal_copy(other_ptr: isize) -> isize {}

    #[unity_icall("UnityEngine.Event::GetTypeForControl(System.Int32)")]
    pub fn get_type_for_control(&self, control_id: i32) -> EventType {}

    #[unity_icall("UnityEngine.Event::CopyFromPtr(System.IntPtr)")]
    pub fn copy_from_ptr(&self, ptr: isize) {}

    #[unity_icall("UnityEngine.Event::PopEvent(Event)")]
    pub fn pop_event(out_event: Option<Event>) -> bool {}

    #[unity_icall("UnityEngine.Event::GetEventCount")]
    pub fn get_event_count() -> i32 {}

    #[unity_icall("UnityEngine.Event::Internal_SetNativeEvent(System.IntPtr)")]
    pub fn internal_set_native_event(ptr: isize) {}

    #[unity_icall("UnityEngine.Event::GetDoubleClickTime")]
    pub fn get_double_click_time() -> i32 {}

}
