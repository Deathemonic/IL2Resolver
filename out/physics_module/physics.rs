#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::collections::{Array};
use super::collider::Collider;
use super::physics_scene::PhysicsScene;
use super::query_trigger_interaction::QueryTriggerInteraction;
use super::raycast_hit::RaycastHit;
use crate::core_module::{Bounds, Ray};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "Physics", namespace = "UnityEngine")]
pub struct Physics(pub *mut c_void);

#[unity_impl]
impl Physics {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_minPenetrationForPenalty", static)]
    pub fn get_min_penetration_for_penalty() -> f32 {}

    #[unity_method(name = "set_minPenetrationForPenalty", static)]
    pub fn set_min_penetration_for_penalty(value: f32) {}

    #[unity_method(name = "get_bounceTreshold", static)]
    pub fn get_bounce_treshold() -> f32 {}

    #[unity_method(name = "set_bounceTreshold", static)]
    pub fn set_bounce_treshold(value: f32) {}

    #[unity_method(name = "get_sleepVelocity", static)]
    pub fn get_sleep_velocity() -> f32 {}

    #[unity_method(name = "set_sleepVelocity", static)]
    pub fn set_sleep_velocity(value: f32) {}

    #[unity_method(name = "get_sleepAngularVelocity", static)]
    pub fn get_sleep_angular_velocity() -> f32 {}

    #[unity_method(name = "set_sleepAngularVelocity", static)]
    pub fn set_sleep_angular_velocity(value: f32) {}

    #[unity_method(name = "get_maxAngularVelocity", static)]
    pub fn get_max_angular_velocity() -> f32 {}

    #[unity_method(name = "set_maxAngularVelocity", static)]
    pub fn set_max_angular_velocity(value: f32) {}

    #[unity_method(name = "get_solverIterationCount", static)]
    pub fn get_solver_iteration_count() -> i32 {}

    #[unity_method(name = "set_solverIterationCount", static)]
    pub fn set_solver_iteration_count(value: i32) {}

    #[unity_method(name = "get_solverVelocityIterationCount", static)]
    pub fn get_solver_velocity_iteration_count() -> i32 {}

    #[unity_method(name = "set_solverVelocityIterationCount", static)]
    pub fn set_solver_velocity_iteration_count(value: i32) {}

    #[unity_method(name = "get_penetrationPenaltyForce", static)]
    pub fn get_penetration_penalty_force() -> f32 {}

