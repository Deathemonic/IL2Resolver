#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::compression_level::CompressionLevel;
use super::compression_type::CompressionType;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AssetBundleModule", class = "BuildCompression", namespace = "UnityEngine", value_type)]
pub struct BuildCompression {
    pub _compression: CompressionType,
    pub _level: CompressionLevel,
    pub _block_size: u32,
}

#[unity_impl]
impl BuildCompression {
    #[unity_method(name = "get_compression")]
    pub fn get_compression(&self) -> CompressionType {}

    #[unity_method(name = "set_compression")]
    pub fn set_compression(&self, value: CompressionType) {}

    #[unity_method(name = "get_level")]
    pub fn get_level(&self) -> CompressionLevel {}

    #[unity_method(name = "set_level")]
    pub fn set_level(&self, value: CompressionLevel) {}

    #[unity_method(name = "get_blockSize")]
    pub fn get_block_size(&self) -> u32 {}

    #[unity_method(name = "set_blockSize")]
    pub fn set_block_size(&self, value: u32) {}

}
