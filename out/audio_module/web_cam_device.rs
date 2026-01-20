#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::web_cam_kind::WebCamKind;
use crate::core_module::Resolution;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "WebCamDevice", namespace = "UnityEngine", value_type)]
pub struct WebCamDevice {
    pub m_name: Option<SystemString>,
    pub m_depth_camera_name: Option<SystemString>,
    pub m_flags: i32,
    pub m_kind: WebCamKind,
    pub m_resolutions: Array<Resolution>,
}

#[unity_impl]
impl WebCamDevice {
    #[unity_method(name = "get_name")]
    pub fn get_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_isFrontFacing")]
    pub fn get_is_front_facing(&self) -> bool {}

    #[unity_method(name = "get_kind")]
    pub fn get_kind(&self) -> WebCamKind {}

    #[unity_method(name = "get_depthCameraName")]
    pub fn get_depth_camera_name(&self) -> Option<SystemString> {}

    #[unity_method(name = "get_isAutoFocusPointSupported")]
    pub fn get_is_auto_focus_point_supported(&self) -> bool {}

    #[unity_method(name = "get_availableResolutions")]
    pub fn get_available_resolutions(&self) -> Array<Resolution> {}

}
