#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PropertyName", namespace = "UnityEngine", value_type)]
pub struct PropertyName {
    pub id: i32,
}

#[unity_impl]
impl PropertyName {
    #[unity_method(name = "IsNullOrEmpty", static)]
    pub fn is_null_or_empty(prop: PropertyName) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: PropertyName) -> bool {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
