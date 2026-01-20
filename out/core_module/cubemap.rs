#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemArray};
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::cubemap_face::CubemapFace;
use super::default_format::DefaultFormat;
use super::graphics_format::GraphicsFormat;
use super::texture_creation_flags::TextureCreationFlags;
use super::texture_format::TextureFormat;
use crate::core_module::{Object, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Cubemap", namespace = "UnityEngine", inherit = "Texture,Object")]
pub struct Cubemap(pub *mut c_void);

#[unity_impl]
impl Cubemap {
    #[unity_ctor]
    pub fn new(width: i32, format: DefaultFormat, flags: TextureCreationFlags) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(width: i32, format: GraphicsFormat, flags: TextureCreationFlags) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(width: i32, format: TextureFormat, mip_count: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(width: i32, texture_format: TextureFormat, mip_chain: bool) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(width: i32, format: GraphicsFormat, flags: TextureCreationFlags, mip_count: i32) -> Option<Self> {}

    #[unity_icall("UnityEngine.Cubemap::get_format")]
    pub fn get_format(&self) -> TextureFormat {}

    #[unity_icall("UnityEngine.Cubemap::get_isReadable")]
    pub fn get_is_readable(&self) -> bool {}

    #[unity_icall("UnityEngine.Cubemap::get_streamingMipmaps")]
    pub fn get_streaming_mipmaps(&self) -> bool {}

    #[unity_icall("UnityEngine.Cubemap::get_streamingMipmapsPriority")]
    pub fn get_streaming_mipmaps_priority(&self) -> i32 {}

    #[unity_icall("UnityEngine.Cubemap::get_requestedMipmapLevel")]
    pub fn get_requested_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Cubemap::set_requestedMipmapLevel(System.Int32)")]
    pub fn set_requested_mipmap_level(&self, value: i32) {}

    #[unity_icall("UnityEngine.Cubemap::get_desiredMipmapLevel")]
    pub fn get_desired_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Cubemap::get_loadingMipmapLevel")]
    pub fn get_loading_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Cubemap::get_loadedMipmapLevel")]
    pub fn get_loaded_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Cubemap::Internal_CreateImpl(Cubemap,System.Int32,System.Int32,GraphicsFormat,TextureColorSpace,TextureCreationFlags,System.IntPtr)")]
    pub fn internal_create_impl(mono: Option<Cubemap>, ext: i32, mip_count: i32, format: GraphicsFormat, color_space: *mut c_void, flags: TextureCreationFlags, native_tex: isize) -> bool {}

    #[unity_icall("UnityEngine.Cubemap::UpdateExternalTexture(System.IntPtr)")]
    pub fn update_external_texture(&self, native_texture: isize) {}

    #[unity_icall("UnityEngine.Cubemap::SmoothEdges(System.Int32)")]
    pub fn smooth_edges(&self, smooth_region_width_in_pixels: i32) {}

    #[unity_icall("UnityEngine.Cubemap::GetPixels(CubemapFace,System.Int32)")]
    pub fn get_pixels(&self, face: CubemapFace, miplevel: i32) -> Array<Color> {}

    #[unity_icall("UnityEngine.Cubemap::SetPixelDataImplArray(System.Array,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_pixel_data_impl_array(&self, data: Option<SystemArray>, mip_level: i32, face: i32, element_size: i32, data_array_size: i32, source_data_start_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Cubemap::SetPixelDataImpl(System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_pixel_data_impl(&self, data: isize, mip_level: i32, face: i32, element_size: i32, data_array_size: i32, source_data_start_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Cubemap::SetPixels(Color[],CubemapFace,System.Int32)")]
    pub fn set_pixels(&self, colors: Array<Color>, face: CubemapFace, miplevel: i32) {}

    #[unity_icall("UnityEngine.Cubemap::GetWritableImageData(System.Int32)")]
    pub fn get_writable_image_data(&self, frame: i32) -> isize {}

    #[unity_icall("UnityEngine.Cubemap::ClearRequestedMipmapLevel")]
    pub fn clear_requested_mipmap_level(&self) {}

    #[unity_icall("UnityEngine.Cubemap::IsRequestedMipmapLevelLoaded")]
    pub fn is_requested_mipmap_level_loaded(&self) -> bool {}

    #[unity_method(name = "CreateExternalTexture", static)]
    pub fn create_external_texture(width: i32, format: TextureFormat, mipmap: bool, native_tex: isize) -> Option<Cubemap> {}

    #[unity_icall("UnityEngine.Cubemap::SetPixelImpl_Injected(System.Int32,System.Int32,System.Int32,System.Int32,Color&)")]
    pub fn set_pixel(&self, image: i32, mip: i32, x: i32, y: i32, color: &mut Color) {}

    #[unity_icall("UnityEngine.Cubemap::ApplyImpl(System.Boolean,System.Boolean)")]
    pub fn apply(&self, update_mipmaps: bool, make_no_longer_readable: bool) {}

    #[unity_icall("UnityEngine.Cubemap::ApplyImpl(System.Boolean,System.Boolean)")]
    pub fn apply_1(&self, update_mipmaps: bool, make_no_longer_readable: bool) {}

    #[unity_icall("UnityEngine.Cubemap::GetPixelImpl_Injected(System.Int32,System.Int32,System.Int32,System.Int32,Color&)")]
    pub fn get_pixel_impl(&self, image: i32, mip: i32, x: i32, y: i32, ret: &mut Color) {}

}
