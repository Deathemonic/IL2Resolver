#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use crate::mscorlib::collections::{Array, List};
use super::animation_triggers::AnimationTriggers;
use super::axis_event_data::AxisEventData;
use super::base_event_data::BaseEventData;
use super::color_block::ColorBlock;
use super::graphic::Graphic;
use super::image::Image;
use super::navigation::Navigation;
use super::pointer_event_data::PointerEventData;
use super::sprite_state::SpriteState;
use crate::animation_module::Animator;
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Selectable", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Selectable(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Transition {
    #[default]
    None = 0,
    ColorTint = 1,
    SpriteSwap = 2,
    Animation = 3,
}

#[unity_impl]
impl Selectable {
    #[unity_method(name = "get_allSelectablesArray", static)]
    pub fn get_all_selectables_array() -> Array<Selectable> {}

    #[unity_method(name = "get_allSelectableCount", static)]
    pub fn get_all_selectable_count() -> i32 {}

    #[unity_method(name = "get_allSelectables", static)]
    pub fn get_all_selectables() -> List<Selectable> {}

    #[unity_method(name = "get_navigation")]
    pub fn get_navigation(&self) -> Navigation {}

    #[unity_method(name = "set_navigation")]
    pub fn set_navigation(&self, value: Navigation) {}

    #[unity_method(name = "get_transition")]
    pub fn get_transition(&self) -> Transition {}

    #[unity_method(name = "set_transition")]
    pub fn set_transition(&self, value: Transition) {}

    #[unity_method(name = "get_colors")]
    pub fn get_colors(&self) -> ColorBlock {}

    #[unity_method(name = "set_colors")]
    pub fn set_colors(&self, value: ColorBlock) {}

    #[unity_method(name = "get_spriteState")]
    pub fn get_sprite_state(&self) -> SpriteState {}

    #[unity_method(name = "set_spriteState")]
    pub fn set_sprite_state(&self, value: SpriteState) {}

    #[unity_method(name = "get_animationTriggers")]
    pub fn get_animation_triggers(&self) -> Option<AnimationTriggers> {}

    #[unity_method(name = "set_animationTriggers")]
    pub fn set_animation_triggers(&self, value: Option<AnimationTriggers>) {}

    #[unity_method(name = "get_targetGraphic")]
    pub fn get_target_graphic(&self) -> Option<Graphic> {}

    #[unity_method(name = "set_targetGraphic")]
    pub fn set_target_graphic(&self, value: Option<Graphic>) {}

    #[unity_method(name = "get_interactable")]
    pub fn get_interactable(&self) -> bool {}

    #[unity_method(name = "set_interactable")]
    pub fn set_interactable(&self, value: bool) {}

    #[unity_method(name = "get_image")]
    pub fn get_image(&self) -> Option<Image> {}

    #[unity_method(name = "set_image")]
    pub fn set_image(&self, value: Option<Image>) {}

    #[unity_method(name = "get_animator")]
    pub fn get_animator(&self) -> Option<Animator> {}

    #[unity_method(name = "AllSelectablesNoAlloc", static)]
    pub fn all_selectables_no_alloc(selectables: Array<Selectable>) -> i32 {}

    #[unity_method(name = "IsInteractable")]
    pub fn is_interactable(&self) -> bool {}

    #[unity_method(name = "FindSelectable")]
    pub fn find_selectable(&self, dir: Vector3) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnLeft")]
    pub fn find_selectable_on_left(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnRight")]
    pub fn find_selectable_on_right(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnUp")]
    pub fn find_selectable_on_up(&self) -> Option<Selectable> {}

    #[unity_method(name = "FindSelectableOnDown")]
    pub fn find_selectable_on_down(&self) -> Option<Selectable> {}

    #[unity_method(name = "OnMove")]
    pub fn on_move(&self, event_data: Option<AxisEventData>) {}

    #[unity_method(name = "OnPointerDown")]
    pub fn on_pointer_down(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerUp")]
    pub fn on_pointer_up(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerEnter")]
    pub fn on_pointer_enter(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnPointerExit")]
    pub fn on_pointer_exit(&self, event_data: Option<PointerEventData>) {}

    #[unity_method(name = "OnSelect")]
    pub fn on_select(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "OnDeselect")]
    pub fn on_deselect(&self, event_data: Option<BaseEventData>) {}

    #[unity_method(name = "Select")]
    pub fn select(&self) {}

}
