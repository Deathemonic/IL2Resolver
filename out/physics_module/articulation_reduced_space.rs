#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ArticulationReducedSpace", namespace = "UnityEngine", value_type)]
pub struct ArticulationReducedSpace {
    pub x: *mut c_void,
    pub dof_count: i32,
}

#[unity_impl]
impl ArticulationReducedSpace {
    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> f32 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: f32) {}

}
