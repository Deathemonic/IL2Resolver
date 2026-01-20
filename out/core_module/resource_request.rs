#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::object::Object;
use crate::core_module::{AsyncOperation, YieldInstruction};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ResourceRequest", namespace = "UnityEngine", inherit = "AsyncOperation,YieldInstruction")]
pub struct ResourceRequest(pub *mut c_void);

#[unity_impl]
impl ResourceRequest {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_asset")]
    pub fn get_asset(&self) -> Option<Object> {}

}
