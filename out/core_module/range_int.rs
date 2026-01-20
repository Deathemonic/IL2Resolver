#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RangeInt", namespace = "UnityEngine", value_type)]
pub struct RangeInt {
    pub start: i32,
    pub length: i32,
}

#[unity_impl]
impl RangeInt {
    #[unity_method(name = "get_end")]
    pub fn get_end(&self) -> i32 {}

}
