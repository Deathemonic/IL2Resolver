#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{Exception, SystemObject};
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::log_option::LogOption;
use super::log_type::LogType;
use super::object::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Debug", namespace = "UnityEngine")]
pub struct Debug(pub *mut c_void);

#[unity_impl]
impl Debug {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_unityLogger", static)]
    pub fn get_unity_logger() -> *mut c_void {}

    #[unity_icall("UnityEngine.Debug::get_developerConsoleVisible")]
    pub fn get_developer_console_visible() -> bool {}

    #[unity_icall("UnityEngine.Debug::set_developerConsoleVisible(System.Boolean)")]
    pub fn set_developer_console_visible(value: bool) {}

    #[unity_icall("UnityEngine.Debug::get_isDebugBuild")]
    pub fn get_is_debug_build() -> bool {}

    #[unity_method(name = "get_logger", static)]
    pub fn get_logger() -> *mut c_void {}

    #[unity_icall("UnityEngine.Debug::DrawLine(Vector3,Vector3,Color,System.Single,System.Boolean)")]
    pub fn draw_ray(start: Vector3, end: Vector3, color: Color, duration: f32, depth_test: bool) {}

    #[unity_icall("UnityEngine.Debug::Break")]
    pub fn break_value() {}

    #[unity_icall("UnityEngine.Debug::DebugBreak")]
    pub fn debug_break() {}

    #[unity_icall("UnityEngine.Debug::ExtractStackTraceNoAlloc(System.Byte*,System.Int32,System.String)")]
    pub fn extract_stack_trace_no_alloc(buffer: *mut u8, buffer_max: i32, project_folder: &str) -> i32 {}

    #[unity_method(name = "Log", static)]
    pub fn log(message: Option<SystemObject>) {}

    #[unity_method(name = "Log", static)]
    pub fn log_1(message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogFormat", static)]
    pub fn log_format(format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogFormat", static)]
    pub fn log_format_1(context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogFormat", static)]
    pub fn log_format_2(log_type: LogType, log_options: LogOption, context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogError", static)]
    pub fn log_error(message: Option<SystemObject>) {}

    #[unity_method(name = "LogError", static)]
    pub fn log_error_1(message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogErrorFormat", static)]
    pub fn log_error_format(format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogErrorFormat", static)]
    pub fn log_error_format_1(context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_icall("UnityEngine.Debug::ClearDeveloperConsole")]
    pub fn clear_developer_console() {}

    #[unity_method(name = "LogException", static)]
    pub fn log_exception(exception: Option<Exception>) {}

    #[unity_method(name = "LogException", static)]
    pub fn log_exception_1(exception: Option<Exception>, context: Option<Object>) {}

    #[unity_method(name = "LogWarning", static)]
    pub fn log_warning(message: Option<SystemObject>) {}

    #[unity_method(name = "LogWarning", static)]
    pub fn log_warning_1(message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogWarningFormat", static)]
    pub fn log_warning_format(format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogWarningFormat", static)]
    pub fn log_warning_format_1(context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert(condition: bool) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_1(condition: bool, context: Option<Object>) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_2(condition: bool, message: Option<SystemObject>) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_3(condition: bool, message: &str) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_4(condition: bool, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_5(condition: bool, message: &str, context: Option<Object>) {}

    #[unity_method(name = "AssertFormat", static)]
    pub fn assert_format(condition: bool, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "AssertFormat", static)]
    pub fn assert_format_1(condition: bool, context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogAssertion", static)]
    pub fn log_assertion(message: Option<SystemObject>) {}

    #[unity_method(name = "LogAssertion", static)]
    pub fn log_assertion_1(message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogAssertionFormat", static)]
    pub fn log_assertion_format(format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogAssertionFormat", static)]
    pub fn log_assertion_format_1(context: Option<Object>, format: &str, args: Array<SystemObject>) {}

    #[unity_icall("UnityEngine.Debug::OpenConsoleFile")]
    pub fn open_console_file() {}

    #[unity_method(name = "Assert", static)]
    pub fn assert_6(condition: bool, format: &str, args: Array<SystemObject>) {}

    #[unity_icall("UnityEngine.Debug::DrawLine_Injected(Vector3&,Vector3&,Color&,System.Single,System.Boolean)")]
    pub fn draw_line(start: &mut Vector3, end: &mut Vector3, color: &mut Color, duration: f32, depth_test: bool) {}

}
