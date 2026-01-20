#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::vector2int::Vector2Int;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PositionEnumerator", namespace = "UnityEngine", value_type)]
pub struct PositionEnumerator {
    pub _min: Vector2Int,
    pub _max: Vector2Int,
    pub _current: Vector2Int,
}

#[unity_impl]
impl PositionEnumerator {
    #[unity_method(name = "get_Current")]
    pub fn get_current(&self) -> Vector2Int {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> PositionEnumerator {}

    #[unity_method(name = "MoveNext")]
    pub fn move_next(&self) -> bool {}

    #[unity_method(name = "Reset")]
    pub fn reset(&self) {}

}
