#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Vector3};
use super::articulation_body::ArticulationBody;
use super::physic_material::PhysicMaterial;
use super::raycast_hit::RaycastHit;
use super::rigidbody::Rigidbody;
use crate::core_module::{Bounds, Ray};
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "Collider", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct Collider(pub *mut c_void);

#[unity_impl]
impl Collider {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Collider::get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.Collider::set_enabled(System.Boolean)")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_icall("UnityEngine.Collider::get_attachedRigidbody")]
    pub fn get_attached_rigidbody(&self) -> Option<Rigidbody> {}

    #[unity_icall("UnityEngine.Collider::get_attachedArticulationBody")]
    pub fn get_attached_articulation_body(&self) -> Option<ArticulationBody> {}

    #[unity_icall("UnityEngine.Collider::get_isTrigger")]
    pub fn get_is_trigger(&self) -> bool {}

    #[unity_icall("UnityEngine.Collider::set_isTrigger(System.Boolean)")]
    pub fn set_is_trigger(&self, value: bool) {}

    #[unity_icall("UnityEngine.Collider::get_contactOffset")]
    pub fn get_contact_offset(&self) -> f32 {}

    #[unity_icall("UnityEngine.Collider::set_contactOffset(System.Single)")]
    pub fn set_contact_offset(&self, value: f32) {}

    #[unity_icall("UnityEngine.Collider::get_bounds_Injected(Bounds&)")]
    pub fn get_bounds(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Collider::get_hasModifiableContacts")]
    pub fn get_has_modifiable_contacts(&self) -> bool {}

    #[unity_icall("UnityEngine.Collider::set_hasModifiableContacts(System.Boolean)")]
    pub fn set_has_modifiable_contacts(&self, value: bool) {}

    #[unity_icall("UnityEngine.Collider::get_sharedMaterial")]
    pub fn get_shared_material(&self) -> Option<PhysicMaterial> {}

    #[unity_icall("UnityEngine.Collider::set_sharedMaterial(PhysicMaterial)")]
    pub fn set_shared_material(&self, value: Option<PhysicMaterial>) {}

    #[unity_icall("UnityEngine.Collider::get_material")]
    pub fn get_material(&self) -> Option<PhysicMaterial> {}

    #[unity_icall("UnityEngine.Collider::set_material(PhysicMaterial)")]
    pub fn set_material(&self, value: Option<PhysicMaterial>) {}

    #[unity_icall("UnityEngine.Collider::ClosestPoint_Injected(Vector3&,Vector3&)")]
    pub fn closest_point(&self, position: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Collider::Raycast_Injected(Ray&,System.Single,System.Boolean&,RaycastHit&)")]
    pub fn raycast(&self, ray: &mut Ray, max_distance: f32, has_hit: &mut bool, ret: &mut RaycastHit) {}

    #[unity_icall("UnityEngine.Collider::Internal_ClosestPointOnBounds_Injected(Vector3&,Vector3&,System.Single&)")]
    pub fn internal_closest_point_on_bounds(&self, point: &mut Vector3, out_pos: &mut Vector3, distance: &mut f32) {}

}
