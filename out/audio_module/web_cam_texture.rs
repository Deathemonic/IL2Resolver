#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{Nullable, SystemString};
use crate::mscorlib::collections::{Array};
use super::web_cam_device::WebCamDevice;
use crate::core_module::{Color, Color32};
use crate::core_module::{Object, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "WebCamTexture", namespace = "UnityEngine", inherit = "Texture,Object")]
pub struct WebCamTexture(pub *mut c_void);

#[unity_impl]
impl WebCamTexture {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(device_name: &str) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(requested_width: i32, requested_height: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_3(device_name: &str, requested_width: i32, requested_height: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_4(requested_width: i32, requested_height: i32, requested_fps: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_5(device_name: &str, requested_width: i32, requested_height: i32, requested_fps: i32) -> Option<Self> {}

    #[unity_icall("UnityEngine.WebCamTexture::get_devices")]
    pub fn get_devices() -> Array<WebCamDevice> {}

    #[unity_icall("UnityEngine.WebCamTexture::get_isPlaying")]
    pub fn get_is_playing(&self) -> bool {}

    #[unity_icall("UnityEngine.WebCamTexture::get_deviceName")]
    pub fn get_device_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.WebCamTexture::set_deviceName(System.String)")]
    pub fn set_device_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_requestedFPS")]
    pub fn get_requested_fps(&self) -> f32 {}

    #[unity_icall("UnityEngine.WebCamTexture::set_requestedFPS(System.Single)")]
    pub fn set_requested_fps(&self, value: f32) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_requestedWidth")]
    pub fn get_requested_width(&self) -> i32 {}

    #[unity_icall("UnityEngine.WebCamTexture::set_requestedWidth(System.Int32)")]
    pub fn set_requested_width(&self, value: i32) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_requestedHeight")]
    pub fn get_requested_height(&self) -> i32 {}

    #[unity_icall("UnityEngine.WebCamTexture::set_requestedHeight(System.Int32)")]
    pub fn set_requested_height(&self, value: i32) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_videoRotationAngle")]
    pub fn get_video_rotation_angle(&self) -> i32 {}

    #[unity_icall("UnityEngine.WebCamTexture::get_videoVerticallyMirrored")]
    pub fn get_video_vertically_mirrored(&self) -> bool {}

    #[unity_icall("UnityEngine.WebCamTexture::get_didUpdateThisFrame")]
    pub fn get_did_update_this_frame(&self) -> bool {}

    #[unity_method(name = "get_autoFocusPoint")]
    pub fn get_auto_focus_point(&self) -> Nullable<Vector2> {}

    #[unity_method(name = "set_autoFocusPoint")]
    pub fn set_auto_focus_point(&self, value: Nullable<Vector2>) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_isDepth")]
    pub fn get_is_depth(&self) -> bool {}

    #[unity_icall("UnityEngine.WebCamTexture::Play")]
    pub fn play(&self) {}

    #[unity_icall("UnityEngine.WebCamTexture::Pause")]
    pub fn pause(&self) {}

    #[unity_icall("UnityEngine.WebCamTexture::Stop")]
    pub fn stop(&self) {}

    #[unity_icall("UnityEngine.WebCamTexture::GetPixel(System.Int32,System.Int32)")]
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {}

    #[unity_icall("UnityEngine.WebCamTexture::GetPixels(System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn get_pixels(&self, x: i32, y: i32, block_width: i32, block_height: i32) -> Array<Color> {}

    #[unity_icall("UnityEngine.WebCamTexture::GetPixels32(Color32[])")]
    pub fn get_pixels32(&self, colors: Array<Color32>) -> Array<Color32> {}

    #[unity_icall("UnityEngine.WebCamTexture::Internal_CreateWebCamTexture(WebCamTexture,System.String,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_create_web_cam_texture(this: Option<WebCamTexture>, scripting_device: &str, requested_width: i32, requested_height: i32, max_framerate: i32) {}

    #[unity_icall("UnityEngine.WebCamTexture::GetPixel_Injected(System.Int32,System.Int32,Color&)")]
    pub fn get_pixel_1(&self, x: i32, y: i32, ret: &mut Color) {}

    #[unity_icall("UnityEngine.WebCamTexture::get_internalAutoFocusPoint_Injected(Vector2&)")]
    pub fn get_internal_auto_focus_point(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.WebCamTexture::set_internalAutoFocusPoint_Injected(Vector2&)")]
    pub fn set_internal_auto_focus_point(&self, value: &mut Vector2) {}

}
