#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::collections::{Array};
use super::collision_detection_mode::CollisionDetectionMode;
use super::force_mode::ForceMode;
use super::query_trigger_interaction::QueryTriggerInteraction;
use super::raycast_hit::RaycastHit;
use super::rigidbody_constraints::RigidbodyConstraints;
use super::rigidbody_interpolation::RigidbodyInterpolation;
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "Rigidbody", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct Rigidbody(pub *mut c_void);

#[unity_impl]
impl Rigidbody {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_method(name = "get_sleepVelocity")]
    pub fn get_sleep_velocity(&self) -> f32 {}

    #[unity_method(name = "set_sleepVelocity")]
    pub fn set_sleep_velocity(&self, value: f32) {}

    #[unity_method(name = "get_sleepAngularVelocity")]
    pub fn get_sleep_angular_velocity(&self) -> f32 {}

    #[unity_method(name = "set_sleepAngularVelocity")]
    pub fn set_sleep_angular_velocity(&self, value: f32) {}

    #[unity_method(name = "get_useConeFriction")]
    pub fn get_use_cone_friction(&self) -> bool {}

    #[unity_method(name = "set_useConeFriction")]
    pub fn set_use_cone_friction(&self, value: bool) {}

    #[unity_method(name = "get_solverIterationCount")]
    pub fn get_solver_iteration_count(&self) -> i32 {}

    #[unity_method(name = "set_solverIterationCount")]
    pub fn set_solver_iteration_count(&self, value: i32) {}

    #[unity_method(name = "get_solverVelocityIterationCount")]
    pub fn get_solver_velocity_iteration_count(&self) -> i32 {}

    #[unity_method(name = "set_solverVelocityIterationCount")]
    pub fn set_solver_velocity_iteration_count(&self, value: i32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_velocity_Injected(Vector3&)")]
    pub fn get_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::set_velocity_Injected(Vector3&)")]
    pub fn set_velocity(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_angularVelocity_Injected(Vector3&)")]
    pub fn get_angular_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::set_angularVelocity_Injected(Vector3&)")]
    pub fn set_angular_velocity(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_drag")]
    pub fn get_drag(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_drag(System.Single)")]
    pub fn set_drag(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_angularDrag")]
    pub fn get_angular_drag(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_angularDrag(System.Single)")]
    pub fn set_angular_drag(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_mass")]
    pub fn get_mass(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_mass(System.Single)")]
    pub fn set_mass(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_useGravity")]
    pub fn get_use_gravity(&self) -> bool {}

    #[unity_icall("UnityEngine.Rigidbody::set_useGravity(System.Boolean)")]
    pub fn set_use_gravity(&self, value: bool) {}

    #[unity_icall("UnityEngine.Rigidbody::get_maxDepenetrationVelocity")]
    pub fn get_max_depenetration_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_maxDepenetrationVelocity(System.Single)")]
    pub fn set_max_depenetration_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_isKinematic")]
    pub fn get_is_kinematic(&self) -> bool {}

    #[unity_icall("UnityEngine.Rigidbody::set_isKinematic(System.Boolean)")]
    pub fn set_is_kinematic(&self, value: bool) {}

    #[unity_icall("UnityEngine.Rigidbody::get_freezeRotation")]
    pub fn get_freeze_rotation(&self) -> bool {}

    #[unity_icall("UnityEngine.Rigidbody::set_freezeRotation(System.Boolean)")]
    pub fn set_freeze_rotation(&self, value: bool) {}

    #[unity_icall("UnityEngine.Rigidbody::get_constraints")]
    pub fn get_constraints(&self) -> RigidbodyConstraints {}

    #[unity_icall("UnityEngine.Rigidbody::set_constraints(RigidbodyConstraints)")]
    pub fn set_constraints(&self, value: RigidbodyConstraints) {}

    #[unity_icall("UnityEngine.Rigidbody::get_collisionDetectionMode")]
    pub fn get_collision_detection_mode(&self) -> CollisionDetectionMode {}

    #[unity_icall("UnityEngine.Rigidbody::set_collisionDetectionMode(CollisionDetectionMode)")]
    pub fn set_collision_detection_mode(&self, value: CollisionDetectionMode) {}

    #[unity_icall("UnityEngine.Rigidbody::get_centerOfMass_Injected(Vector3&)")]
    pub fn get_center_of_mass(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::set_centerOfMass_Injected(Vector3&)")]
    pub fn set_center_of_mass(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_worldCenterOfMass_Injected(Vector3&)")]
    pub fn get_world_center_of_mass(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_inertiaTensorRotation_Injected(Quaternion&)")]
    pub fn get_inertia_tensor_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Rigidbody::set_inertiaTensorRotation_Injected(Quaternion&)")]
    pub fn set_inertia_tensor_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Rigidbody::get_inertiaTensor_Injected(Vector3&)")]
    pub fn get_inertia_tensor(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::set_inertiaTensor_Injected(Vector3&)")]
    pub fn set_inertia_tensor(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_detectCollisions")]
    pub fn get_detect_collisions(&self) -> bool {}

    #[unity_icall("UnityEngine.Rigidbody::set_detectCollisions(System.Boolean)")]
    pub fn set_detect_collisions(&self, value: bool) {}

    #[unity_icall("UnityEngine.Rigidbody::get_position_Injected(Vector3&)")]
    pub fn get_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::set_position_Injected(Vector3&)")]
    pub fn set_position(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::get_rotation_Injected(Quaternion&)")]
    pub fn get_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Rigidbody::set_rotation_Injected(Quaternion&)")]
    pub fn set_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Rigidbody::get_interpolation")]
    pub fn get_interpolation(&self) -> RigidbodyInterpolation {}

    #[unity_icall("UnityEngine.Rigidbody::set_interpolation(RigidbodyInterpolation)")]
    pub fn set_interpolation(&self, value: RigidbodyInterpolation) {}

    #[unity_icall("UnityEngine.Rigidbody::get_solverIterations")]
    pub fn get_solver_iterations(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_solverIterations(System.Int32)")]
    pub fn set_solver_iterations(&self, value: i32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_sleepThreshold")]
    pub fn get_sleep_threshold(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_sleepThreshold(System.Single)")]
    pub fn set_sleep_threshold(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_maxAngularVelocity")]
    pub fn get_max_angular_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_maxAngularVelocity(System.Single)")]
    pub fn set_max_angular_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::get_solverVelocityIterations")]
    pub fn get_solver_velocity_iterations(&self) -> i32 {}

    #[unity_icall("UnityEngine.Rigidbody::set_solverVelocityIterations(System.Int32)")]
    pub fn set_solver_velocity_iterations(&self, value: i32) {}

    #[unity_icall("UnityEngine.Rigidbody::SetDensity(System.Single)")]
    pub fn set_density(&self, density: f32) {}

    #[unity_icall("UnityEngine.Rigidbody::MovePosition_Injected(Vector3&)")]
    pub fn move_position(&self, position: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::MoveRotation_Injected(Quaternion&)")]
    pub fn move_rotation(&self, rot: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Rigidbody::Sleep")]
    pub fn sleep(&self) {}

    #[unity_icall("UnityEngine.Rigidbody::IsSleeping")]
    pub fn is_sleeping(&self) -> bool {}

    #[unity_icall("UnityEngine.Rigidbody::WakeUp")]
    pub fn wake_up(&self) {}

    #[unity_icall("UnityEngine.Rigidbody::ResetCenterOfMass")]
    pub fn reset_center_of_mass(&self) {}

    #[unity_icall("UnityEngine.Rigidbody::ResetInertiaTensor")]
    pub fn reset_inertia_tensor(&self) {}

    #[unity_icall("UnityEngine.Rigidbody::AddForce_Injected(Vector3&,ForceMode)")]
    pub fn add_force(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddForce_Injected(Vector3&,ForceMode)")]
    pub fn add_force_1(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddRelativeForce_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_force(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddRelativeForce_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_force_1(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_torque(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_torque_1(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddRelativeTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_torque(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddRelativeTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_torque_1(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddRelativeTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_torque_2(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddForceAtPosition_Injected(Vector3&,Vector3&,ForceMode)")]
    pub fn add_force_at_position(&self, force: &mut Vector3, position: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddForceAtPosition_Injected(Vector3&,Vector3&,ForceMode)")]
    pub fn add_force_at_position_1(&self, force: &mut Vector3, position: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddExplosionForce_Injected(System.Single,Vector3&,System.Single,System.Single,ForceMode)")]
    pub fn add_explosion_force(&self, explosion_force: f32, explosion_position: &mut Vector3, explosion_radius: f32, upwards_modifier: f32, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddExplosionForce_Injected(System.Single,Vector3&,System.Single,System.Single,ForceMode)")]
    pub fn add_explosion_force_1(&self, explosion_force: f32, explosion_position: &mut Vector3, explosion_radius: f32, upwards_modifier: f32, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::AddExplosionForce_Injected(System.Single,Vector3&,System.Single,System.Single,ForceMode)")]
    pub fn add_explosion_force_2(&self, explosion_force: f32, explosion_position: &mut Vector3, explosion_radius: f32, upwards_modifier: f32, mode: ForceMode) {}

    #[unity_icall("UnityEngine.Rigidbody::GetRelativePointVelocity_Injected(Vector3&,Vector3&)")]
    pub fn get_relative_point_velocity(&self, relative_point: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::GetPointVelocity_Injected(Vector3&,Vector3&)")]
    pub fn get_point_velocity(&self, world_point: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Rigidbody::Internal_ClosestPointOnBounds_Injected(Vector3&,Vector3&,System.Single&)")]
    pub fn internal_closest_point_on_bounds(&self, point: &mut Vector3, out_pos: &mut Vector3, distance: &mut f32) {}

    #[unity_icall("UnityEngine.Rigidbody::SweepTest_Injected(Vector3&,System.Single,QueryTriggerInteraction,System.Boolean&,RaycastHit&)")]
    pub fn sweep_test(&self, direction: &mut Vector3, max_distance: f32, query_trigger_interaction: QueryTriggerInteraction, has_hit: &mut bool, ret: &mut RaycastHit) {}

    #[unity_icall("UnityEngine.Rigidbody::Internal_SweepTestAll_Injected(Vector3&,System.Single,QueryTriggerInteraction)")]
    pub fn internal_sweep_test_all(&self, direction: &mut Vector3, max_distance: f32, query_trigger_interaction: QueryTriggerInteraction) -> Array<RaycastHit> {}

}
