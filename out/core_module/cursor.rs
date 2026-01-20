#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use super::cursor_lock_mode::CursorLockMode;
use super::cursor_mode::CursorMode;
use super::texture2d::Texture2D;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Cursor", namespace = "UnityEngine")]
pub struct Cursor(pub *mut c_void);

#[unity_impl]
impl Cursor {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Cursor::get_visible")]
    pub fn get_visible() -> bool {}

    #[unity_icall("UnityEngine.Cursor::set_visible(System.Boolean)")]
    pub fn set_visible(value: bool) {}

    #[unity_icall("UnityEngine.Cursor::get_lockState")]
    pub fn get_lock_state() -> CursorLockMode {}

    #[unity_icall("UnityEngine.Cursor::set_lockState(CursorLockMode)")]
    pub fn set_lock_state(value: CursorLockMode) {}

    #[unity_icall("UnityEngine.Cursor::SetCursor_Injected(Texture2D,Vector2&,CursorMode)")]
    pub fn set_cursor(texture: Option<Texture2D>, hotspot: &mut Vector2, cursor_mode: CursorMode) {}

}
