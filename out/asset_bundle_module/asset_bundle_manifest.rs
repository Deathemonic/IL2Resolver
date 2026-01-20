#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use crate::core_module::Hash128;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "AssetBundleManifest", namespace = "UnityEngine", inherit = "Object")]
pub struct AssetBundleManifest(pub *mut c_void);

#[unity_impl]
impl AssetBundleManifest {
    #[unity_icall("UnityEngine.AssetBundleManifest::GetAllAssetBundles")]
    pub fn get_all_asset_bundles(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleManifest::GetAllAssetBundlesWithVariant")]
    pub fn get_all_asset_bundles_with_variant(&self) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleManifest::GetAssetBundleHash(System.String)")]
    pub fn get_asset_bundle_hash(&self, asset_bundle_name: &str) -> Hash128 {}

    #[unity_icall("UnityEngine.AssetBundleManifest::GetDirectDependencies(System.String)")]
    pub fn get_direct_dependencies(&self, asset_bundle_name: &str) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleManifest::GetAllDependencies(System.String)")]
    pub fn get_all_dependencies(&self, asset_bundle_name: &str) -> Array<SystemString> {}

    #[unity_icall("UnityEngine.AssetBundleManifest::GetAssetBundleHash_Injected(System.String,Hash128&)")]
    pub fn get_asset_bundle_hash_1(&self, asset_bundle_name: &str, ret: &mut Hash128) {}

}
