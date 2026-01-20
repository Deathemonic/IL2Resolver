#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PlayerPrefs", namespace = "UnityEngine")]
pub struct PlayerPrefs(pub *mut c_void);

#[unity_impl]
impl PlayerPrefs {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.PlayerPrefs::TrySetInt(System.String,System.Int32)")]
    pub fn try_set_int(key: &str, value: i32) -> bool {}

    #[unity_icall("UnityEngine.PlayerPrefs::TrySetFloat(System.String,System.Single)")]
    pub fn try_set_float(key: &str, value: f32) -> bool {}

    #[unity_icall("UnityEngine.PlayerPrefs::TrySetSetString(System.String,System.String)")]
    pub fn try_set_set_string(key: &str, value: &str) -> bool {}

    #[unity_icall("UnityEngine.PlayerPrefs::GetInt(System.String,System.Int32)")]
    pub fn get_int(key: &str, default_value: i32) -> i32 {}

    #[unity_icall("UnityEngine.PlayerPrefs::GetFloat(System.String,System.Single)")]
    pub fn get_float(key: &str, default_value: f32) -> f32 {}

    #[unity_icall("UnityEngine.PlayerPrefs::GetString(System.String,System.String)")]
    pub fn get_string(key: &str, default_value: &str) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.PlayerPrefs::HasKey(System.String)")]
    pub fn has_key(key: &str) -> bool {}

    #[unity_icall("UnityEngine.PlayerPrefs::DeleteKey(System.String)")]
    pub fn delete_key(key: &str) {}

    #[unity_icall("UnityEngine.PlayerPrefs::DeleteAll")]
    pub fn delete_all() {}

    #[unity_icall("UnityEngine.PlayerPrefs::Save")]
    pub fn save() {}

}
