#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::event_trigger_type::EventTriggerType;
use super::trigger_event::TriggerEvent;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Entry", namespace = "UnityEngine.EventSystems")]
pub struct Entry(pub *mut c_void);
