#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::selectable::Selectable;

#[repr(C)]
#[derive(Clone, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Navigation", namespace = "UnityEngine.UI", value_type)]
pub struct Navigation {
    pub m_mode: Mode,
    pub m_wrap_around: bool,
    pub m_select_on_up: Option<Selectable>,
    pub m_select_on_down: Option<Selectable>,
    pub m_select_on_left: Option<Selectable>,
    pub m_select_on_right: Option<Selectable>,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Automatic = 3,
    Explicit = 4,
}

#[unity_impl]
impl Navigation {
    #[unity_method(name = "get_mode")]
    pub fn get_mode(&self) -> Mode {}

    #[unity_method(name = "set_mode")]
    pub fn set_mode(&self, value: Mode) {}

    #[unity_method(name = "get_wrapAround")]
    pub fn get_wrap_around(&self) -> bool {}

    #[unity_method(name = "set_wrapAround")]
    pub fn set_wrap_around(&self, value: bool) {}

    #[unity_method(name = "get_selectOnUp")]
    pub fn get_select_on_up(&self) -> Option<Selectable> {}

    #[unity_method(name = "set_selectOnUp")]
    pub fn set_select_on_up(&self, value: Option<Selectable>) {}

    #[unity_method(name = "get_selectOnDown")]
    pub fn get_select_on_down(&self) -> Option<Selectable> {}

    #[unity_method(name = "set_selectOnDown")]
    pub fn set_select_on_down(&self, value: Option<Selectable>) {}

    #[unity_method(name = "get_selectOnLeft")]
    pub fn get_select_on_left(&self) -> Option<Selectable> {}

    #[unity_method(name = "set_selectOnLeft")]
    pub fn set_select_on_left(&self, value: Option<Selectable>) {}

    #[unity_method(name = "get_selectOnRight")]
    pub fn get_select_on_right(&self) -> Option<Selectable> {}

    #[unity_method(name = "set_selectOnRight")]
    pub fn set_select_on_right(&self, value: Option<Selectable>) {}

    #[unity_method(name = "get_defaultNavigation", static)]
    pub fn get_default_navigation() -> Navigation {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Navigation) -> bool {}

}
