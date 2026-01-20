#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Resolution", namespace = "UnityEngine", value_type)]
pub struct Resolution {
    pub m_width: i32,
    pub m_height: i32,
    pub m_refresh_rate: i32,
}

#[unity_impl]
impl Resolution {
    #[unity_method(name = "get_width")]
    pub fn get_width(&self) -> i32 {}

    #[unity_method(name = "set_width")]
    pub fn set_width(&self, value: i32) {}

    #[unity_method(name = "get_height")]
    pub fn get_height(&self) -> i32 {}

    #[unity_method(name = "set_height")]
    pub fn set_height(&self, value: i32) {}

    #[unity_method(name = "get_refreshRate")]
    pub fn get_refresh_rate(&self) -> i32 {}

    #[unity_method(name = "set_refreshRate")]
    pub fn set_refresh_rate(&self, value: i32) {}

    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

}
