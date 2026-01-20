#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use super::collision_flags::CollisionFlags;
use crate::core_module::{Component, Object};
use crate::physics_module::Collider;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "CharacterController", namespace = "UnityEngine", inherit = "Collider,Component,Object")]
pub struct CharacterController(pub *mut c_void);

#[unity_impl]
impl CharacterController {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.CharacterController::get_velocity_Injected(Vector3&)")]
    pub fn get_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.CharacterController::get_isGrounded")]
    pub fn get_is_grounded(&self) -> bool {}

    #[unity_icall("UnityEngine.CharacterController::get_collisionFlags")]
    pub fn get_collision_flags(&self) -> CollisionFlags {}

    #[unity_icall("UnityEngine.CharacterController::get_radius")]
    pub fn get_radius(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_radius(System.Single)")]
    pub fn set_radius(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_height")]
    pub fn get_height(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_height(System.Single)")]
    pub fn set_height(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_center_Injected(Vector3&)")]
    pub fn get_center(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.CharacterController::set_center_Injected(Vector3&)")]
    pub fn set_center(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.CharacterController::get_slopeLimit")]
    pub fn get_slope_limit(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_slopeLimit(System.Single)")]
    pub fn set_slope_limit(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_stepOffset")]
    pub fn get_step_offset(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_stepOffset(System.Single)")]
    pub fn set_step_offset(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_skinWidth")]
    pub fn get_skin_width(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_skinWidth(System.Single)")]
    pub fn set_skin_width(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_minMoveDistance")]
    pub fn get_min_move_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.CharacterController::set_minMoveDistance(System.Single)")]
    pub fn set_min_move_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.CharacterController::get_detectCollisions")]
    pub fn get_detect_collisions(&self) -> bool {}

    #[unity_icall("UnityEngine.CharacterController::set_detectCollisions(System.Boolean)")]
    pub fn set_detect_collisions(&self, value: bool) {}

    #[unity_icall("UnityEngine.CharacterController::get_enableOverlapRecovery")]
    pub fn get_enable_overlap_recovery(&self) -> bool {}

    #[unity_icall("UnityEngine.CharacterController::set_enableOverlapRecovery(System.Boolean)")]
    pub fn set_enable_overlap_recovery(&self, value: bool) {}

    #[unity_icall("UnityEngine.CharacterController::SimpleMove_Injected(Vector3&)")]
    pub fn simple_move(&self, speed: &mut Vector3) -> bool {}

    #[unity_icall("UnityEngine.CharacterController::Move_Injected(Vector3&)")]
    pub fn move_value(&self, motion: &mut Vector3) -> CollisionFlags {}

}
