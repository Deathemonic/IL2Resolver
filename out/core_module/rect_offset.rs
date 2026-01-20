#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::rect::Rect;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RectOffset", namespace = "UnityEngine")]
pub struct RectOffset(pub *mut c_void);

#[unity_impl]
impl RectOffset {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(left: i32, right: i32, top: i32, bottom: i32) -> Option<Self> {}

    #[unity_icall("UnityEngine.RectOffset::get_left")]
    pub fn get_left(&self) -> i32 {}

    #[unity_icall("UnityEngine.RectOffset::set_left(System.Int32)")]
    pub fn set_left(&self, value: i32) {}

    #[unity_icall("UnityEngine.RectOffset::get_right")]
    pub fn get_right(&self) -> i32 {}

    #[unity_icall("UnityEngine.RectOffset::set_right(System.Int32)")]
    pub fn set_right(&self, value: i32) {}

    #[unity_icall("UnityEngine.RectOffset::get_top")]
    pub fn get_top(&self) -> i32 {}

    #[unity_icall("UnityEngine.RectOffset::set_top(System.Int32)")]
    pub fn set_top(&self, value: i32) {}

    #[unity_icall("UnityEngine.RectOffset::get_bottom")]
    pub fn get_bottom(&self) -> i32 {}

    #[unity_icall("UnityEngine.RectOffset::set_bottom(System.Int32)")]
    pub fn set_bottom(&self, value: i32) {}

    #[unity_icall("UnityEngine.RectOffset::get_horizontal")]
    pub fn get_horizontal(&self) -> i32 {}

    #[unity_icall("UnityEngine.RectOffset::get_vertical")]
    pub fn get_vertical(&self) -> i32 {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.RectOffset::InternalCreate")]
    pub fn internal_create() -> isize {}

    #[unity_icall("UnityEngine.RectOffset::InternalDestroy(System.IntPtr)")]
    pub fn internal_destroy(ptr: isize) {}

    #[unity_icall("UnityEngine.RectOffset::Add_Injected(Rect&,Rect&)")]
    pub fn add(&self, rect: Rect) -> Rect {}

    #[unity_icall("UnityEngine.RectOffset::Remove_Injected(Rect&,Rect&)")]
    pub fn remove(&self, rect: Rect) -> Rect {}

}
