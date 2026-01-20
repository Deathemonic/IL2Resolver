#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Color, Rect, RectOffset, Texture};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.IMGUIModule", class = "GUITexture", namespace = "UnityEngine")]
pub struct GUITexture(pub *mut c_void);

#[unity_impl]
impl GUITexture {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_color")]
    pub fn get_color(&self) -> Color {}

    #[unity_method(name = "set_color")]
    pub fn set_color(&self, value: Color) {}

    #[unity_method(name = "get_texture")]
    pub fn get_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "set_texture")]
    pub fn set_texture(&self, value: Option<Texture>) {}

    #[unity_method(name = "get_pixelInset")]
    pub fn get_pixel_inset(&self) -> Rect {}

    #[unity_method(name = "set_pixelInset")]
    pub fn set_pixel_inset(&self, value: Rect) {}

    #[unity_method(name = "get_border")]
    pub fn get_border(&self) -> Option<RectOffset> {}

    #[unity_method(name = "set_border")]
    pub fn set_border(&self, value: Option<RectOffset>) {}

}
