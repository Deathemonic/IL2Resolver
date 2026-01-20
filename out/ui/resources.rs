#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::core_module::Sprite;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Resources", namespace = "UnityEngine.UI", value_type)]
pub struct Resources {
    pub standard: Option<Sprite>,
    pub background: Option<Sprite>,
    pub input_field: Option<Sprite>,
    pub knob: Option<Sprite>,
    pub checkmark: Option<Sprite>,
    pub dropdown: Option<Sprite>,
    pub mask: Option<Sprite>,
}
