#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector2, Vector4};
use crate::mscorlib::collections::{Array, List};
use super::bounds::Bounds;
use super::rect::Rect;
use super::sprite_mesh_type::SpriteMeshType;
use super::sprite_packing_mode::SpritePackingMode;
use super::sprite_packing_rotation::SpritePackingRotation;
use super::texture2d::Texture2D;
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Sprite", namespace = "UnityEngine", inherit = "Object")]
pub struct Sprite(pub *mut c_void);

#[unity_impl]
impl Sprite {
    #[unity_icall("UnityEngine.Sprite::get_bounds_Injected(Bounds&)")]
    pub fn get_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Sprite::get_rect_Injected(Rect&)")]
    pub fn get_rect(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Sprite::get_border_Injected(Vector4&)")]
    pub fn get_border(&self, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Sprite::get_texture")]
    pub fn get_texture(&self) -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Sprite::get_pixelsPerUnit")]
    pub fn get_pixels_per_unit(&self) -> f32 {}

    #[unity_icall("UnityEngine.Sprite::get_spriteAtlasTextureScale")]
    pub fn get_sprite_atlas_texture_scale(&self) -> f32 {}

    #[unity_icall("UnityEngine.Sprite::get_associatedAlphaSplitTexture")]
    pub fn get_associated_alpha_split_texture(&self) -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Sprite::get_pivot")]
    pub fn get_pivot(&self) -> Vector2 {}

    #[unity_method(name = "get_packed")]
    pub fn get_packed(&self) -> bool {}

    #[unity_method(name = "get_packingMode")]
    pub fn get_packing_mode(&self) -> SpritePackingMode {}

    #[unity_method(name = "get_packingRotation")]
    pub fn get_packing_rotation(&self) -> SpritePackingRotation {}

    #[unity_method(name = "get_textureRect")]
    pub fn get_texture_rect(&self) -> Rect {}

    #[unity_method(name = "get_textureRectOffset")]
    pub fn get_texture_rect_offset(&self) -> Vector2 {}

    #[unity_icall("UnityEngine.Sprite::get_vertices")]
    pub fn get_vertices(&self) -> Array<Vector2> {}

    #[unity_icall("UnityEngine.Sprite::get_triangles")]
    pub fn get_triangles(&self) -> Array<u16> {}

    #[unity_icall("UnityEngine.Sprite::get_uv")]
    pub fn get_uv(&self) -> Array<Vector2> {}

    #[unity_icall("UnityEngine.Sprite::CreateSpriteWithoutTextureScripting(Rect,Vector2,System.Single,Texture2D)")]
    pub fn create_sprite_without_texture_scripting(rect: Rect, pivot: Vector2, pixels_to_units: f32, texture: Option<Texture2D>) -> Option<Sprite> {}

    #[unity_icall("UnityEngine.Sprite::CreateSprite(Texture2D,Rect,Vector2,System.Single,System.UInt32,SpriteMeshType,Vector4,System.Boolean)")]
    pub fn create_sprite(texture: Option<Texture2D>, rect: Rect, pivot: Vector2, pixels_per_unit: f32, extrude: u32, mesh_type: SpriteMeshType, border: Vector4, generate_fallback_physics_shape: bool) -> Option<Sprite> {}

    #[unity_icall("UnityEngine.Sprite::GetSecondaryTexture(System.Int32)")]
    pub fn get_secondary_texture(&self, index: i32) -> Option<Texture2D> {}

    #[unity_icall("UnityEngine.Sprite::GetPhysicsShapeCount")]
    pub fn get_physics_shape_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Sprite::Internal_GetPhysicsShapePointCount(System.Int32)")]
    pub fn internal_get_physics_shape_point_count(&self, shape_idx: i32) -> i32 {}

    #[unity_icall("UnityEngine.Sprite::GetPhysicsShapeImpl(Sprite,System.Int32,List<Vector2>)")]
    pub fn get_physics_shape_impl(sprite: Option<Sprite>, shape_idx: i32, physics_shape: List<Vector2>) {}

    #[unity_icall("UnityEngine.Sprite::OverridePhysicsShapeCount(Sprite,System.Int32)")]
    pub fn override_physics_shape_count(sprite: Option<Sprite>, physics_shape_count: i32) {}

    #[unity_icall("UnityEngine.Sprite::OverridePhysicsShape(Sprite,Vector2[],System.Int32)")]
    pub fn override_physics_shape(sprite: Option<Sprite>, physics_shape: Array<Vector2>, idx: i32) {}

    #[unity_icall("UnityEngine.Sprite::OverrideGeometry(Vector2[],System.UInt16[])")]
    pub fn override_geometry(&self, vertices: Array<Vector2>, triangles: Array<u16>) {}

    #[unity_icall("UnityEngine.Sprite::GetTextureRect_Injected(Rect&)")]
    pub fn get_texture_rect_1(&self, ret: &mut Rect) {}

    #[unity_icall("UnityEngine.Sprite::GetTextureRectOffset_Injected(Vector2&)")]
    pub fn get_texture_rect_offset_1(&self, ret: &mut Vector2) {}

    #[unity_icall("UnityEngine.Sprite::GetInnerUVs_Injected(Vector4&)")]
    pub fn get_inner_u_vs(&self, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Sprite::GetOuterUVs_Injected(Vector4&)")]
    pub fn get_outer_u_vs(&self, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Sprite::GetPadding_Injected(Vector4&)")]
    pub fn get_padding(&self, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Sprite::CreateSpriteWithoutTextureScripting_Injected(Rect&,Vector2&,System.Single,Texture2D)")]
    pub fn create_sprite_without_texture_scripting_1(rect: &mut Rect, pivot: &mut Vector2, pixels_to_units: f32, texture: Option<Texture2D>) -> Option<Sprite> {}

    #[unity_icall("UnityEngine.Sprite::CreateSprite_Injected(Texture2D,Rect&,Vector2&,System.Single,System.UInt32,SpriteMeshType,Vector4&,System.Boolean)")]
    pub fn create_sprite_1(texture: Option<Texture2D>, rect: &mut Rect, pivot: &mut Vector2, pixels_per_unit: f32, extrude: u32, mesh_type: SpriteMeshType, border: &mut Vector4, generate_fallback_physics_shape: bool) -> Option<Sprite> {}

    #[unity_icall("UnityEngine.Sprite::get_pivot_Injected(Vector2&)")]
    pub fn get_pivot_1(&self, ret: &mut Vector2) {}

}
