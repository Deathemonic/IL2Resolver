#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector4};
use super::canvas_update::CanvasUpdate;
use crate::core_module::{Camera, Color, Material, Rect, RectTransform, Texture, UnityAction};
use crate::ui_module::{Canvas, CanvasRenderer};
use crate::core_module::{Behaviour, Component, MonoBehaviour, Object};
use crate::ui::UIBehaviour;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UI", class = "Graphic", namespace = "UnityEngine.UI", inherit = "UIBehaviour,MonoBehaviour,Behaviour,Component,Object")]
pub struct Graphic(pub *mut c_void);

#[unity_impl]
impl Graphic {
    #[unity_method(name = "get_defaultGraphicMaterial", static)]
    pub fn get_default_graphic_material() -> Option<Material> {}

    #[unity_method(name = "get_color")]
    pub fn get_color(&self) -> Color {}

    #[unity_method(name = "set_color")]
    pub fn set_color(&self, value: Color) {}

    #[unity_method(name = "get_raycastTarget")]
    pub fn get_raycast_target(&self) -> bool {}

    #[unity_method(name = "set_raycastTarget")]
    pub fn set_raycast_target(&self, value: bool) {}

    #[unity_method(name = "get_raycastPadding")]
    pub fn get_raycast_padding(&self) -> Vector4 {}

    #[unity_method(name = "set_raycastPadding")]
    pub fn set_raycast_padding(&self, value: Vector4) {}

    #[unity_method(name = "get_depth")]
    pub fn get_depth(&self) -> i32 {}

    #[unity_method(name = "get_rectTransform")]
    pub fn get_rect_transform(&self) -> Option<RectTransform> {}

    #[unity_method(name = "get_canvas")]
    pub fn get_canvas(&self) -> Option<Canvas> {}

    #[unity_method(name = "get_canvasRenderer")]
    pub fn get_canvas_renderer(&self) -> Option<CanvasRenderer> {}

    #[unity_method(name = "get_defaultMaterial")]
    pub fn get_default_material(&self) -> Option<Material> {}

    #[unity_method(name = "get_material")]
    pub fn get_material(&self) -> Option<Material> {}

    #[unity_method(name = "set_material")]
    pub fn set_material(&self, value: Option<Material>) {}

    #[unity_method(name = "get_materialForRendering")]
    pub fn get_material_for_rendering(&self) -> Option<Material> {}

    #[unity_method(name = "get_mainTexture")]
    pub fn get_main_texture(&self) -> Option<Texture> {}

    #[unity_method(name = "SetAllDirty")]
    pub fn set_all_dirty(&self) {}

    #[unity_method(name = "SetLayoutDirty")]
    pub fn set_layout_dirty(&self) {}

    #[unity_method(name = "SetVerticesDirty")]
    pub fn set_vertices_dirty(&self) {}

    #[unity_method(name = "SetMaterialDirty")]
    pub fn set_material_dirty(&self) {}

    #[unity_method(name = "SetRaycastDirty")]
    pub fn set_raycast_dirty(&self) {}

    #[unity_method(name = "OnCullingChanged")]
    pub fn on_culling_changed(&self) {}

    #[unity_method(name = "Rebuild")]
    pub fn rebuild(&self, update: CanvasUpdate) {}

    #[unity_method(name = "LayoutComplete")]
    pub fn layout_complete(&self) {}

    #[unity_method(name = "GraphicUpdateComplete")]
    pub fn graphic_update_complete(&self) {}

    #[unity_method(name = "OnRebuildRequested")]
    pub fn on_rebuild_requested(&self) {}

    #[unity_method(name = "SetNativeSize")]
    pub fn set_native_size(&self) {}

    #[unity_method(name = "Raycast")]
    pub fn raycast(&self, sp: Vector2, event_camera: Option<Camera>) -> bool {}

    #[unity_method(name = "PixelAdjustPoint")]
    pub fn pixel_adjust_point(&self, point: Vector2) -> Vector2 {}

    #[unity_method(name = "GetPixelAdjustedRect")]
    pub fn get_pixel_adjusted_rect(&self) -> Rect {}

    #[unity_method(name = "CrossFadeColor")]
    pub fn cross_fade_color(&self, target_color: Color, duration: f32, ignore_time_scale: bool, use_alpha: bool) {}

    #[unity_method(name = "CrossFadeColor")]
    pub fn cross_fade_color_1(&self, target_color: Color, duration: f32, ignore_time_scale: bool, use_alpha: bool, use_rgb: bool) {}

    #[unity_method(name = "CrossFadeAlpha")]
    pub fn cross_fade_alpha(&self, alpha: f32, duration: f32, ignore_time_scale: bool) {}

    #[unity_method(name = "RegisterDirtyLayoutCallback")]
    pub fn register_dirty_layout_callback(&self, action: Option<UnityAction>) {}

    #[unity_method(name = "UnregisterDirtyLayoutCallback")]
    pub fn unregister_dirty_layout_callback(&self, action: Option<UnityAction>) {}

    #[unity_method(name = "RegisterDirtyVerticesCallback")]
    pub fn register_dirty_vertices_callback(&self, action: Option<UnityAction>) {}

    #[unity_method(name = "UnregisterDirtyVerticesCallback")]
    pub fn unregister_dirty_vertices_callback(&self, action: Option<UnityAction>) {}

    #[unity_method(name = "RegisterDirtyMaterialCallback")]
    pub fn register_dirty_material_callback(&self, action: Option<UnityAction>) {}

    #[unity_method(name = "UnregisterDirtyMaterialCallback")]
    pub fn unregister_dirty_material_callback(&self, action: Option<UnityAction>) {}

}
