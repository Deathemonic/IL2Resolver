#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use super::modifiable_mass_properties::ModifiableMassProperties;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ModifiableContactPair", namespace = "UnityEngine", value_type)]
pub struct ModifiableContactPair {
    pub actor: isize,
    pub other_actor: isize,
    pub shape: isize,
    pub other_shape: isize,
    pub rotation: Quaternion,
    pub position: Vector3,
    pub other_rotation: Quaternion,
    pub other_position: Vector3,
    pub num_contacts: i32,
    pub contacts: isize,
}

#[unity_impl]
impl ModifiableContactPair {
    #[unity_method(name = "get_colliderInstanceID")]
    pub fn get_collider_instance_id(&self) -> i32 {}

    #[unity_method(name = "get_otherColliderInstanceID")]
    pub fn get_other_collider_instance_id(&self) -> i32 {}

    #[unity_method(name = "get_bodyInstanceID")]
    pub fn get_body_instance_id(&self) -> i32 {}

    #[unity_method(name = "get_otherBodyInstanceID")]
    pub fn get_other_body_instance_id(&self) -> i32 {}

    #[unity_method(name = "get_contactCount")]
    pub fn get_contact_count(&self) -> i32 {}

    #[unity_method(name = "get_massProperties")]
    pub fn get_mass_properties(&self) -> ModifiableMassProperties {}

    #[unity_method(name = "set_massProperties")]
    pub fn set_mass_properties(&self, value: ModifiableMassProperties) {}

    #[unity_method(name = "GetPoint")]
    pub fn get_point(&self, i: i32) -> Vector3 {}

    #[unity_method(name = "SetPoint")]
    pub fn set_point(&self, i: i32, v: Vector3) {}

    #[unity_method(name = "GetNormal")]
    pub fn get_normal(&self, i: i32) -> Vector3 {}

    #[unity_method(name = "SetNormal")]
    pub fn set_normal(&self, i: i32, normal: Vector3) {}

    #[unity_method(name = "GetSeparation")]
    pub fn get_separation(&self, i: i32) -> f32 {}

    #[unity_method(name = "SetSeparation")]
    pub fn set_separation(&self, i: i32, separation: f32) {}

    #[unity_method(name = "GetTargetVelocity")]
    pub fn get_target_velocity(&self, i: i32) -> Vector3 {}

    #[unity_method(name = "SetTargetVelocity")]
    pub fn set_target_velocity(&self, i: i32, velocity: Vector3) {}

    #[unity_method(name = "GetBounciness")]
    pub fn get_bounciness(&self, i: i32) -> f32 {}

    #[unity_method(name = "SetBounciness")]
    pub fn set_bounciness(&self, i: i32, bounciness: f32) {}

    #[unity_method(name = "GetStaticFriction")]
    pub fn get_static_friction(&self, i: i32) -> f32 {}

    #[unity_method(name = "SetStaticFriction")]
    pub fn set_static_friction(&self, i: i32, static_friction: f32) {}

    #[unity_method(name = "GetDynamicFriction")]
    pub fn get_dynamic_friction(&self, i: i32) -> f32 {}

    #[unity_method(name = "SetDynamicFriction")]
    pub fn set_dynamic_friction(&self, i: i32, dynamic_friction: f32) {}

    #[unity_method(name = "GetMaxImpulse")]
    pub fn get_max_impulse(&self, i: i32) -> f32 {}

    #[unity_method(name = "SetMaxImpulse")]
    pub fn set_max_impulse(&self, i: i32, value: f32) {}

    #[unity_method(name = "IgnoreContact")]
    pub fn ignore_contact(&self, i: i32) {}

    #[unity_icall("UnityEngine.ModifiableContactPair::ResolveColliderInstanceID(System.IntPtr)")]
    pub fn resolve_collider_instance_id(shape_ptr: isize) -> i32 {}

    #[unity_icall("UnityEngine.ModifiableContactPair::ResolveBodyInstanceID(System.IntPtr)")]
    pub fn resolve_body_instance_id(actor_ptr: isize) -> i32 {}

    #[unity_icall("UnityEngine.ModifiableContactPair::TranslateTriangleIndex(System.IntPtr,System.UInt32)")]
    pub fn translate_triangle_index(shape_ptr: isize, raw_index: u32) -> u32 {}

}
