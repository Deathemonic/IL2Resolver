#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use crate::core_module::Texture;
use crate::text_rendering_module::{Font, FontStyle, HorizontalWrapMode, TextAnchor, TextGenerationSettings, TextGenerator, VerticalWrapMode};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::{Graphic, MaskableGraphic, UIBehaviour};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Text", namespace = "UnityEngine.UI", inherit = "MaskableGraphic,Graphic,UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Text(pub *mut c_void);

#[unity_impl]
impl Text {
    #[unity_method(name = "get_cachedTextGenerator")]
    pub fn get_cached_text_generator(&self) -> Option<TextGenerator> {}

    #[unity_method(name = "get_cachedTextGeneratorForLayout")]
    pub fn get_cached_text_generator_for_layout(&self) -> Option<TextGenerator> {}

    #[unity_method(name = "get_mainTexture")]
    pub fn get_main_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "get_font")]
    pub fn get_font(&self) -> Option<Font> {}

    #[unity_method(name = "set_font")]
    pub fn set_font(&self, value: Option<Font>) {}

    #[unity_method(name = "get_text")]
    pub fn get_text(&self) -> Option<SystemString> {}

    #[unity_method(name = "set_text")]
    pub fn set_text(&self, value: &str) {}

    #[unity_method(name = "get_supportRichText")]
    pub fn get_support_rich_text(&self) -> bool {}

    #[unity_method(name = "set_supportRichText")]
    pub fn set_support_rich_text(&self, value: bool) {}

    #[unity_method(name = "get_resizeTextForBestFit")]
    pub fn get_resize_text_for_best_fit(&self) -> bool {}

    #[unity_method(name = "set_resizeTextForBestFit")]
    pub fn set_resize_text_for_best_fit(&self, value: bool) {}

    #[unity_method(name = "get_resizeTextMinSize")]
    pub fn get_resize_text_min_size(&self) -> i32 {}

    #[unity_method(name = "set_resizeTextMinSize")]
    pub fn set_resize_text_min_size(&self, value: i32) {}

    #[unity_method(name = "get_resizeTextMaxSize")]
    pub fn get_resize_text_max_size(&self) -> i32 {}

    #[unity_method(name = "set_resizeTextMaxSize")]
    pub fn set_resize_text_max_size(&self, value: i32) {}

    #[unity_method(name = "get_alignment")]
    pub fn get_alignment(&self) -> TextAnchor {}

    #[unity_method(name = "set_alignment")]
    pub fn set_alignment(&self, value: TextAnchor) {}

    #[unity_method(name = "get_alignByGeometry")]
    pub fn get_align_by_geometry(&self) -> bool {}

    #[unity_method(name = "set_alignByGeometry")]
    pub fn set_align_by_geometry(&self, value: bool) {}

    #[unity_method(name = "get_fontSize")]
    pub fn get_font_size(&self) -> i32 {}

    #[unity_method(name = "set_fontSize")]
    pub fn set_font_size(&self, value: i32) {}

    #[unity_method(name = "get_horizontalOverflow")]
    pub fn get_horizontal_overflow(&self) -> HorizontalWrapMode {}

    #[unity_method(name = "set_horizontalOverflow")]
    pub fn set_horizontal_overflow(&self, value: HorizontalWrapMode) {}

    #[unity_method(name = "get_verticalOverflow")]
    pub fn get_vertical_overflow(&self) -> VerticalWrapMode {}

    #[unity_method(name = "set_verticalOverflow")]
    pub fn set_vertical_overflow(&self, value: VerticalWrapMode) {}

    #[unity_method(name = "get_lineSpacing")]
    pub fn get_line_spacing(&self) -> f32 {}

    #[unity_method(name = "set_lineSpacing")]
    pub fn set_line_spacing(&self, value: f32) {}

    #[unity_method(name = "get_fontStyle")]
    pub fn get_font_style(&self) -> FontStyle {}

    #[unity_method(name = "set_fontStyle")]
    pub fn set_font_style(&self, value: FontStyle) {}

    #[unity_method(name = "get_pixelsPerUnit")]
    pub fn get_pixels_per_unit(&self) -> f32 {}

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

    #[unity_method(name = "FontTextureChanged")]
    pub fn font_texture_changed(&self) {}

    #[unity_method(name = "GetGenerationSettings")]
    pub fn get_generation_settings(&self, extents: Vector2) -> TextGenerationSettings {}

    #[unity_method(name = "GetTextAnchorPivot", static)]
    pub fn get_text_anchor_pivot(anchor: TextAnchor) -> Vector2 {}

    #[unity_method(name = "CalculateLayoutInputHorizontal")]
    pub fn calculate_layout_input_horizontal(&self) {}

    #[unity_method(name = "CalculateLayoutInputVertical")]
    pub fn calculate_layout_input_vertical(&self) {}

    #[unity_method(name = "OnRebuildRequested")]
    pub fn on_rebuild_requested(&self) {}

}
