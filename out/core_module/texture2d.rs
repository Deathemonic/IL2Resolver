#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemArray};
use crate::mscorlib::collections::{Array};
use super::color::Color;
use super::color32::Color32;
use super::default_format::DefaultFormat;
use super::graphics_format::GraphicsFormat;
use super::rect::Rect;
use super::texture_creation_flags::TextureCreationFlags;
use super::texture_format::TextureFormat;
use crate::core_module::{Object, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Texture2D", namespace = "UnityEngine", inherit = "Texture,Object")]
pub struct Texture2D(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EXRFlags {
    #[default]
    None = 0,
    OutputAsFloat = 1,
    CompressZIP = 2,
    CompressRLE = 4,
    CompressPIZ = 8,
}

#[unity_impl]
impl Texture2D {
    #[unity_ctor]
    pub fn new(width: i32, height: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(width: i32, height: i32, format: DefaultFormat, flags: TextureCreationFlags) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(width: i32, height: i32, format: GraphicsFormat, flags: TextureCreationFlags) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(width: i32, height: i32, texture_format: TextureFormat, mip_chain: bool) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(width: i32, height: i32, format: GraphicsFormat, mip_count: i32, flags: TextureCreationFlags) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_5(width: i32, height: i32, texture_format: TextureFormat, mip_count: i32, linear: bool) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_6(width: i32, height: i32, texture_format: TextureFormat, mip_chain: bool, linear: bool) -> Option<Self> {}

    #[unity_icall("UnityEngine.Texture2D::get_format")]
    pub fn get_format(&self) -> TextureFormat {}

    #[unity_icall("UnityEngine.Texture2D::get_ignoreMipmapLimit")]
    pub fn get_ignore_mipmap_limit(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::set_ignoreMipmapLimit(System.Boolean)")]
    pub fn set_ignore_mipmap_limit(&self, value: bool) {}

    #[unity_icall("UnityEngine.Texture2D::get_whiteTexture")]
    pub fn get_white_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_blackTexture")]
    pub fn get_black_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_redTexture")]
    pub fn get_red_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_grayTexture")]
    pub fn get_gray_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_linearGrayTexture")]
    pub fn get_linear_gray_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_normalTexture")]
    pub fn get_normal_texture() -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::get_isReadable")]
    pub fn get_is_readable(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::get_vtOnly")]
    pub fn get_vt_only(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::get_streamingMipmaps")]
    pub fn get_streaming_mipmaps(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::get_streamingMipmapsPriority")]
    pub fn get_streaming_mipmaps_priority(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::get_requestedMipmapLevel")]
    pub fn get_requested_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::set_requestedMipmapLevel(System.Int32)")]
    pub fn set_requested_mipmap_level(&self, value: i32) {}

    #[unity_icall("UnityEngine.Texture2D::get_minimumMipmapLevel")]
    pub fn get_minimum_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::set_minimumMipmapLevel(System.Int32)")]
    pub fn set_minimum_mipmap_level(&self, value: i32) {}

    #[unity_icall("UnityEngine.Texture2D::get_calculatedMipmapLevel")]
    pub fn get_calculated_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::get_desiredMipmapLevel")]
    pub fn get_desired_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::get_loadingMipmapLevel")]
    pub fn get_loading_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::get_loadedMipmapLevel")]
    pub fn get_loaded_mipmap_level(&self) -> i32 {}

    #[unity_icall("UnityEngine.Texture2D::Compress(System.Boolean)")]
    pub fn compress(&self, high_quality: bool) {}

    #[unity_icall("UnityEngine.Texture2D::Internal_CreateImpl(Texture2D,System.Int32,System.Int32,System.Int32,GraphicsFormat,TextureColorSpace,TextureCreationFlags,System.IntPtr)")]
    pub fn internal_create_impl(mono: Option<Texture2D>, w: i32, h: i32, mip_count: i32, format: GraphicsFormat, color_space: *mut c_void, flags: TextureCreationFlags, native_tex: isize) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::ReinitializeImpl(System.Int32,System.Int32)")]
    pub fn reinitialize_impl(&self, width: i32, height: i32) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::ReinitializeWithFormatImpl(System.Int32,System.Int32,GraphicsFormat,System.Boolean)")]
    pub fn reinitialize_with_format_impl(&self, width: i32, height: i32, format: GraphicsFormat, has_mip_map: bool) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::LoadRawTextureDataImpl(System.IntPtr,System.UInt64)")]
    pub fn load_raw_texture_data_impl(&self, data: isize, size: u64) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::LoadRawTextureDataImplArray(System.Byte[])")]
    pub fn load_raw_texture_data_impl_array(&self, data: Array<u8>) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::SetPixelDataImplArray(System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_pixel_data_impl_array(&self, data: Option<SystemArray>, mip_level: i32, element_size: i32, data_array_size: i32, source_data_start_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::SetPixelDataImpl(System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_pixel_data_impl(&self, data: isize, mip_level: i32, element_size: i32, data_array_size: i32, source_data_start_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::GetWritableImageData(System.Int32)")]
    pub fn get_writable_image_data(&self, frame: i32) -> isize {}

    #[unity_icall("UnityEngine.Texture2D::GetRawImageDataSize")]
    pub fn get_raw_image_data_size(&self) -> u64 {}

    #[unity_icall("UnityEngine.Texture2D::GenerateAtlasImpl(Vector2[],System.Int32,System.Int32,Rect[])")]
    pub fn generate_atlas_impl(sizes: Array<Vector2>, padding: i32, atlas_size: i32, rect: &mut Array<Rect>) {}

    #[unity_icall("UnityEngine.Texture2D::ClearRequestedMipmapLevel")]
    pub fn clear_requested_mipmap_level(&self) {}

    #[unity_icall("UnityEngine.Texture2D::IsRequestedMipmapLevelLoaded")]
    pub fn is_requested_mipmap_level_loaded(&self) -> bool {}

    #[unity_icall("UnityEngine.Texture2D::ClearMinimumMipmapLevel")]
    pub fn clear_minimum_mipmap_level(&self) {}

    #[unity_icall("UnityEngine.Texture2D::UpdateExternalTexture(System.IntPtr)")]
    pub fn update_external_texture(&self, native_tex: isize) {}

    #[unity_icall("UnityEngine.Texture2D::GetRawTextureData")]
    pub fn get_raw_texture_data(&self) -> Array<u8> {}

    #[unity_icall("UnityEngine.Texture2D::GetPixels(System.Int32,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn get_pixels(&self, x: i32, y: i32, block_width: i32, block_height: i32, miplevel: i32) -> Array<Color> {}

    #[unity_icall("UnityEngine.Texture2D::GetPixels32(System.Int32)")]
    pub fn get_pixels32(&self, miplevel: i32) -> Array<Color32> {}

    #[unity_icall("UnityEngine.Texture2D::PackTextures(Texture2D[],System.Int32,System.Int32,System.Boolean)")]
    pub fn pack_textures(&self, textures: Array<Texture2D>, padding: i32, maximum_atlas_size: i32, make_no_longer_readable: bool) -> Array<Rect> {}

    #[unity_method(name = "CreateExternalTexture", static)]
    pub fn create_external_texture(width: i32, height: i32, format: TextureFormat, mip_chain: bool, linear: bool, native_tex: isize) -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Texture2D::SetPixelsImpl(System.Int32,System.Int32,System.Int32,System.Int32,Color[],System.Int32,System.Int32)")]
    pub fn set_pixels(&self, x: i32, y: i32, w: i32, h: i32, pixel: Array<Color>, miplevel: i32, frame: i32) {}

    #[unity_icall("UnityEngine.Texture2D::SetPixelsImpl(System.Int32,System.Int32,System.Int32,System.Int32,Color[],System.Int32,System.Int32)")]
    pub fn set_pixels_1(&self, x: i32, y: i32, w: i32, h: i32, pixel: Array<Color>, miplevel: i32, frame: i32) {}

    #[unity_icall("UnityEngine.Texture2D::ApplyImpl(System.Boolean,System.Boolean)")]
    pub fn apply(&self, update_mipmaps: bool, make_no_longer_readable: bool) {}

    #[unity_icall("UnityEngine.Texture2D::ApplyImpl(System.Boolean,System.Boolean)")]
    pub fn apply_1(&self, update_mipmaps: bool, make_no_longer_readable: bool) {}

    #[unity_icall("UnityEngine.Texture2D::ReadPixelsImpl(Rect,System.Int32,System.Int32,System.Boolean)")]
    pub fn read_pixels(&self, source: Rect, dest_x: i32, dest_y: i32, recalculate_mip_maps: bool) {}

    #[unity_icall("UnityEngine.Texture2D::SetAllPixels32(Color32[],System.Int32)")]
    pub fn set_pixels32(&self, colors: Array<Color32>, miplevel: i32) {}

    #[unity_icall("UnityEngine.Texture2D::SetAllPixels32(Color32[],System.Int32)")]
    pub fn set_pixels32_1(&self, colors: Array<Color32>, miplevel: i32) {}

    #[unity_icall("UnityEngine.Texture2D::SetBlockOfPixels32(System.Int32,System.Int32,System.Int32,System.Int32,Color32[],System.Int32)")]
    pub fn set_pixels32_2(&self, x: i32, y: i32, block_width: i32, block_height: i32, colors: Array<Color32>, miplevel: i32) {}

    #[unity_icall("UnityEngine.Texture2D::SetBlockOfPixels32(System.Int32,System.Int32,System.Int32,System.Int32,Color32[],System.Int32)")]
    pub fn set_pixels32_3(&self, x: i32, y: i32, block_width: i32, block_height: i32, colors: Array<Color32>, miplevel: i32) {}

    #[unity_icall("UnityEngine.Texture2D::SetPixelImpl_Injected(System.Int32,System.Int32,System.Int32,System.Int32,Color&)")]
    pub fn set_pixel_impl(&self, image: i32, mip: i32, x: i32, y: i32, color: &mut Color) {}

    #[unity_icall("UnityEngine.Texture2D::GetPixelImpl_Injected(System.Int32,System.Int32,System.Int32,System.Int32,Color&)")]
    pub fn get_pixel_impl(&self, image: i32, mip: i32, x: i32, y: i32, ret: &mut Color) {}

    #[unity_icall("UnityEngine.Texture2D::GetPixelBilinearImpl_Injected(System.Int32,System.Int32,System.Single,System.Single,Color&)")]
    pub fn get_pixel_bilinear_impl(&self, image: i32, mip: i32, u: f32, v: f32, ret: &mut Color) {}

    #[unity_icall("UnityEngine.Texture2D::ReadPixelsImpl_Injected(Rect&,System.Int32,System.Int32,System.Boolean)")]
    pub fn read_pixels_impl(&self, source: &mut Rect, dest_x: i32, dest_y: i32, recalculate_mip_maps: bool) {}

}
