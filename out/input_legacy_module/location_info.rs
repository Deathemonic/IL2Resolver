#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "LocationInfo", namespace = "UnityEngine", value_type)]
pub struct LocationInfo {
    pub m_timestamp: f64,
    pub m_latitude: f32,
    pub m_longitude: f32,
    pub m_altitude: f32,
    pub m_horizontal_accuracy: f32,
    pub m_vertical_accuracy: f32,
}

#[unity_impl]
impl LocationInfo {
    #[unity_method(name = "get_latitude")]
    pub fn get_latitude(&self) -> f32 {}

    #[unity_method(name = "get_longitude")]
    pub fn get_longitude(&self) -> f32 {}

    #[unity_method(name = "get_altitude")]
    pub fn get_altitude(&self) -> f32 {}

    #[unity_method(name = "get_horizontalAccuracy")]
    pub fn get_horizontal_accuracy(&self) -> f32 {}

    #[unity_method(name = "get_verticalAccuracy")]
    pub fn get_vertical_accuracy(&self) -> f32 {}

    #[unity_method(name = "get_timestamp")]
    pub fn get_timestamp(&self) -> f64 {}

}
