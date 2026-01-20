#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{UnityEvent, UnityEventBase};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "OnChangeEvent", namespace = "UnityEngine.UI", inherit = "UnityEvent,UnityEventBase")]
pub struct OnChangeEvent(pub *mut c_void);
