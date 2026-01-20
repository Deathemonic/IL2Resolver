#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::location_info::LocationInfo;
use super::location_service_status::LocationServiceStatus;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.InputLegacyModule", class = "LocationService", namespace = "UnityEngine")]
pub struct LocationService(pub *mut c_void);

#[unity_impl]
impl LocationService {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_isEnabledByUser")]
    pub fn get_is_enabled_by_user(&self) -> bool {}

    #[unity_method(name = "get_status")]
    pub fn get_status(&self) -> LocationServiceStatus {}

    #[unity_method(name = "get_lastData")]
    pub fn get_last_data(&self) -> LocationInfo {}

    #[unity_icall("UnityEngine.LocationService::IsServiceEnabledByUser")]
    pub fn is_service_enabled_by_user() -> bool {}

    #[unity_icall("UnityEngine.LocationService::GetLocationStatus")]
    pub fn get_location_status() -> LocationServiceStatus {}

    #[unity_icall("UnityEngine.LocationService::GetLastLocation")]
    pub fn get_last_location() -> LocationInfo {}

    #[unity_icall("UnityEngine.LocationService::SetDesiredAccuracy(System.Single)")]
    pub fn set_desired_accuracy(value: f32) {}

    #[unity_icall("UnityEngine.LocationService::SetDistanceFilter(System.Single)")]
    pub fn set_distance_filter(value: f32) {}

    #[unity_icall("UnityEngine.LocationService::StartUpdatingLocation")]
    pub fn start_updating_location() {}

    #[unity_icall("UnityEngine.LocationService::StopUpdatingLocation")]
    pub fn stop_updating_location() {}

    #[unity_icall("UnityEngine.LocationService::GetLastHeading")]
    pub fn get_last_heading() -> *mut c_void {}

    #[unity_icall("UnityEngine.LocationService::IsHeadingUpdatesEnabled")]
    pub fn is_heading_updates_enabled() -> bool {}

    #[unity_icall("UnityEngine.LocationService::SetHeadingUpdatesEnabled(System.Boolean)")]
    pub fn set_heading_updates_enabled(value: bool) {}

    #[unity_icall("UnityEngine.LocationService::SetDesiredAccuracy(System.Single)")]
    pub fn start(value: f32) {}

    #[unity_icall("UnityEngine.LocationService::SetDesiredAccuracy(System.Single)")]
    pub fn start_1(value: f32) {}

    #[unity_icall("UnityEngine.LocationService::SetDesiredAccuracy(System.Single)")]
    pub fn start_2(value: f32) {}

    #[unity_icall("UnityEngine.LocationService::StopUpdatingLocation")]
    pub fn stop() {}

    #[unity_icall("UnityEngine.LocationService::GetLastLocation_Injected(LocationInfo&)")]
    pub fn get_last_location_1(ret: &mut LocationInfo) {}

    #[unity_icall("UnityEngine.LocationService::GetLastHeading_Injected(LocationService.HeadingInfo&)")]
    pub fn get_last_heading_1(ret: &mut *mut c_void) {}

}
