#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RefreshRate", namespace = "UnityEngine", value_type)]
pub struct RefreshRate {
    pub numerator: u32,
    pub denominator: u32,
}

#[unity_impl]
impl RefreshRate {
    #[unity_method(name = "get_value")]
    pub fn get_value(&self) -> f64 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: RefreshRate) -> bool {}

}
