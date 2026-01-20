#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector2, Vector3};
use crate::mscorlib::{SystemObject, SystemString, SystemType};
use super::event::Event;
use super::focus_type::FocusType;
use crate::core_module::{Object, Rect, ScriptableObject};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUIUtility", namespace = "UnityEngine")]
pub struct GUIUtility(pub *mut c_void);

#[unity_impl]
impl GUIUtility {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.GUIUtility::get_hasModalWindow")]
    pub fn get_has_modal_window() -> bool {}

    #[unity_icall("UnityEngine.GUIUtility::get_systemCopyBuffer")]
    pub fn get_system_copy_buffer() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.GUIUtility::set_systemCopyBuffer(System.String)")]
    pub fn set_system_copy_buffer(value: &str) {}

    #[unity_method(name = "get_hotControl", static)]
    pub fn get_hot_control() -> i32 {}

    #[unity_method(name = "set_hotControl", static)]
    pub fn set_hot_control(value: i32) {}

    #[unity_method(name = "get_keyboardControl", static)]
    pub fn get_keyboard_control() -> i32 {}

    #[unity_method(name = "set_keyboardControl", static)]
    pub fn set_keyboard_control(value: i32) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetControlID(System.Int32,FocusType,Rect)")]
    pub fn internal_get_control_id(hint: i32, focus_type: FocusType, rect: Rect) -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::BeginContainerFromOwner(ScriptableObject)")]
    pub fn begin_container_from_owner(owner: Option<ScriptableObject>) {}

    #[unity_icall("UnityEngine.GUIUtility::BeginContainer(ObjectGUIState)")]
    pub fn begin_container(object_gui_state: *mut c_void) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_EndContainer")]
    pub fn internal_end_container() {}

    #[unity_icall("UnityEngine.GUIUtility::GetPermanentControlID")]
    pub fn get_permanent_control_id() -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::CheckForTabEvent(Event)")]
    pub fn check_for_tab_event(evt: Option<Event>) -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::SetKeyboardControlToFirstControlId")]
    pub fn set_keyboard_control_to_first_control_id() {}

    #[unity_icall("UnityEngine.GUIUtility::SetKeyboardControlToLastControlId")]
    pub fn set_keyboard_control_to_last_control_id() {}

    #[unity_icall("UnityEngine.GUIUtility::HasFocusableControls")]
    pub fn has_focusable_controls() -> bool {}

    #[unity_icall("UnityEngine.GUIUtility::OwnsId(System.Int32)")]
    pub fn owns_id(id: i32) -> bool {}

    #[unity_icall("UnityEngine.GUIUtility::GetChanged")]
    pub fn get_changed() -> bool {}

    #[unity_icall("UnityEngine.GUIUtility::SetChanged(System.Boolean)")]
    pub fn set_changed(changed: bool) {}

    #[unity_icall("UnityEngine.GUIUtility::SetDidGUIWindowsEatLastEvent(System.Boolean)")]
    pub fn set_did_gui_windows_eat_last_event(value: bool) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetHotControl")]
    pub fn internal_get_hot_control() -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetKeyboardControl")]
    pub fn internal_get_keyboard_control() -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_SetHotControl(System.Int32)")]
    pub fn internal_set_hot_control(value: i32) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_SetKeyboardControl(System.Int32)")]
    pub fn internal_set_keyboard_control(value: i32) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetDefaultSkin(System.Int32)")]
    pub fn internal_get_default_skin(skin_mode: i32) -> Option<SystemObject> {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetBuiltinSkin(System.Int32)")]
    pub fn internal_get_builtin_skin(skin: i32) -> Option<Object> {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_ExitGUI")]
    pub fn internal_exit_gui() {}

    #[unity_method(name = "GetStateObject", static)]
    pub fn get_state_object(t: Option<SystemType>, control_id: i32) -> Option<SystemObject> {}

    #[unity_method(name = "QueryStateObject", static)]
    pub fn query_state_object(t: Option<SystemType>, control_id: i32) -> Option<SystemObject> {}

    #[unity_method(name = "ExitGUI", static)]
    pub fn exit_gui() {}

    #[unity_method(name = "RotateAroundPivot", static)]
    pub fn rotate_around_pivot(angle: f32, pivot_point: Vector2) {}

    #[unity_method(name = "ScaleAroundPivot", static)]
    pub fn scale_around_pivot(scale: Vector2, pivot_point: Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::get_s_EditorScreenPointOffset_Injected(Vector2&)")]
    pub fn get_s_editor_screen_point_offset(ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::set_s_EditorScreenPointOffset_Injected(Vector2&)")]
    pub fn set_s_editor_screen_point_offset(value: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_GetControlID_Injected(System.Int32,FocusType,Rect&)")]
    pub fn internal_get_control_id_1(hint: i32, focus_type: FocusType, rect: &mut Rect) -> i32 {}

    #[unity_icall("UnityEngine.GUIUtility::AlignRectToDevice_Injected(Rect&,System.Int32&,System.Int32&,Rect&)")]
    pub fn align_rect_to_device(rect: &mut Rect, width_in_pixels: &mut i32, height_in_pixels: &mut i32, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.GUIUtility::get_compositionCursorPos_Injected(Vector2&)")]
    pub fn get_composition_cursor_pos(ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::set_compositionCursorPos_Injected(Vector2&)")]
    pub fn set_composition_cursor_pos(value: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::Internal_MultiplyPoint_Injected(Vector3&,Matrix4x4&,Vector3&)")]
    pub fn internal_multiply_point(point: &mut Vector3, transform: &mut Matrix4x4, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.GUIUtility::InternalWindowToScreenPoint_Injected(Vector2&,Vector2&)")]
    pub fn internal_window_to_screen_point(window_point: &mut Vector2, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.GUIUtility::InternalScreenToWindowPoint_Injected(Vector2&,Vector2&)")]
    pub fn internal_screen_to_window_point(screen_point: &mut Vector2, ret: &mut Vector2) {}

}
