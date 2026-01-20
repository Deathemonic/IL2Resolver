#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector3, Vector4};
use super::canvas::Canvas;
use crate::core_module::{Bounds, Camera, Ray, Rect, RectTransform, Transform};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.UIModule", class = "RectTransformUtility", namespace = "UnityEngine")]
pub struct RectTransformUtility(pub *mut c_void);

#[unity_impl]
impl RectTransformUtility {
    #[unity_method(name = "ScreenPointToWorldPointInRectangle", static)]
    pub fn screen_point_to_world_point_in_rectangle(rect: Option<RectTransform>, screen_point: Vector2, cam: Option<Camera>, world_point: &mut Vector3) -> bool {}

    #[unity_method(name = "ScreenPointToLocalPointInRectangle", static)]
    pub fn screen_point_to_local_point_in_rectangle(rect: Option<RectTransform>, screen_point: Vector2, cam: Option<Camera>, local_point: &mut Vector2) -> bool {}

    #[unity_method(name = "ScreenPointToRay", static)]
    pub fn screen_point_to_ray(cam: Option<Camera>, screen_pos: Vector2) -> Ray {}

    #[unity_method(name = "WorldToScreenPoint", static)]
    pub fn world_to_screen_point(cam: Option<Camera>, world_point: Vector3) -> Vector2 {}

    #[unity_method(name = "CalculateRelativeRectTransformBounds", static)]
    pub fn calculate_relative_rect_transform_bounds(root: Option<Transform>, child: Option<Transform>) -> Bounds {}

    #[unity_method(name = "CalculateRelativeRectTransformBounds", static)]
    pub fn calculate_relative_rect_transform_bounds_1(trans: Option<Transform>) -> Bounds {}

    #[unity_method(name = "FlipLayoutOnAxis", static)]
    pub fn flip_layout_on_axis(rect: Option<RectTransform>, axis: i32, keep_positioning: bool, recursive: bool) {}

    #[unity_method(name = "FlipLayoutAxes", static)]
    pub fn flip_layout_axes(rect: Option<RectTransform>, keep_positioning: bool, recursive: bool) {}

    #[unity_icall("UnityEngine.RectTransformUtility::PixelAdjustPoint_Injected(Vector2&,Transform,Canvas,Vector2&)")]
    pub fn pixel_adjust_point(point: &mut Vector2, element_transform: Option<Transform>, canvas: Option<Canvas>, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.RectTransformUtility::PixelAdjustRect_Injected(RectTransform,Canvas,Rect&)")]
    pub fn pixel_adjust_rect(rect_transform: Option<RectTransform>, canvas: Option<Canvas>, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.RectTransformUtility::PointInRectangle_Injected(Vector2&,RectTransform,Camera,Vector4&)")]
    pub fn point_in_rectangle(screen_point: &mut Vector2, rect: Option<RectTransform>, cam: Option<Camera>, offset: &mut Vector4) -> bool {}

}
