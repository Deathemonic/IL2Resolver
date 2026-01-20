#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use super::position_enumerator::PositionEnumerator;
use super::vector2int::Vector2Int;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RectInt", namespace = "UnityEngine", value_type)]
pub struct RectInt {
    pub m_x_min: i32,
    pub m_y_min: i32,
    pub m_width: i32,
    pub m_height: i32,
}

#[unity_impl]
impl RectInt {
    #[unity_method(name = "get_x")]
    pub fn get_x(&self) -> i32 {}

    #[unity_method(name = "set_x")]
    pub fn set_x(&self, value: i32) {}

    #[unity_method(name = "get_y")]
    pub fn get_y(&self) -> i32 {}

    #[unity_method(name = "set_y")]
    pub fn set_y(&self, value: i32) {}

    #[unity_method(name = "get_center")]
    pub fn get_center(&self) -> Vector2 {}

    #[unity_method(name = "get_min")]
    pub fn get_min(&self) -> Vector2Int {}

    #[unity_method(name = "set_min")]
    pub fn set_min(&self, value: Vector2Int) {}

    #[unity_method(name = "get_max")]
    pub fn get_max(&self) -> Vector2Int {}

    #[unity_method(name = "set_max")]
    pub fn set_max(&self, value: Vector2Int) {}

    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_method(name = "set_width")]
    pub fn set_width(&self, value: i32) {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_method(name = "set_height")]
    pub fn set_height(&self, value: i32) {}

    #[unity_method(name = "get_xMin")]
    pub fn get_x_min(&self) -> i32 {}

    #[unity_method(name = "set_xMin")]
    pub fn set_x_min(&self, value: i32) {}

    #[unity_method(name = "get_yMin")]
    pub fn get_y_min(&self) -> i32 {}

    #[unity_method(name = "set_yMin")]
    pub fn set_y_min(&self, value: i32) {}

    #[unity_method(name = "get_xMax")]
    pub fn get_x_max(&self) -> i32 {}

    #[unity_method(name = "set_xMax")]
    pub fn set_x_max(&self, value: i32) {}

    #[unity_method(name = "get_yMax")]
    pub fn get_y_max(&self) -> i32 {}

    #[unity_method(name = "set_yMax")]
    pub fn set_y_max(&self, value: i32) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Vector2Int {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Vector2Int) {}

    #[unity_method(name = "get_size")]
    pub fn get_size(&self) -> Vector2Int {}

    #[unity_method(name = "set_size")]
    pub fn set_size(&self, value: Vector2Int) {}

    #[unity_method(name = "get_allPositionsWithin")]
    pub fn get_all_positions_within(&self) -> PositionEnumerator {}

    #[unity_method(name = "SetMinMax")]
    pub fn set_min_max(&self, min_position: Vector2Int, max_position: Vector2Int) {}

    #[unity_method(name = "ClampToBounds")]
    pub fn clamp_to_bounds(&self, bounds: RectInt) {}

    #[unity_method(name = "Contains")]
    pub fn contains(&self, position: Vector2Int) -> bool {}

    #[unity_method(name = "Overlaps")]
    pub fn overlaps(&self, other: RectInt) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: RectInt) -> bool {}

}
