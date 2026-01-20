#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2};
use crate::mscorlib::{SystemString};
use super::additional_canvas_shader_channels::AdditionalCanvasShaderChannels;
use super::render_mode::RenderMode;
use super::will_render_canvases::WillRenderCanvases;
use crate::core_module::{Camera, Material, Rect};
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UIModule", class = "Canvas", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct Canvas(pub *mut c_void);

#[unity_impl]
impl Canvas {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Canvas::get_renderMode")]
    pub fn get_render_mode(&self) -> RenderMode {}

    #[unity_icall("UnityEngine.Canvas::set_renderMode(RenderMode)")]
    pub fn set_render_mode(&self, value: RenderMode) {}

    #[unity_icall("UnityEngine.Canvas::get_isRootCanvas")]
    pub fn get_is_root_canvas(&self) -> bool {}

    #[unity_icall("UnityEngine.Canvas::get_pixelRect_Injected(Rect&)")]
    pub fn get_pixel_rect(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Canvas::get_scaleFactor")]
    pub fn get_scale_factor(&self) -> f32 {}

    #[unity_icall("UnityEngine.Canvas::set_scaleFactor(System.Single)")]
    pub fn set_scale_factor(&self, value: f32) {}

    #[unity_icall("UnityEngine.Canvas::get_referencePixelsPerUnit")]
    pub fn get_reference_pixels_per_unit(&self) -> f32 {}

    #[unity_icall("UnityEngine.Canvas::set_referencePixelsPerUnit(System.Single)")]
    pub fn set_reference_pixels_per_unit(&self, value: f32) {}

    #[unity_icall("UnityEngine.Canvas::get_overridePixelPerfect")]
    pub fn get_override_pixel_perfect(&self) -> bool {}

    #[unity_icall("UnityEngine.Canvas::set_overridePixelPerfect(System.Boolean)")]
    pub fn set_override_pixel_perfect(&self, value: bool) {}

    #[unity_icall("UnityEngine.Canvas::get_vertexColorAlwaysGammaSpace")]
    pub fn get_vertex_color_always_gamma_space(&self) -> bool {}

    #[unity_icall("UnityEngine.Canvas::set_vertexColorAlwaysGammaSpace(System.Boolean)")]
    pub fn set_vertex_color_always_gamma_space(&self, value: bool) {}

    #[unity_icall("UnityEngine.Canvas::get_pixelPerfect")]
    pub fn get_pixel_perfect(&self) -> bool {}

    #[unity_icall("UnityEngine.Canvas::set_pixelPerfect(System.Boolean)")]
    pub fn set_pixel_perfect(&self, value: bool) {}

    #[unity_icall("UnityEngine.Canvas::get_planeDistance")]
    pub fn get_plane_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.Canvas::set_planeDistance(System.Single)")]
    pub fn set_plane_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.Canvas::get_renderOrder")]
    pub fn get_render_order(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::get_overrideSorting")]
    pub fn get_override_sorting(&self) -> bool {}

    #[unity_icall("UnityEngine.Canvas::set_overrideSorting(System.Boolean)")]
    pub fn set_override_sorting(&self, value: bool) {}

    #[unity_icall("UnityEngine.Canvas::get_sortingOrder")]
    pub fn get_sorting_order(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::set_sortingOrder(System.Int32)")]
    pub fn set_sorting_order(&self, value: i32) {}

    #[unity_icall("UnityEngine.Canvas::get_targetDisplay")]
    pub fn get_target_display(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::set_targetDisplay(System.Int32)")]
    pub fn set_target_display(&self, value: i32) {}

    #[unity_icall("UnityEngine.Canvas::get_sortingLayerID")]
    pub fn get_sorting_layer_id(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::set_sortingLayerID(System.Int32)")]
    pub fn set_sorting_layer_id(&self, value: i32) {}

    #[unity_icall("UnityEngine.Canvas::get_cachedSortingLayerValue")]
    pub fn get_cached_sorting_layer_value(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::get_additionalShaderChannels")]
    pub fn get_additional_shader_channels(&self) -> AdditionalCanvasShaderChannels {}

    #[unity_icall("UnityEngine.Canvas::set_additionalShaderChannels(AdditionalCanvasShaderChannels)")]
    pub fn set_additional_shader_channels(&self, value: AdditionalCanvasShaderChannels) {}

    #[unity_icall("UnityEngine.Canvas::get_sortingLayerName")]
    pub fn get_sorting_layer_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Canvas::set_sortingLayerName(System.String)")]
    pub fn set_sorting_layer_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.Canvas::get_rootCanvas")]
    pub fn get_root_canvas(&self) -> Option<Canvas> {}

    #[unity_icall("UnityEngine.Canvas::get_renderingDisplaySize_Injected(Vector2&)")]
    pub fn get_rendering_display_size(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Canvas::get_worldCamera")]
    pub fn get_world_camera(&self) -> Option<Camera> {}

    #[unity_icall("UnityEngine.Canvas::set_worldCamera(Camera)")]
    pub fn set_world_camera(&self, value: Option<Camera>) {}

    #[unity_icall("UnityEngine.Canvas::get_normalizedSortingGridSize")]
    pub fn get_normalized_sorting_grid_size(&self) -> f32 {}

    #[unity_icall("UnityEngine.Canvas::set_normalizedSortingGridSize(System.Single)")]
    pub fn set_normalized_sorting_grid_size(&self, value: f32) {}

    #[unity_icall("UnityEngine.Canvas::get_sortingGridNormalizedSize")]
    pub fn get_sorting_grid_normalized_size(&self) -> i32 {}

    #[unity_icall("UnityEngine.Canvas::set_sortingGridNormalizedSize(System.Int32)")]
    pub fn set_sorting_grid_normalized_size(&self, value: i32) {}

    #[unity_method(name = "add_preWillRenderCanvases", static)]
    pub fn add_pre_will_render_canvases(value: Option<WillRenderCanvases>) {}

    #[unity_method(name = "remove_preWillRenderCanvases", static)]
    pub fn remove_pre_will_render_canvases(value: Option<WillRenderCanvases>) {}

    #[unity_method(name = "add_willRenderCanvases", static)]
    pub fn add_will_render_canvases(value: Option<WillRenderCanvases>) {}

    #[unity_method(name = "remove_willRenderCanvases", static)]
    pub fn remove_will_render_canvases(value: Option<WillRenderCanvases>) {}

    #[unity_icall("UnityEngine.Canvas::SetExternalCanvasEnabled(System.Boolean)")]
    pub fn set_external_canvas_enabled(enabled: bool) {}

    #[unity_icall("UnityEngine.Canvas::GetDefaultCanvasTextMaterial")]
    pub fn get_default_canvas_text_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Canvas::GetDefaultCanvasMaterial")]
    pub fn get_default_canvas_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Canvas::GetETC1SupportedCanvasMaterial")]
    pub fn get_etc1supported_canvas_material() -> Option<Material> {}

    #[unity_icall("UnityEngine.Canvas::UpdateCanvasRectTransform(System.Boolean)")]
    pub fn update_canvas_rect_transform(&self, align_with_camera: bool) {}

    #[unity_method(name = "ForceUpdateCanvases", static)]
    pub fn force_update_canvases() {}

}
