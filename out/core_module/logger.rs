#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{Exception, SystemObject};
use crate::mscorlib::collections::{Array};
use super::log_type::LogType;
use super::object::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Logger", namespace = "UnityEngine")]
pub struct Logger(pub *mut c_void);

#[unity_impl]
impl Logger {
    #[unity_ctor]
    pub fn new(log_handler: *mut c_void) -> Option<Self> {}

    #[unity_method(name = "get_logHandler")]
    pub fn get_log_handler(&self) -> *mut c_void {}

    #[unity_method(name = "set_logHandler")]
    pub fn set_log_handler(&self, value: *mut c_void) {}

    #[unity_method(name = "get_logEnabled")]
    pub fn get_log_enabled(&self) -> bool {}

    #[unity_method(name = "set_logEnabled")]
    pub fn set_log_enabled(&self, value: bool) {}

    #[unity_method(name = "get_filterLogType")]
    pub fn get_filter_log_type(&self) -> LogType {}

    #[unity_method(name = "set_filterLogType")]
    pub fn set_filter_log_type(&self, value: LogType) {}

    #[unity_method(name = "IsLogTypeAllowed")]
    pub fn is_log_type_allowed(&self, log_type: LogType) -> bool {}

    #[unity_method(name = "Log")]
    pub fn log(&self, log_type: LogType, message: Option<SystemObject>) {}

    #[unity_method(name = "Log")]
    pub fn log_1(&self, log_type: LogType, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "Log")]
    pub fn log_2(&self, log_type: LogType, tag: &str, message: Option<SystemObject>) {}

    #[unity_method(name = "Log")]
    pub fn log_3(&self, log_type: LogType, tag: &str, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "Log")]
    pub fn log_4(&self, message: Option<SystemObject>) {}

    #[unity_method(name = "Log")]
    pub fn log_5(&self, tag: &str, message: Option<SystemObject>) {}

    #[unity_method(name = "Log")]
    pub fn log_6(&self, tag: &str, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogWarning")]
    pub fn log_warning(&self, tag: &str, message: Option<SystemObject>) {}

    #[unity_method(name = "LogWarning")]
    pub fn log_warning_1(&self, tag: &str, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogError")]
    pub fn log_error(&self, tag: &str, message: Option<SystemObject>) {}

    #[unity_method(name = "LogError")]
    pub fn log_error_1(&self, tag: &str, message: Option<SystemObject>, context: Option<Object>) {}

    #[unity_method(name = "LogException")]
    pub fn log_exception(&self, exception: Option<Exception>) {}

    #[unity_method(name = "LogException")]
    pub fn log_exception_1(&self, exception: Option<Exception>, context: Option<Object>) {}

    #[unity_method(name = "LogFormat")]
    pub fn log_format(&self, log_type: LogType, format: &str, args: Array<SystemObject>) {}

    #[unity_method(name = "LogFormat")]
    pub fn log_format_1(&self, log_type: LogType, context: Option<Object>, format: &str, args: Array<SystemObject>) {}

}
