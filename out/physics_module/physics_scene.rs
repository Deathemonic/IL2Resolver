#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::collider::Collider;
use super::query_trigger_interaction::QueryTriggerInteraction;
use super::raycast_hit::RaycastHit;
use crate::core_module::Ray;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "PhysicsScene", namespace = "UnityEngine", value_type)]
pub struct PhysicsScene {
    pub m_handle: i32,
}

#[unity_impl]
impl PhysicsScene {
    #[unity_method(name = "ToString")]
    pub fn to_string(&self) -> Option<SystemString> {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, other: PhysicsScene) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::IsValid_Internal(PhysicsScene)")]
    pub fn is_valid_internal(physics_scene: PhysicsScene) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::IsEmpty_Internal(PhysicsScene)")]
    pub fn is_empty_internal(physics_scene: PhysicsScene) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::IsValid_Internal_Injected(PhysicsScene&)")]
    pub fn is_valid_internal_1(physics_scene: &mut PhysicsScene) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::IsEmpty_Internal_Injected(PhysicsScene&)")]
    pub fn is_empty_internal_1(physics_scene: &mut PhysicsScene) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_RaycastTest_Injected(PhysicsScene&,Ray&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_raycast_test(physics_scene: &mut PhysicsScene, ray: &mut Ray, max_distance: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_Raycast_Injected(PhysicsScene&,Ray&,System.Single,RaycastHit&,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_raycast(physics_scene: &mut PhysicsScene, ray: &mut Ray, max_distance: f32, hit: &mut RaycastHit, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_RaycastNonAlloc_Injected(PhysicsScene&,Ray&,RaycastHit[],System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_raycast_non_alloc(physics_scene: &mut PhysicsScene, ray: &mut Ray, raycast_hits: Array<RaycastHit>, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::Query_CapsuleCast_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,Vector3&,System.Single,RaycastHit&,System.Int32,QueryTriggerInteraction)")]
    pub fn query_capsule_cast(physics_scene: &mut PhysicsScene, point1: &mut Vector3, point2: &mut Vector3, radius: f32, direction: &mut Vector3, max_distance: f32, hit_info: &mut RaycastHit, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_CapsuleCastNonAlloc_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,Vector3&,RaycastHit[],System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_capsule_cast_non_alloc(physics_scene: &mut PhysicsScene, p0: &mut Vector3, p1: &mut Vector3, radius: f32, direction: &mut Vector3, raycast_hits: Array<RaycastHit>, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::OverlapCapsuleNonAlloc_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,Collider[],System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_capsule_non_alloc_internal(physics_scene: &mut PhysicsScene, point0: &mut Vector3, point1: &mut Vector3, radius: f32, results: Array<Collider>, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::Query_SphereCast_Injected(PhysicsScene&,Vector3&,System.Single,Vector3&,System.Single,RaycastHit&,System.Int32,QueryTriggerInteraction)")]
    pub fn query_sphere_cast(physics_scene: &mut PhysicsScene, origin: &mut Vector3, radius: f32, direction: &mut Vector3, max_distance: f32, hit_info: &mut RaycastHit, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_SphereCastNonAlloc_Injected(PhysicsScene&,Vector3&,System.Single,Vector3&,RaycastHit[],System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_sphere_cast_non_alloc(physics_scene: &mut PhysicsScene, origin: &mut Vector3, radius: f32, direction: &mut Vector3, raycast_hits: Array<RaycastHit>, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::OverlapSphereNonAlloc_Internal_Injected(PhysicsScene&,Vector3&,System.Single,Collider[],System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_sphere_non_alloc_internal(physics_scene: &mut PhysicsScene, position: &mut Vector3, radius: f32, results: Array<Collider>, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::Query_BoxCast_Injected(PhysicsScene&,Vector3&,Vector3&,Vector3&,Quaternion&,System.Single,RaycastHit&,System.Int32,QueryTriggerInteraction)")]
    pub fn query_box_cast(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, direction: &mut Vector3, orientation: &mut Quaternion, max_distance: f32, out_hit: &mut RaycastHit, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.PhysicsScene::OverlapBoxNonAlloc_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,Collider[],Quaternion&,System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_box_non_alloc_internal(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, results: Array<Collider>, orientation: &mut Quaternion, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

    #[unity_icall("UnityEngine.PhysicsScene::Internal_BoxCastNonAlloc_Injected(PhysicsScene&,Vector3&,Vector3&,Vector3&,RaycastHit[],Quaternion&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_box_cast_non_alloc(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, direction: &mut Vector3, raycast_hits: Array<RaycastHit>, orientation: &mut Quaternion, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> i32 {}

}
