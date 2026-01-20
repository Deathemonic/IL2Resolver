#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::core_module::Sprite;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "SpriteState", namespace = "UnityEngine.UI", value_type)]
pub struct SpriteState {
    pub m_highlighted_sprite: Option<Sprite>,
    pub m_pressed_sprite: Option<Sprite>,
    pub m_selected_sprite: Option<Sprite>,
    pub m_disabled_sprite: Option<Sprite>,
}

#[unity_impl]
impl SpriteState {
    #[unity_method(name = "get_highlightedSprite")]
    pub fn get_highlighted_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_highlightedSprite")]
    pub fn set_highlighted_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "get_pressedSprite")]
    pub fn get_pressed_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_pressedSprite")]
    pub fn set_pressed_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "get_selectedSprite")]
    pub fn get_selected_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_selectedSprite")]
    pub fn set_selected_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "get_disabledSprite")]
    pub fn get_disabled_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_disabledSprite")]
    pub fn set_disabled_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: SpriteState) -> bool {}

}
