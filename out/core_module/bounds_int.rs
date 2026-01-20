#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::{SystemObject, SystemString};
use super::position_enumerator::PositionEnumerator;
use super::vector3int::Vector3Int;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "BoundsInt", namespace = "UnityEngine", value_type)]
pub struct BoundsInt {
    pub m_position: Vector3Int,
    pub m_size: Vector3Int,
}

#[unity_impl]
impl BoundsInt {
    #[unity_method(name = "get_x")]
    pub fn get_x(&self) -> i32 {}

    #[unity_method(name = "set_x")]
    pub fn set_x(&self, value: i32) {}

    #[unity_method(name = "get_y")]
    pub fn get_y(&self) -> i32 {}

    #[unity_method(name = "set_y")]
    pub fn set_y(&self, value: i32) {}

    #[unity_method(name = "get_z")]
    pub fn get_z(&self) -> i32 {}

    #[unity_method(name = "set_z")]
    pub fn set_z(&self, value: i32) {}

    #[unity_method(name = "get_center")]
    pub fn get_center(&self) -> Vector3 {}

    #[unity_method(name = "get_min")]
    pub fn get_min(&self) -> Vector3Int {}

    #[unity_method(name = "set_min")]
    pub fn set_min(&self, value: Vector3Int) {}

    #[unity_method(name = "get_max")]
    pub fn get_max(&self) -> Vector3Int {}

    #[unity_method(name = "set_max")]
    pub fn set_max(&self, value: Vector3Int) {}

    #[unity_method(name = "get_xMin")]
    pub fn get_x_min(&self) -> i32 {}

    #[unity_method(name = "set_xMin")]
    pub fn set_x_min(&self, value: i32) {}

    #[unity_method(name = "get_yMin")]
    pub fn get_y_min(&self) -> i32 {}

    #[unity_method(name = "set_yMin")]
    pub fn set_y_min(&self, value: i32) {}

    #[unity_method(name = "get_zMin")]
    pub fn get_z_min(&self) -> i32 {}

    #[unity_method(name = "set_zMin")]
    pub fn set_z_min(&self, value: i32) {}

    #[unity_method(name = "get_xMax")]
    pub fn get_x_max(&self) -> i32 {}

    #[unity_method(name = "set_xMax")]
    pub fn set_x_max(&self, value: i32) {}

    #[unity_method(name = "get_yMax")]
    pub fn get_y_max(&self) -> i32 {}

    #[unity_method(name = "set_yMax")]
    pub fn set_y_max(&self, value: i32) {}

    #[unity_method(name = "get_zMax")]
    pub fn get_z_max(&self) -> i32 {}

    #[unity_method(name = "set_zMax")]
    pub fn set_z_max(&self, value: i32) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Vector3Int {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Vector3Int) {}

    #[unity_method(name = "get_size")]
    pub fn get_size(&self) -> Vector3Int {}

    #[unity_method(name = "set_size")]
    pub fn set_size(&self, value: Vector3Int) {}

    #[unity_method(name = "get_allPositionsWithin")]
    pub fn get_all_positions_within(&self) -> PositionEnumerator {}

    #[unity_method(name = "SetMinMax")]
    pub fn set_min_max(&self, min_position: Vector3Int, max_position: Vector3Int) {}

    #[unity_method(name = "ClampToBounds")]
    pub fn clamp_to_bounds(&self, bounds: BoundsInt) {}

    #[unity_method(name = "Contains")]
    pub fn contains(&self, position: Vector3Int) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_1(&self, format: &str) -> Option<SystemString> {}

    #[unity_method(name = "ToString")]
    pub fn to_string_2(&self, format: &str, format_provider: *mut c_void) -> Option<SystemString> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: BoundsInt) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

}
