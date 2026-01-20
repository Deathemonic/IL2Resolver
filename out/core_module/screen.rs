#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array, List};
use super::async_operation::AsyncOperation;
use super::display_info::DisplayInfo;
use super::full_screen_mode::FullScreenMode;
use super::rect::Rect;
use super::resolution::Resolution;
use super::screen_orientation::ScreenOrientation;
use super::vector2int::Vector2Int;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Screen", namespace = "UnityEngine")]
pub struct Screen(pub *mut c_void);

#[unity_impl]
impl Screen {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Screen::get_width")]
    pub fn get_width() -> i32 {}

    #[unity_icall("UnityEngine.Screen::get_height")]
    pub fn get_height() -> i32 {}

    #[unity_icall("UnityEngine.Screen::get_dpi")]
    pub fn get_dpi() -> f32 {}

    #[unity_method(name = "get_orientation", static)]
    pub fn get_orientation() -> ScreenOrientation {}

    #[unity_method(name = "set_orientation", static)]
    pub fn set_orientation(value: ScreenOrientation) {}

    #[unity_icall("UnityEngine.Screen::get_sleepTimeout")]
    pub fn get_sleep_timeout() -> i32 {}

    #[unity_icall("UnityEngine.Screen::set_sleepTimeout(System.Int32)")]
    pub fn set_sleep_timeout(value: i32) {}

    #[unity_method(name = "get_autorotateToPortrait", static)]
    pub fn get_autorotate_to_portrait() -> bool {}

    #[unity_method(name = "set_autorotateToPortrait", static)]
    pub fn set_autorotate_to_portrait(value: bool) {}

    #[unity_method(name = "get_autorotateToPortraitUpsideDown", static)]
    pub fn get_autorotate_to_portrait_upside_down() -> bool {}

    #[unity_method(name = "set_autorotateToPortraitUpsideDown", static)]
    pub fn set_autorotate_to_portrait_upside_down(value: bool) {}

    #[unity_method(name = "get_autorotateToLandscapeLeft", static)]
    pub fn get_autorotate_to_landscape_left() -> bool {}

    #[unity_method(name = "set_autorotateToLandscapeLeft", static)]
    pub fn set_autorotate_to_landscape_left(value: bool) {}

    #[unity_method(name = "get_autorotateToLandscapeRight", static)]
    pub fn get_autorotate_to_landscape_right() -> bool {}

    #[unity_method(name = "set_autorotateToLandscapeRight", static)]
    pub fn set_autorotate_to_landscape_right(value: bool) {}

    #[unity_icall("UnityEngine.Screen::get_currentResolution_Injected(Resolution&)")]
    pub fn get_current_resolution(ret: &mut Resolution) {}

    #[unity_icall("UnityEngine.Screen::get_fullScreen")]
    pub fn get_full_screen() -> bool {}

    #[unity_icall("UnityEngine.Screen::set_fullScreen(System.Boolean)")]
    pub fn set_full_screen(value: bool) {}

    #[unity_icall("UnityEngine.Screen::get_fullScreenMode")]
    pub fn get_full_screen_mode() -> FullScreenMode {}

    #[unity_icall("UnityEngine.Screen::set_fullScreenMode(FullScreenMode)")]
    pub fn set_full_screen_mode(value: FullScreenMode) {}

    #[unity_icall("UnityEngine.Screen::get_safeArea_Injected(Rect&)")]
    pub fn get_safe_area(ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Screen::get_cutouts")]
    pub fn get_cutouts() -> Array<Rect> {}

    #[unity_method(name = "get_mainWindowPosition", static)]
    pub fn get_main_window_position() -> Vector2Int {}

    #[unity_method(name = "get_mainWindowDisplayInfo", static)]
    pub fn get_main_window_display_info() -> DisplayInfo {}

    #[unity_icall("UnityEngine.Screen::get_resolutions")]
    pub fn get_resolutions() -> Array<Resolution> {}

    #[unity_icall("UnityEngine.Screen::get_brightness")]
    pub fn get_brightness() -> f32 {}

    #[unity_icall("UnityEngine.Screen::set_brightness(System.Single)")]
    pub fn set_brightness(value: f32) {}

    #[unity_method(name = "get_lockCursor", static)]
    pub fn get_lock_cursor() -> bool {}

    #[unity_method(name = "set_lockCursor", static)]
    pub fn set_lock_cursor(value: bool) {}

    #[unity_icall("UnityEngine.Screen::RequestOrientation(ScreenOrientation)")]
    pub fn request_orientation(orient: ScreenOrientation) {}

    #[unity_icall("UnityEngine.Screen::GetScreenOrientation")]
    pub fn get_screen_orientation() -> ScreenOrientation {}

    #[unity_icall("UnityEngine.Screen::IsOrientationEnabled(EnabledOrientation)")]
    pub fn is_orientation_enabled(orient: *mut c_void) -> bool {}

    #[unity_icall("UnityEngine.Screen::SetOrientationEnabled(EnabledOrientation,System.Boolean)")]
    pub fn set_orientation_enabled(orient: *mut c_void, enabled: bool) {}

    #[unity_icall("UnityEngine.Screen::SetResolution(System.Int32,System.Int32,FullScreenMode,System.Int32)")]
    pub fn set_resolution(width: i32, height: i32, fullscreen_mode: FullScreenMode, preferred_refresh_rate: i32) {}

    #[unity_icall("UnityEngine.Screen::SetResolution(System.Int32,System.Int32,FullScreenMode,System.Int32)")]
    pub fn set_resolution_1(width: i32, height: i32, fullscreen_mode: FullScreenMode, preferred_refresh_rate: i32) {}

    #[unity_icall("UnityEngine.Screen::GetDisplayLayoutImpl(List<DisplayInfo>)")]
    pub fn get_display_layout_impl(display_layout: List<DisplayInfo>) {}

    #[unity_icall("UnityEngine.Screen::MoveMainWindowImpl(DisplayInfo&,Vector2Int)")]
    pub fn move_main_window_impl(display: &mut DisplayInfo, position: Vector2Int) -> Option<AsyncOperation> {}

    #[unity_icall("UnityEngine.Screen::GetMainWindowPosition_Injected(Vector2Int&)")]
    pub fn get_main_window_position_1(ret: &mut Vector2Int) {}

    #[unity_icall("UnityEngine.Screen::GetMainWindowDisplayInfo_Injected(DisplayInfo&)")]
    pub fn get_main_window_display_info_1(ret: &mut DisplayInfo) {}

    #[unity_icall("UnityEngine.Screen::MoveMainWindowImpl_Injected(DisplayInfo&,Vector2Int&)")]
    pub fn move_main_window_impl_1(display: &mut DisplayInfo, position: &mut Vector2Int) -> Option<AsyncOperation> {}

}
