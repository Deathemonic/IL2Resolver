#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GPUFence", namespace = "UnityEngine.Rendering", value_type)]
pub struct GPUFence {
}

#[unity_impl]
impl GPUFence {
    #[unity_method(name = "get_passed")]
    pub fn get_passed(&self) -> bool {}

}
