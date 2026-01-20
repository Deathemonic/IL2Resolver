#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use crate::core_module::Object;
use crate::core_module::{AsyncOperation, ResourceRequest, YieldInstruction};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "AssetBundleRequest", namespace = "UnityEngine", inherit = "ResourceRequest,AsyncOperation,YieldInstruction")]
pub struct AssetBundleRequest(pub *mut c_void);

#[unity_impl]
impl AssetBundleRequest {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_asset")]
    pub fn get_asset(&self) -> Option<Object> {}

    #[unity_icall("UnityEngine.AssetBundleRequest::get_allAssets")]
    pub fn get_all_assets(&self) -> Array<Object> {}

    #[unity_icall("UnityEngine.AssetBundleRequest::GetResult")]
    pub fn get_result(&self) -> Option<Object> {}

}
