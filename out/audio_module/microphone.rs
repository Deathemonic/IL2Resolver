#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::audio_clip::AudioClip;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "Microphone", namespace = "UnityEngine")]
pub struct Microphone(pub *mut c_void);

#[unity_impl]
impl Microphone {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Microphone::get_devices")]
    pub fn get_devices() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Microphone::GetMicrophoneDeviceIDFromName(System.String)")]
    pub fn get_microphone_device_id_from_name(name: &str) -> i32 {}

    #[unity_icall("UnityEngine.Microphone::StartRecord(System.Int32,System.Boolean,System.Single,System.Int32)")]
    pub fn start_record(device_id: i32, loop_ref: bool, length_sec: f32, frequency: i32) -> Option<AudioClip> {}

    #[unity_icall("UnityEngine.Microphone::EndRecord(System.Int32)")]
    pub fn end_record(device_id: i32) {}

    #[unity_icall("UnityEngine.Microphone::IsRecording(System.Int32)")]
    pub fn is_recording(device_id: i32) -> bool {}

    #[unity_icall("UnityEngine.Microphone::GetRecordPosition(System.Int32)")]
    pub fn get_record_position(device_id: i32) -> i32 {}

    #[unity_icall("UnityEngine.Microphone::GetDeviceCaps(System.Int32,System.Int32&,System.Int32&)")]
    pub fn get_device_caps(device_id: i32, min_freq: &mut i32, max_freq: &mut i32) {}

}
