#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{List};
use super::base_event_data::BaseEventData;
use super::dropdown_event::DropdownEvent;
use super::image::Image;
use super::option_data::OptionData;
use super::pointer_event_data::PointerEventData;
use super::text::Text;
use crate::core_module::{RectTransform, Sprite};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Selectable, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Dropdown", namespace = "UnityEngine.UI", inherit = "Selectable,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Dropdown(pub *mut c_void);

#[unity_impl]
impl Dropdown {
    #[unity_method(name = "get_template")]
    pub fn get_template(&self) -> Option<RectTransform> {}

    #[unity_method(name = "set_template")]
    pub fn set_template(&self, value: Option<RectTransform>) {}

    #[unity_method(name = "get_captionText")]
    pub fn get_caption_text(&self) -> Option<Text> {}

    #[unity_method(name = "set_captionText")]
    pub fn set_caption_text(&self, value: Option<Text>) {}

    #[unity_method(name = "get_captionImage")]
    pub fn get_caption_image(&self) -> Option<Image> {}

    #[unity_method(name = "set_captionImage")]
    pub fn set_caption_image(&self, value: Option<Image>) {}

    #[unity_method(name = "get_itemText")]
    pub fn get_item_text(&self) -> Option<Text> {}

    #[unity_method(name = "set_itemText")]
    pub fn set_item_text(&self, value: Option<Text>) {}

    #[unity_method(name = "get_itemImage")]
    pub fn get_item_image(&self) -> Option<Image> {}

    #[unity_method(name = "set_itemImage")]
    pub fn set_item_image(&self, value: Option<Image>) {}

    #[unity_method(name = "get_options")]
    pub fn get_options(&self) -> List<OptionData> {}

    #[unity_method(name = "set_options")]
    pub fn set_options(&self, value: List<OptionData>) {}

    #[unity_method(name = "get_onValueChanged")]
    pub fn get_on_value_changed(&self) -> Option<DropdownEvent> {}

    #[unity_method(name = "set_onValueChanged")]
    pub fn set_on_value_changed(&self, value: Option<DropdownEvent>) {}

    #[unity_method(name = "get_alphaFadeSpeed")]
    pub fn get_alpha_fade_speed(&self) -> f32 {}

    #[unity_method(name = "set_alphaFadeSpeed")]
    pub fn set_alpha_fade_speed(&self, value: f32) {}

    #[unity_method(name = "get_value")]
    pub fn get_value(&self) -> i32 {}

    #[unity_method(name = "set_value")]
    pub fn set_value(&self, value: i32) {}

    #[unity_method(name = "SetValueWithoutNotify")]
    pub fn set_value_without_notify(&self, input: i32) {}

    #[unity_method(name = "RefreshShownValue")]
    pub fn refresh_shown_value(&self) {}

    #[unity_method(name = "AddOptions")]
    pub fn add_options(&self, options: List<OptionData>) {}

    #[unity_method(name = "AddOptions")]
    pub fn add_options_1(&self, options: List<SystemString>) {}

    #[unity_method(name = "AddOptions")]
    pub fn add_options_2(&self, options: List<Sprite>) {}

    #[unity_method(name = "ClearOptions")]
    pub fn clear_options(&self) {}

    #[unity_method(name = "OnPointerClick")]
    pub fn on_pointer_click(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSubmit")]
    pub fn on_submit(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnCancel")]
    pub fn on_cancel(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "Show")]
    pub fn show(&self) {}

    #[unity_method(name = "Hide")]
    pub fn hide(&self) {}

}
