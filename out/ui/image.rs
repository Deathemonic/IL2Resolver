#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::core_module::{Camera, Material, Sprite, Texture};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Graphic, MaskableGraphic, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Image", namespace = "UnityEngine.UI", inherit = "MaskableGraphic,Graphic,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Image(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Type {
    #[default]
    Simple = 0,
    Sliced = 1,
    Tiled = 2,
    Filled = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillMethod {
    #[default]
    Horizontal = 0,
    Vertical = 1,
    Radial90 = 2,
    Radial180 = 3,
    Radial360 = 4,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OriginHorizontal {
    #[default]
    Left = 0,
    Right = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OriginVertical {
    #[default]
    Bottom = 0,
    Top = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin90 {
    #[default]
    BottomLeft = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomRight = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin180 {
    #[default]
    Bottom = 0,
    Left = 1,
    Top = 2,
    Right = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Origin360 {
    #[default]
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
}

#[unity_impl]
impl Image {
    #[unity_method(name = "get_sprite")]
    pub fn get_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_sprite")]
    pub fn set_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "get_overrideSprite")]
    pub fn get_override_sprite(&self) -> Option<Sprite> {}

    #[unity_method(name = "set_overrideSprite")]
    pub fn set_override_sprite(&self, value: Option<Sprite>) {}

    #[unity_method(name = "get_type")]
    pub fn get_type(&self) -> Type {}

    #[unity_method(name = "set_type")]
    pub fn set_type(&self, value: Type) {}

    #[unity_method(name = "get_preserveAspect")]
    pub fn get_preserve_aspect(&self) -> bool {}

    #[unity_method(name = "set_preserveAspect")]
    pub fn set_preserve_aspect(&self, value: bool) {}

    #[unity_method(name = "get_fillCenter")]
    pub fn get_fill_center(&self) -> bool {}

    #[unity_method(name = "set_fillCenter")]
    pub fn set_fill_center(&self, value: bool) {}

    #[unity_method(name = "get_fillMethod")]
    pub fn get_fill_method(&self) -> FillMethod {}

    #[unity_method(name = "set_fillMethod")]
    pub fn set_fill_method(&self, value: FillMethod) {}

    #[unity_method(name = "get_fillAmount")]
    pub fn get_fill_amount(&self) -> f32 {}

    #[unity_method(name = "set_fillAmount")]
    pub fn set_fill_amount(&self, value: f32) {}

    #[unity_method(name = "get_fillClockwise")]
    pub fn get_fill_clockwise(&self) -> bool {}

    #[unity_method(name = "set_fillClockwise")]
    pub fn set_fill_clockwise(&self, value: bool) {}

    #[unity_method(name = "get_fillOrigin")]
    pub fn get_fill_origin(&self) -> i32 {}

    #[unity_method(name = "set_fillOrigin")]
    pub fn set_fill_origin(&self, value: i32) {}

    #[unity_method(name = "get_eventAlphaThreshold")]
    pub fn get_event_alpha_threshold(&self) -> f32 {}

    #[unity_method(name = "set_eventAlphaThreshold")]
    pub fn set_event_alpha_threshold(&self, value: f32) {}

    #[unity_method(name = "get_alphaHitTestMinimumThreshold")]
    pub fn get_alpha_hit_test_minimum_threshold(&self) -> f32 {}

    #[unity_method(name = "set_alphaHitTestMinimumThreshold")]
    pub fn set_alpha_hit_test_minimum_threshold(&self, value: f32) {}

    #[unity_method(name = "get_useSpriteMesh")]
    pub fn get_use_sprite_mesh(&self) -> bool {}

    #[unity_method(name = "set_useSpriteMesh")]
    pub fn set_use_sprite_mesh(&self, value: bool) {}

    #[unity_method(name = "get_defaultETC1GraphicMaterial", static)]
    pub fn get_default_etc1graphic_material() -> Option<Material> {}

    #[unity_method(name = "get_mainTexture")]
    pub fn get_main_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "get_hasBorder")]
    pub fn get_has_border(&self) -> bool {}

    #[unity_method(name = "get_pixelsPerUnitMultiplier")]
    pub fn get_pixels_per_unit_multiplier(&self) -> f32 {}

    #[unity_method(name = "set_pixelsPerUnitMultiplier")]
    pub fn set_pixels_per_unit_multiplier(&self, value: f32) {}

    #[unity_method(name = "get_pixelsPerUnit")]
    pub fn get_pixels_per_unit(&self) -> f32 {}

    #[unity_method(name = "get_material")]
    pub fn get_material(&self) -> Option<Material> {}

    #[unity_method(name = "set_material")]
    pub fn set_material(&self, value: Option<Material>) {}

    #[unity_method(name = "get_minWidth")]
    pub fn get_min_width(&self) -> f32 {}

    #[unity_method(name = "get_preferredWidth")]
    pub fn get_preferred_width(&self) -> f32 {}

    #[unity_method(name = "get_flexibleWidth")]
    pub fn get_flexible_width(&self) -> f32 {}

    #[unity_method(name = "get_minHeight")]
    pub fn get_min_height(&self) -> f32 {}

    #[unity_method(name = "get_preferredHeight")]
    pub fn get_preferred_height(&self) -> f32 {}

    #[unity_method(name = "get_flexibleHeight")]
    pub fn get_flexible_height(&self) -> f32 {}

    #[unity_method(name = "get_layoutPriority")]
    pub fn get_layout_priority(&self) -> i32 {}

    #[unity_method(name = "DisableSpriteOptimizations")]
    pub fn disable_sprite_optimizations(&self) {}

    #[unity_method(name = "OnBeforeSerialize")]
    pub fn on_before_serialize(&self) {}

    #[unity_method(name = "OnAfterDeserialize")]
    pub fn on_after_deserialize(&self) {}

    #[unity_method(name = "SetNativeSize")]
    pub fn set_native_size(&self) {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "IsRaycastLocationValid")]
    pub fn is_raycast_location_valid(&self, screen_point: Vector2, event_camera: Option<Camera>) -> bool {}

}
