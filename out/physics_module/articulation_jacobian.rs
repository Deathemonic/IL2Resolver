#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::collections::{List};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ArticulationJacobian", namespace = "UnityEngine", value_type)]
pub struct ArticulationJacobian {
    pub rows_count: i32,
    pub cols_count: i32,
    pub matrix_data: List<f32>,
}

#[unity_impl]
impl ArticulationJacobian {
    #[unity_method(name = "get_Item")]
    pub fn get_item(&self) -> f32 {}

    #[unity_method(name = "set_Item")]
    pub fn set_item(&self, value: f32) {}

    #[unity_method(name = "get_rows")]
    pub fn get_rows(&self) -> i32 {}

    #[unity_method(name = "set_rows")]
    pub fn set_rows(&self, value: i32) {}

    #[unity_method(name = "get_columns")]
    pub fn get_columns(&self) -> i32 {}

    #[unity_method(name = "set_columns")]
    pub fn set_columns(&self, value: i32) {}

    #[unity_method(name = "get_elements")]
    pub fn get_elements(&self) -> List<f32> {}

    #[unity_method(name = "set_elements")]
    pub fn set_elements(&self, value: List<f32>) {}

}
