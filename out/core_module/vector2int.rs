#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Vector2Int", namespace = "UnityEngine", value_type)]
pub struct Vector2Int {
    pub m_x: i32,
    pub m_y: i32,
}

#[unity_impl]
impl Vector2Int {
    #[unity_method(name = "get_x")]
    pub fn get_x(&self) -> i32 {}

    #[unity_method(name = "set_x")]
    pub fn set_x(&self, value: i32) {}

    #[unity_method(name = "get_y")]
    pub fn get_y(&self) -> i32 {}

    #[unity_method(name = "set_y")]
    pub fn set_y(&self, value: i32) {}

    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> i32 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: i32) {}

    #[unity_method(name = "get_magnitude")]
    pub fn get_magnitude(&self) -> f32 {}

    #[unity_method(name = "get_sqrMagnitude")]
    pub fn get_sqr_magnitude(&self) -> i32 {}

    #[unity_method(name = "get_zero", static)]
    pub fn get_zero() -> Vector2Int {}

    #[unity_method(name = "get_one", static)]
    pub fn get_one() -> Vector2Int {}

    #[unity_method(name = "get_up", static)]
    pub fn get_up() -> Vector2Int {}

    #[unity_method(name = "get_down", static)]
    pub fn get_down() -> Vector2Int {}

    #[unity_method(name = "get_left", static)]
    pub fn get_left() -> Vector2Int {}

    #[unity_method(name = "get_right", static)]
    pub fn get_right() -> Vector2Int {}

    #[unity_method(name = "Set")]
    pub fn set(&self, x: i32, y: i32) {}

    #[unity_method(name = "Distance", static)]
    pub fn distance(a: Vector2Int, b: Vector2Int) -> f32 {}

    #[unity_method(name = "Min", static)]
    pub fn min(lhs: Vector2Int, rhs: Vector2Int) -> Vector2Int {}

    #[unity_method(name = "Max", static)]
    pub fn max(lhs: Vector2Int, rhs: Vector2Int) -> Vector2Int {}

    #[unity_method(name = "Scale", static)]
    pub fn scale(a: Vector2Int, b: Vector2Int) -> Vector2Int {}

    #[unity_method(name = "Scale")]
    pub fn scale_1(&self, scale: Vector2Int) {}

    #[unity_method(name = "Clamp")]
    pub fn clamp(&self, min: Vector2Int, max: Vector2Int) {}

    #[unity_method(name = "FloorToInt", static)]
    pub fn floor_to_int(v: Vector2) -> Vector2Int {}

    #[unity_method(name = "CeilToInt", static)]
    pub fn ceil_to_int(v: Vector2) -> Vector2Int {}

    #[unity_method(name = "RoundToInt", static)]
    pub fn round_to_int(v: Vector2) -> Vector2Int {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: Vector2Int) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

}
