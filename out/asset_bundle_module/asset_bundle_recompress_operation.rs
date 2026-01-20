#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::asset_bundle_load_result::AssetBundleLoadResult;
use crate::core_module::{AsyncOperation, YieldInstruction};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "AssetBundleRecompressOperation", namespace = "UnityEngine", inherit = "AsyncOperation,YieldInstruction")]
pub struct AssetBundleRecompressOperation(pub *mut c_void);

#[unity_impl]
impl AssetBundleRecompressOperation {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AssetBundleRecompressOperation::get_humanReadableResult")]
    pub fn get_human_readable_result(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleRecompressOperation::get_inputPath")]
    pub fn get_input_path(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleRecompressOperation::get_outputPath")]
    pub fn get_output_path(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleRecompressOperation::get_result")]
    pub fn get_result(&self) -> AssetBundleLoadResult {}

    #[unity_icall("UnityEngine.AssetBundleRecompressOperation::get_success")]
    pub fn get_success(&self) -> bool {}

}
