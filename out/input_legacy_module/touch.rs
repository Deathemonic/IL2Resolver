#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Vector2};
use super::touch_phase::TouchPhase;
use super::touch_type::TouchType;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "Touch", namespace = "UnityEngine", value_type)]
pub struct Touch {
    pub m_finger_id: i32,
    pub m_position: Vector2,
    pub m_raw_position: Vector2,
    pub m_position_delta: Vector2,
    pub m_time_delta: f32,
    pub m_tap_count: i32,
    pub m_phase: TouchPhase,
    pub m_type: TouchType,
    pub m_pressure: f32,
    pub m_maximum_possible_pressure: f32,
    pub m_radius: f32,
    pub m_radius_variance: f32,
    pub m_altitude_angle: f32,
    pub m_azimuth_angle: f32,
}

#[unity_impl]
impl Touch {
    #[unity_method(name = "get_fingerId")]
    pub fn get_finger_id(&self) -> i32 {}

    #[unity_method(name = "set_fingerId")]
    pub fn set_finger_id(&self, value: i32) {}

    #[unity_method(name = "get_position")]
    pub fn get_position(&self) -> Vector2 {}

    #[unity_method(name = "set_position")]
    pub fn set_position(&self, value: Vector2) {}

    #[unity_method(name = "get_rawPosition")]
    pub fn get_raw_position(&self) -> Vector2 {}

    #[unity_method(name = "set_rawPosition")]
    pub fn set_raw_position(&self, value: Vector2) {}

    #[unity_method(name = "get_deltaPosition")]
    pub fn get_delta_position(&self) -> Vector2 {}

    #[unity_method(name = "set_deltaPosition")]
    pub fn set_delta_position(&self, value: Vector2) {}

    #[unity_method(name = "get_deltaTime")]
    pub fn get_delta_time(&self) -> f32 {}

    #[unity_method(name = "set_deltaTime")]
    pub fn set_delta_time(&self, value: f32) {}

    #[unity_method(name = "get_tapCount")]
    pub fn get_tap_count(&self) -> i32 {}

    #[unity_method(name = "set_tapCount")]
    pub fn set_tap_count(&self, value: i32) {}

    #[unity_method(name = "get_phase")]
    pub fn get_phase(&self) -> TouchPhase {}

    #[unity_method(name = "set_phase")]
    pub fn set_phase(&self, value: TouchPhase) {}

    #[unity_method(name = "get_pressure")]
    pub fn get_pressure(&self) -> f32 {}

    #[unity_method(name = "set_pressure")]
    pub fn set_pressure(&self, value: f32) {}

    #[unity_method(name = "get_maximumPossiblePressure")]
    pub fn get_maximum_possible_pressure(&self) -> f32 {}

    #[unity_method(name = "set_maximumPossiblePressure")]
    pub fn set_maximum_possible_pressure(&self, value: f32) {}

    #[unity_method(name = "get_type")]
    pub fn get_type(&self) -> TouchType {}

    #[unity_method(name = "set_type")]
    pub fn set_type(&self, value: TouchType) {}

    #[unity_method(name = "get_altitudeAngle")]
    pub fn get_altitude_angle(&self) -> f32 {}

    #[unity_method(name = "set_altitudeAngle")]
    pub fn set_altitude_angle(&self, value: f32) {}

    #[unity_method(name = "get_azimuthAngle")]
    pub fn get_azimuth_angle(&self) -> f32 {}

    #[unity_method(name = "set_azimuthAngle")]
    pub fn set_azimuth_angle(&self, value: f32) {}

    #[unity_method(name = "get_radius")]
    pub fn get_radius(&self) -> f32 {}

    #[unity_method(name = "set_radius")]
    pub fn set_radius(&self, value: f32) {}

    #[unity_method(name = "get_radiusVariance")]
    pub fn get_radius_variance(&self) -> f32 {}

    #[unity_method(name = "set_radiusVariance")]
    pub fn set_radius_variance(&self, value: f32) {}

}
