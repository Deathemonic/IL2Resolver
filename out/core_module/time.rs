#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Time", namespace = "UnityEngine")]
pub struct Time(pub *mut c_void);

#[unity_impl]
impl Time {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Time::get_time")]
    pub fn get_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_timeAsDouble")]
    pub fn get_time_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_timeSinceLevelLoad")]
    pub fn get_time_since_level_load() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_timeSinceLevelLoadAsDouble")]
    pub fn get_time_since_level_load_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_deltaTime")]
    pub fn get_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_fixedTime")]
    pub fn get_fixed_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_fixedTimeAsDouble")]
    pub fn get_fixed_time_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_unscaledTime")]
    pub fn get_unscaled_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_unscaledTimeAsDouble")]
    pub fn get_unscaled_time_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_fixedUnscaledTime")]
    pub fn get_fixed_unscaled_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_fixedUnscaledTimeAsDouble")]
    pub fn get_fixed_unscaled_time_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_unscaledDeltaTime")]
    pub fn get_unscaled_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_fixedUnscaledDeltaTime")]
    pub fn get_fixed_unscaled_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_fixedDeltaTime")]
    pub fn get_fixed_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::set_fixedDeltaTime(System.Single)")]
    pub fn set_fixed_delta_time(value: f32) {}

    #[unity_icall("UnityEngine.Time::get_maximumDeltaTime")]
    pub fn get_maximum_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::set_maximumDeltaTime(System.Single)")]
    pub fn set_maximum_delta_time(value: f32) {}

    #[unity_icall("UnityEngine.Time::get_smoothDeltaTime")]
    pub fn get_smooth_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_maximumParticleDeltaTime")]
    pub fn get_maximum_particle_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::set_maximumParticleDeltaTime(System.Single)")]
    pub fn set_maximum_particle_delta_time(value: f32) {}

    #[unity_icall("UnityEngine.Time::get_timeScale")]
    pub fn get_time_scale() -> f32 {}

    #[unity_icall("UnityEngine.Time::set_timeScale(System.Single)")]
    pub fn set_time_scale(value: f32) {}

    #[unity_icall("UnityEngine.Time::get_frameCount")]
    pub fn get_frame_count() -> i32 {}

    #[unity_icall("UnityEngine.Time::get_renderedFrameCount")]
    pub fn get_rendered_frame_count() -> i32 {}

    #[unity_icall("UnityEngine.Time::get_realtimeSinceStartup")]
    pub fn get_realtime_since_startup() -> f32 {}

    #[unity_icall("UnityEngine.Time::get_realtimeSinceStartupAsDouble")]
    pub fn get_realtime_since_startup_as_double() -> f64 {}

    #[unity_icall("UnityEngine.Time::get_captureDeltaTime")]
    pub fn get_capture_delta_time() -> f32 {}

    #[unity_icall("UnityEngine.Time::set_captureDeltaTime(System.Single)")]
    pub fn set_capture_delta_time(value: f32) {}

    #[unity_method(name = "get_captureFramerate", static)]
    pub fn get_capture_framerate() -> i32 {}

    #[unity_method(name = "set_captureFramerate", static)]
    pub fn set_capture_framerate(value: i32) {}

    #[unity_icall("UnityEngine.Time::get_inFixedTimeStep")]
    pub fn get_in_fixed_time_step() -> bool {}

}
