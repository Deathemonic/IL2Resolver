#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::range_int::RangeInt;
use super::rect::Rect;
use super::touch_screen_keyboard_type::TouchScreenKeyboardType;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "TouchScreenKeyboard", namespace = "UnityEngine")]
pub struct TouchScreenKeyboard(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Visible = 0,
    Done = 1,
    Canceled = 2,
    LostFocus = 3,
}

#[unity_impl]
impl TouchScreenKeyboard {
    #[unity_ctor]
    pub fn new(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool, secure: bool, alert: bool, text_placeholder: &str, character_limit: i32) -> Option<Self> {}

    #[unity_method(name = "get_isSupported", static)]
    pub fn get_is_supported() -> bool {}

    #[unity_method(name = "get_isInPlaceEditingAllowed", static)]
    pub fn get_is_in_place_editing_allowed() -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::set_text(System.String)")]
    pub fn set_text(&self, value: &str) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_hideInput")]
    pub fn get_hide_input() -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::set_hideInput(System.Boolean)")]
    pub fn set_hide_input(value: bool) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_active")]
    pub fn get_active(&self) -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::set_active(System.Boolean)")]
    pub fn set_active(&self, value: bool) {}

    #[unity_method(name = "get_done")]
    pub fn get_done(&self) -> bool {}

    #[unity_method(name = "get_wasCanceled")]
    pub fn get_was_canceled(&self) -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_status")]
    pub fn get_status(&self) -> Status {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_characterLimit")]
    pub fn get_character_limit(&self) -> i32 {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::set_characterLimit(System.Int32)")]
    pub fn set_character_limit(&self, value: i32) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_canGetSelection")]
    pub fn get_can_get_selection(&self) -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_canSetSelection")]
    pub fn get_can_set_selection(&self) -> bool {}

    #[unity_method(name = "get_selection")]
    pub fn get_selection(&self) -> RangeInt {}

    #[unity_method(name = "set_selection")]
    pub fn set_selection(&self, value: RangeInt) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_type")]
    pub fn get_type(&self) -> TouchScreenKeyboardType {}

    #[unity_method(name = "get_targetDisplay")]
    pub fn get_target_display(&self) -> i32 {}

    #[unity_method(name = "set_targetDisplay")]
    pub fn set_target_display(&self, value: i32) {}

    #[unity_method(name = "get_area", static)]
    pub fn get_area() -> Rect {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::get_visible")]
    pub fn get_visible() -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::Internal_Destroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::IsRequiredToForceOpen")]
    pub fn is_required_to_force_open() -> bool {}

    #[unity_method(name = "Open", static)]
    pub fn open(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool, secure: bool, alert: bool, text_placeholder: &str, character_limit: i32) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_1(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool, secure: bool, alert: bool, text_placeholder: &str) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_2(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool, secure: bool, alert: bool) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_3(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool, secure: bool) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_4(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool, multiline: bool) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_5(text: &str, keyboard_type: TouchScreenKeyboardType, autocorrection: bool) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_6(text: &str, keyboard_type: TouchScreenKeyboardType) -> Option<TouchScreenKeyboard> {}

    #[unity_method(name = "Open", static)]
    pub fn open_7(text: &str) -> Option<TouchScreenKeyboard> {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::GetDone(System.IntPtr)")]
    pub fn get_done_1(ptr: isize) -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::GetWasCanceled(System.IntPtr)")]
    pub fn get_was_canceled_1(ptr: isize) -> bool {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::GetSelection(System.Int32&,System.Int32&)")]
    pub fn get_selection_1(start: &mut i32, length: &mut i32) {}

    #[unity_icall("UnityEngine.TouchScreenKeyboard::SetSelection(System.Int32,System.Int32)")]
    pub fn set_selection_1(start: i32, length: i32) {}

}
