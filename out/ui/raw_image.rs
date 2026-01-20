#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::core_module::{Rect, Texture};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Graphic, MaskableGraphic, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "RawImage", namespace = "UnityEngine.UI", inherit = "MaskableGraphic,Graphic,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct RawImage(pub *mut c_void);

#[unity_impl]
impl RawImage {
    #[unity_method(name = "get_mainTexture")]
    pub fn get_main_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "get_texture")]
    pub fn get_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "set_texture")]
    pub fn set_texture(&self, value: Option<Texture>) {}

    #[unity_method(name = "get_uvRect")]
    pub fn get_uv_rect(&self) -> Rect {}

    #[unity_method(name = "set_uvRect")]
    pub fn set_uv_rect(&self, value: Rect) {}

    #[unity_method(name = "SetNativeSize")]
    pub fn set_native_size(&self) {}

}
