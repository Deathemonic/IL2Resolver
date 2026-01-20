#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::camera;
use super::render_texture::RenderTexture;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "RenderRequest", namespace = "UnityEngine", value_type)]
pub struct RenderRequest {
    pub m_camera_render_mode: camera::RenderRequestMode,
    pub m_result_rt: Option<RenderTexture>,
    pub m_output_space: camera::RenderRequestOutputSpace,
}

#[unity_impl]
impl RenderRequest {
    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "get_mode")]
    pub fn get_mode(&self) -> camera::RenderRequestMode {}

    #[unity_method(name = "get_result")]
    pub fn get_result(&self) -> Option<RenderTexture> {}

    #[unity_method(name = "get_outputSpace")]
    pub fn get_output_space(&self) -> camera::RenderRequestOutputSpace {}

}
