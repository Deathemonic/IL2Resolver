#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "AsyncGPUReadbackRequest", namespace = "UnityEngine.Rendering", value_type)]
pub struct AsyncGPUReadbackRequest {
    pub m_ptr: isize,
    pub m_version: i32,
}

#[unity_impl]
impl AsyncGPUReadbackRequest {
    #[unity_method(name = "get_done")]
    pub fn get_done(&self) -> bool {}

    #[unity_method(name = "get_hasError")]
    pub fn get_has_error(&self) -> bool {}

    #[unity_method(name = "get_layerCount")]
    pub fn get_layer_count(&self) -> i32 {}

    #[unity_method(name = "get_layerDataSize")]
    pub fn get_layer_data_size(&self) -> i32 {}

    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_method(name = "get_depth")]
    pub fn get_depth(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::Update_Injected(AsyncGPUReadbackRequest&)")]
    pub fn update(_unity_self: &mut AsyncGPUReadbackRequest) {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::WaitForCompletion_Injected(AsyncGPUReadbackRequest&)")]
    pub fn wait_for_completion(_unity_self: &mut AsyncGPUReadbackRequest) {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::IsDone_Injected(AsyncGPUReadbackRequest&)")]
    pub fn is_done(_unity_self: &mut AsyncGPUReadbackRequest) -> bool {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::HasError_Injected(AsyncGPUReadbackRequest&)")]
    pub fn has_error(_unity_self: &mut AsyncGPUReadbackRequest) -> bool {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetLayerCount_Injected(AsyncGPUReadbackRequest&)")]
    pub fn get_layer_count_1(_unity_self: &mut AsyncGPUReadbackRequest) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetLayerDataSize_Injected(AsyncGPUReadbackRequest&)")]
    pub fn get_layer_data_size_1(_unity_self: &mut AsyncGPUReadbackRequest) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetWidth_Injected(AsyncGPUReadbackRequest&)")]
    pub fn get_width_1(_unity_self: &mut AsyncGPUReadbackRequest) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetHeight_Injected(AsyncGPUReadbackRequest&)")]
    pub fn get_height_1(_unity_self: &mut AsyncGPUReadbackRequest) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetDepth_Injected(AsyncGPUReadbackRequest&)")]
    pub fn get_depth_1(_unity_self: &mut AsyncGPUReadbackRequest) -> i32 {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::SetScriptingCallback_Injected(AsyncGPUReadbackRequest&,Action<AsyncGPUReadbackRequest>)")]
    pub fn set_scripting_callback(_unity_self: &mut AsyncGPUReadbackRequest, callback: *mut c_void) {}

    #[unity_icall("UnityEngine.Rendering.AsyncGPUReadbackRequest::GetDataRaw_Injected(AsyncGPUReadbackRequest&,System.Int32)")]
    pub fn get_data_raw(_unity_self: &mut AsyncGPUReadbackRequest, layer: i32) -> isize {}

}