    #[unity_method(name = "set_penetrationPenaltyForce", static)]
    pub fn set_penetration_penalty_force(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_gravity_Injected(Vector3&)")]
    pub fn get_gravity(ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Physics::set_gravity_Injected(Vector3&)")]
    pub fn set_gravity(value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Physics::get_defaultContactOffset")]
    pub fn get_default_contact_offset() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_defaultContactOffset(System.Single)")]
    pub fn set_default_contact_offset(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_sleepThreshold")]
    pub fn get_sleep_threshold() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_sleepThreshold(System.Single)")]
    pub fn set_sleep_threshold(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_queriesHitTriggers")]
    pub fn get_queries_hit_triggers() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_queriesHitTriggers(System.Boolean)")]
    pub fn set_queries_hit_triggers(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_queriesHitBackfaces")]
    pub fn get_queries_hit_backfaces() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_queriesHitBackfaces(System.Boolean)")]
    pub fn set_queries_hit_backfaces(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_bounceThreshold")]
    pub fn get_bounce_threshold() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_bounceThreshold(System.Single)")]
    pub fn set_bounce_threshold(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_defaultMaxDepenetrationVelocity")]
    pub fn get_default_max_depenetration_velocity() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_defaultMaxDepenetrationVelocity(System.Single)")]
    pub fn set_default_max_depenetration_velocity(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_defaultSolverIterations")]
    pub fn get_default_solver_iterations() -> i32 {}

    #[unity_icall("UnityEngine.Physics::set_defaultSolverIterations(System.Int32)")]
    pub fn set_default_solver_iterations(value: i32) {}

    #[unity_icall("UnityEngine.Physics::get_defaultSolverVelocityIterations")]
    pub fn get_default_solver_velocity_iterations() -> i32 {}

    #[unity_icall("UnityEngine.Physics::set_defaultSolverVelocityIterations(System.Int32)")]
    pub fn set_default_solver_velocity_iterations(value: i32) {}

    #[unity_icall("UnityEngine.Physics::get_defaultMaxAngularSpeed")]
    pub fn get_default_max_angular_speed() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_defaultMaxAngularSpeed(System.Single)")]
    pub fn set_default_max_angular_speed(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_improvedPatchFriction")]
    pub fn get_improved_patch_friction() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_improvedPatchFriction(System.Boolean)")]
    pub fn set_improved_patch_friction(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_defaultPhysicsScene_Injected(PhysicsScene&)")]
    pub fn get_default_physics_scene(ret: &mut PhysicsScene) {}

    #[unity_icall("UnityEngine.Physics::get_autoSimulation")]
    pub fn get_auto_simulation() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_autoSimulation(System.Boolean)")]
    pub fn set_auto_simulation(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_autoSyncTransforms")]
    pub fn get_auto_sync_transforms() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_autoSyncTransforms(System.Boolean)")]
    pub fn set_auto_sync_transforms(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_reuseCollisionCallbacks")]
    pub fn get_reuse_collision_callbacks() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_reuseCollisionCallbacks(System.Boolean)")]
    pub fn set_reuse_collision_callbacks(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_interCollisionDistance")]
    pub fn get_inter_collision_distance() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_interCollisionDistance(System.Single)")]
    pub fn set_inter_collision_distance(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_interCollisionStiffness")]
    pub fn get_inter_collision_stiffness() -> f32 {}

    #[unity_icall("UnityEngine.Physics::set_interCollisionStiffness(System.Single)")]
    pub fn set_inter_collision_stiffness(value: f32) {}

    #[unity_icall("UnityEngine.Physics::get_interCollisionSettingsToggle")]
    pub fn get_inter_collision_settings_toggle() -> bool {}

    #[unity_icall("UnityEngine.Physics::set_interCollisionSettingsToggle(System.Boolean)")]
    pub fn set_inter_collision_settings_toggle(value: bool) {}

    #[unity_icall("UnityEngine.Physics::get_clothGravity_Injected(Vector3&)")]
    pub fn get_cloth_gravity(ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Physics::set_clothGravity_Injected(Vector3&)")]
    pub fn set_cloth_gravity(value: &mut Vector3) {}

    #[unity_method(name = "add_ContactModifyEvent", static)]
    pub fn add_contact_modify_event(value: *mut c_void) {}

    #[unity_method(name = "remove_ContactModifyEvent", static)]
    pub fn remove_contact_modify_event(value: *mut c_void) {}

    #[unity_method(name = "add_ContactModifyEventCCD", static)]
    pub fn add_contact_modify_event_ccd(value: *mut c_void) {}

    #[unity_method(name = "remove_ContactModifyEventCCD", static)]
    pub fn remove_contact_modify_event_ccd(value: *mut c_void) {}

    #[unity_icall("UnityEngine.Physics::IgnoreCollision(Collider,Collider,System.Boolean)")]
    pub fn ignore_collision(collider1: Option<Collider>, collider2: Option<Collider>, ignore: bool) {}

    #[unity_icall("UnityEngine.Physics::IgnoreLayerCollision(System.Int32,System.Int32,System.Boolean)")]
    pub fn ignore_layer_collision(layer1: i32, layer2: i32, ignore: bool) {}

    #[unity_icall("UnityEngine.Physics::GetIgnoreLayerCollision(System.Int32,System.Int32)")]
    pub fn get_ignore_layer_collision(layer1: i32, layer2: i32) -> bool {}

    #[unity_icall("UnityEngine.Physics::GetIgnoreCollision(Collider,Collider)")]
    pub fn get_ignore_collision(collider1: Option<Collider>, collider2: Option<Collider>) -> bool {}

    #[unity_icall("UnityEngine.Physics::SyncTransforms")]
    pub fn sync_transforms() {}

    #[unity_icall("UnityEngine.Physics::BakeMesh(System.Int32,System.Boolean)")]
    pub fn bake_mesh(mesh_id: i32, convex: bool) {}

    #[unity_icall("UnityEngine.Physics::Internal_RaycastAll_Injected(PhysicsScene&,Ray&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_raycast_all(physics_scene: &mut PhysicsScene, ray: &mut Ray, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<RaycastHit> {}

    #[unity_icall("UnityEngine.Physics::Query_CapsuleCastAll_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn query_capsule_cast_all(physics_scene: &mut PhysicsScene, p0: &mut Vector3, p1: &mut Vector3, radius: f32, direction: &mut Vector3, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<RaycastHit> {}

    #[unity_icall("UnityEngine.Physics::Query_SphereCastAll_Injected(PhysicsScene&,Vector3&,System.Single,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn query_sphere_cast_all(physics_scene: &mut PhysicsScene, origin: &mut Vector3, radius: f32, direction: &mut Vector3, max_distance: f32, mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<RaycastHit> {}

    #[unity_icall("UnityEngine.Physics::OverlapCapsule_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_capsule_internal(physics_scene: &mut PhysicsScene, point0: &mut Vector3, point1: &mut Vector3, radius: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<Collider> {}

    #[unity_icall("UnityEngine.Physics::OverlapSphere_Internal_Injected(PhysicsScene&,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_sphere_internal(physics_scene: &mut PhysicsScene, position: &mut Vector3, radius: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<Collider> {}

    #[unity_icall("UnityEngine.Physics::Simulate_Internal_Injected(PhysicsScene&,System.Single)")]
    pub fn simulate_internal(physics_scene: &mut PhysicsScene, step: f32) {}

    #[unity_icall("UnityEngine.Physics::Query_ComputePenetration_Injected(Collider,Vector3&,Quaternion&,Collider,Vector3&,Quaternion&,Vector3&,System.Single&)")]
    pub fn query_compute_penetration(collider_a: Option<Collider>, position_a: &mut Vector3, rotation_a: &mut Quaternion, collider_b: Option<Collider>, position_b: &mut Vector3, rotation_b: &mut Quaternion, direction: &mut Vector3, distance: &mut f32) -> bool {}

    #[unity_icall("UnityEngine.Physics::Query_ClosestPoint_Injected(Collider,Vector3&,Quaternion&,Vector3&,Vector3&)")]
    pub fn query_closest_point(collider: Option<Collider>, position: &mut Vector3, rotation: &mut Quaternion, point: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Physics::CheckSphere_Internal_Injected(PhysicsScene&,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn check_sphere_internal(physics_scene: &mut PhysicsScene, position: &mut Vector3, radius: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.Physics::CheckCapsule_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn check_capsule_internal(physics_scene: &mut PhysicsScene, start: &mut Vector3, end: &mut Vector3, radius: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.Physics::CheckBox_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,Quaternion&,System.Int32,QueryTriggerInteraction)")]
    pub fn check_box_internal(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, orientation: &mut Quaternion, layermask: i32, query_trigger_interaction: QueryTriggerInteraction) -> bool {}

    #[unity_icall("UnityEngine.Physics::OverlapBox_Internal_Injected(PhysicsScene&,Vector3&,Vector3&,Quaternion&,System.Int32,QueryTriggerInteraction)")]
    pub fn overlap_box_internal(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, orientation: &mut Quaternion, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<Collider> {}

    #[unity_icall("UnityEngine.Physics::Internal_BoxCastAll_Injected(PhysicsScene&,Vector3&,Vector3&,Vector3&,Quaternion&,System.Single,System.Int32,QueryTriggerInteraction)")]
    pub fn internal_box_cast_all(physics_scene: &mut PhysicsScene, center: &mut Vector3, half_extents: &mut Vector3, direction: &mut Vector3, orientation: &mut Quaternion, max_distance: f32, layer_mask: i32, query_trigger_interaction: QueryTriggerInteraction) -> Array<RaycastHit> {}

    #[unity_icall("UnityEngine.Physics::Internal_RebuildBroadphaseRegions_Injected(Bounds&,System.Int32)")]
    pub fn internal_rebuild_broadphase_regions(bounds: &mut Bounds, subdivisions: i32) {}

}
