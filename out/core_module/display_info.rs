#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::rect_int::RectInt;
use super::refresh_rate::RefreshRate;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "DisplayInfo", namespace = "UnityEngine", value_type)]
pub struct DisplayInfo {
    pub handle: u64,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: RefreshRate,
    pub work_area: RectInt,
    pub name: Option<SystemString>,
}

#[unity_impl]
impl DisplayInfo {
    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: DisplayInfo) -> bool {}

}
