#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector3};
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Rect", namespace = "UnityEngine", value_type)]
pub struct Rect {
    pub m_x_min: f32,
    pub m_y_min: f32,
    pub m_width: f32,
    pub m_height: f32,
}

#[unity_impl]
impl Rect {
    #[unity_method(name = "get_zero", static)]
    pub fn get_zero() -> Rect {}

    #[unity_method(name = "get_x")]
    pub fn get_x(&self) -> f32 {}

    #[unity_method(name = "set_x")]
    pub fn set_x(&self, value: f32) {}

    #[unity_method(name = "get_y")]
    pub fn get_y(&self) -> f32 {}

    #[unity_method(name = "set_y")]
    pub fn set_y(&self, value: f32) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Vector2 {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Vector2) {}

    #[unity_method(name = "get_center")]
    pub fn get_center(&self) -> Vector2 {}

    #[unity_method(name = "set_center")]
    pub fn set_center(&self, value: Vector2) {}

    #[unity_method(name = "get_min")]
    pub fn get_min(&self) -> Vector2 {}

    #[unity_method(name = "set_min")]
    pub fn set_min(&self, value: Vector2) {}

    #[unity_method(name = "get_max")]
    pub fn get_max(&self) -> Vector2 {}

    #[unity_method(name = "set_max")]
    pub fn set_max(&self, value: Vector2) {}

    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> f32 {}

    #[unity_method(name = "set_width")]
    pub fn set_width(&self, value: f32) {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> f32 {}

    #[unity_method(name = "set_height")]
    pub fn set_height(&self, value: f32) {}

    #[unity_method(name = "get_size")]
    pub fn get_size(&self) -> Vector2 {}

    #[unity_method(name = "set_size")]
    pub fn set_size(&self, value: Vector2) {}

    #[unity_method(name = "get_xMin")]
    pub fn get_x_min(&self) -> f32 {}

    #[unity_method(name = "set_xMin")]
    pub fn set_x_min(&self, value: f32) {}

    #[unity_method(name = "get_yMin")]
    pub fn get_y_min(&self) -> f32 {}

    #[unity_method(name = "set_yMin")]
    pub fn set_y_min(&self, value: f32) {}

    #[unity_method(name = "get_xMax")]
    pub fn get_x_max(&self) -> f32 {}

    #[unity_method(name = "set_xMax")]
    pub fn set_x_max(&self, value: f32) {}

    #[unity_method(name = "get_yMax")]
    pub fn get_y_max(&self) -> f32 {}

    #[unity_method(name = "set_yMax")]
    pub fn set_y_max(&self, value: f32) {}

    #[unity_method(name = "get_left")]
    pub fn get_left(&self) -> f32 {}

    #[unity_method(name = "get_right")]
    pub fn get_right(&self) -> f32 {}

    #[unity_method(name = "get_top")]
    pub fn get_top(&self) -> f32 {}

    #[unity_method(name = "get_bottom")]
    pub fn get_bottom(&self) -> f32 {}

    #[unity_method(name = "MinMaxRect", static)]
    pub fn min_max_rect(xmin: f32, ymin: f32, xmax: f32, ymax: f32) -> Rect {}

    #[unity_method(name = "Set")]
    pub fn set(&self, x: f32, y: f32, width: f32, height: f32) {}

    #[unity_method(name = "Contains")]
    pub fn contains(&self, point: Vector2) -> bool {}

    #[unity_method(name = "Contains")]
    pub fn contains_1(&self, point: Vector3) -> bool {}

    #[unity_method(name = "Contains")]
    pub fn contains_2(&self, point: Vector3, allow_inverse: bool) -> bool {}

    #[unity_method(name = "Overlaps")]
    pub fn overlaps(&self, other: Rect) -> bool {}

    #[unity_method(name = "Overlaps")]
    pub fn overlaps_1(&self, other: Rect, allow_inverse: bool) -> bool {}

    #[unity_method(name = "NormalizedToPoint", static)]
    pub fn normalized_to_point(rectangle: Rect, normalized_rect_coordinates: Vector2) -> Vector2 {}

    #[unity_method(name = "PointToNormalized", static)]
    pub fn point_to_normalized(rectangle: Rect, point: Vector2) -> Vector2 {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: Rect) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

}
