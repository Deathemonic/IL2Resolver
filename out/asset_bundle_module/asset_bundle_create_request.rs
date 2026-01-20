#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::asset_bundle::AssetBundle;
use crate::core_module::{AsyncOperation, YieldInstruction};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "AssetBundleCreateRequest", namespace = "UnityEngine", inherit = "AsyncOperation,YieldInstruction")]
pub struct AssetBundleCreateRequest(pub *mut c_void);

#[unity_impl]
impl AssetBundleCreateRequest {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AssetBundleCreateRequest::get_assetBundle")]
    pub fn get_asset_bundle(&self) -> Option<AssetBundle> {}

    #[unity_icall("UnityEngine.AssetBundleCreateRequest::SetEnableCompatibilityChecks(System.Boolean)")]
    pub fn set_enable_compatibility_checks(&self, set: bool) {}

}
