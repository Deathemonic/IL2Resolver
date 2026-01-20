#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::collections::{List};
use super::articulation_dof_lock::ArticulationDofLock;
use super::articulation_drive::ArticulationDrive;
use super::articulation_jacobian::ArticulationJacobian;
use super::articulation_joint_type::ArticulationJointType;
use super::articulation_reduced_space::ArticulationReducedSpace;
use super::collision_detection_mode::CollisionDetectionMode;
use super::force_mode::ForceMode;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.PhysicsModule", class = "ArticulationBody", namespace = "UnityEngine", inherit = "Behaviour,Component,Object")]
pub struct ArticulationBody(pub *mut c_void);

#[unity_impl]
impl ArticulationBody {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointType")]
    pub fn get_joint_type(&self) -> ArticulationJointType {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointType(ArticulationJointType)")]
    pub fn set_joint_type(&self, value: ArticulationJointType) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_anchorPosition_Injected(Vector3&)")]
    pub fn get_anchor_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_anchorPosition_Injected(Vector3&)")]
    pub fn set_anchor_position(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_parentAnchorPosition_Injected(Vector3&)")]
    pub fn get_parent_anchor_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_parentAnchorPosition_Injected(Vector3&)")]
    pub fn set_parent_anchor_position(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_anchorRotation_Injected(Quaternion&)")]
    pub fn get_anchor_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_anchorRotation_Injected(Quaternion&)")]
    pub fn set_anchor_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_parentAnchorRotation_Injected(Quaternion&)")]
    pub fn get_parent_anchor_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_parentAnchorRotation_Injected(Quaternion&)")]
    pub fn set_parent_anchor_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_isRoot")]
    pub fn get_is_root(&self) -> bool {}

    #[unity_method(name = "get_computeParentAnchor")]
    pub fn get_compute_parent_anchor(&self) -> bool {}

    #[unity_method(name = "set_computeParentAnchor")]
    pub fn set_compute_parent_anchor(&self, value: bool) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_matchAnchors")]
    pub fn get_match_anchors(&self) -> bool {}

    #[unity_icall("UnityEngine.ArticulationBody::set_matchAnchors(System.Boolean)")]
    pub fn set_match_anchors(&self, value: bool) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_linearLockX")]
    pub fn get_linear_lock_x(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_linearLockX(ArticulationDofLock)")]
    pub fn set_linear_lock_x(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_linearLockY")]
    pub fn get_linear_lock_y(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_linearLockY(ArticulationDofLock)")]
    pub fn set_linear_lock_y(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_linearLockZ")]
    pub fn get_linear_lock_z(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_linearLockZ(ArticulationDofLock)")]
    pub fn set_linear_lock_z(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_swingYLock")]
    pub fn get_swing_y_lock(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_swingYLock(ArticulationDofLock)")]
    pub fn set_swing_y_lock(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_swingZLock")]
    pub fn get_swing_z_lock(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_swingZLock(ArticulationDofLock)")]
    pub fn set_swing_z_lock(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_twistLock")]
    pub fn get_twist_lock(&self) -> ArticulationDofLock {}

    #[unity_icall("UnityEngine.ArticulationBody::set_twistLock(ArticulationDofLock)")]
    pub fn set_twist_lock(&self, value: ArticulationDofLock) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_xDrive_Injected(ArticulationDrive&)")]
    pub fn get_x_drive(&self, ret: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_xDrive_Injected(ArticulationDrive&)")]
    pub fn set_x_drive(&self, value: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_yDrive_Injected(ArticulationDrive&)")]
    pub fn get_y_drive(&self, ret: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_yDrive_Injected(ArticulationDrive&)")]
    pub fn set_y_drive(&self, value: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_zDrive_Injected(ArticulationDrive&)")]
    pub fn get_z_drive(&self, ret: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_zDrive_Injected(ArticulationDrive&)")]
    pub fn set_z_drive(&self, value: &mut ArticulationDrive) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_immovable")]
    pub fn get_immovable(&self) -> bool {}

    #[unity_icall("UnityEngine.ArticulationBody::set_immovable(System.Boolean)")]
    pub fn set_immovable(&self, value: bool) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_useGravity")]
    pub fn get_use_gravity(&self) -> bool {}

    #[unity_icall("UnityEngine.ArticulationBody::set_useGravity(System.Boolean)")]
    pub fn set_use_gravity(&self, value: bool) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_linearDamping")]
    pub fn get_linear_damping(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_linearDamping(System.Single)")]
    pub fn set_linear_damping(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_angularDamping")]
    pub fn get_angular_damping(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_angularDamping(System.Single)")]
    pub fn set_angular_damping(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointFriction")]
    pub fn get_joint_friction(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointFriction(System.Single)")]
    pub fn set_joint_friction(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_velocity_Injected(Vector3&)")]
    pub fn get_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_velocity_Injected(Vector3&)")]
    pub fn set_velocity(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_angularVelocity_Injected(Vector3&)")]
    pub fn get_angular_velocity(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_angularVelocity_Injected(Vector3&)")]
    pub fn set_angular_velocity(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_mass")]
    pub fn get_mass(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_mass(System.Single)")]
    pub fn set_mass(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_centerOfMass_Injected(Vector3&)")]
    pub fn get_center_of_mass(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_centerOfMass_Injected(Vector3&)")]
    pub fn set_center_of_mass(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_worldCenterOfMass_Injected(Vector3&)")]
    pub fn get_world_center_of_mass(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_inertiaTensor_Injected(Vector3&)")]
    pub fn get_inertia_tensor(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_inertiaTensor_Injected(Vector3&)")]
    pub fn set_inertia_tensor(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_inertiaTensorRotation_Injected(Quaternion&)")]
    pub fn get_inertia_tensor_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_inertiaTensorRotation_Injected(Quaternion&)")]
    pub fn set_inertia_tensor_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_sleepThreshold")]
    pub fn get_sleep_threshold(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_sleepThreshold(System.Single)")]
    pub fn set_sleep_threshold(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_solverIterations")]
    pub fn get_solver_iterations(&self) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_solverIterations(System.Int32)")]
    pub fn set_solver_iterations(&self, value: i32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_solverVelocityIterations")]
    pub fn get_solver_velocity_iterations(&self) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_solverVelocityIterations(System.Int32)")]
    pub fn set_solver_velocity_iterations(&self, value: i32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_maxAngularVelocity")]
    pub fn get_max_angular_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_maxAngularVelocity(System.Single)")]
    pub fn set_max_angular_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_maxLinearVelocity")]
    pub fn get_max_linear_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_maxLinearVelocity(System.Single)")]
    pub fn set_max_linear_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_maxJointVelocity")]
    pub fn get_max_joint_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_maxJointVelocity(System.Single)")]
    pub fn set_max_joint_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_maxDepenetrationVelocity")]
    pub fn get_max_depenetration_velocity(&self) -> f32 {}

    #[unity_icall("UnityEngine.ArticulationBody::set_maxDepenetrationVelocity(System.Single)")]
    pub fn set_max_depenetration_velocity(&self, value: f32) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointPosition_Injected(ArticulationReducedSpace&)")]
    pub fn get_joint_position(&self, ret: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointPosition_Injected(ArticulationReducedSpace&)")]
    pub fn set_joint_position(&self, value: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointVelocity_Injected(ArticulationReducedSpace&)")]
    pub fn get_joint_velocity(&self, ret: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointVelocity_Injected(ArticulationReducedSpace&)")]
    pub fn set_joint_velocity(&self, value: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointAcceleration_Injected(ArticulationReducedSpace&)")]
    pub fn get_joint_acceleration(&self, ret: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointAcceleration_Injected(ArticulationReducedSpace&)")]
    pub fn set_joint_acceleration(&self, value: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_jointForce_Injected(ArticulationReducedSpace&)")]
    pub fn get_joint_force(&self, ret: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::set_jointForce_Injected(ArticulationReducedSpace&)")]
    pub fn set_joint_force(&self, value: &mut ArticulationReducedSpace) {}

    #[unity_icall("UnityEngine.ArticulationBody::get_dofCount")]
    pub fn get_dof_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::get_index")]
    pub fn get_index(&self) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::get_collisionDetectionMode")]
    pub fn get_collision_detection_mode(&self) -> CollisionDetectionMode {}

    #[unity_icall("UnityEngine.ArticulationBody::set_collisionDetectionMode(CollisionDetectionMode)")]
    pub fn set_collision_detection_mode(&self, value: CollisionDetectionMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddForce_Injected(Vector3&,ForceMode)")]
    pub fn add_force(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddForce_Injected(Vector3&,ForceMode)")]
    pub fn add_force_1(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddRelativeForce_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_force(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddRelativeForce_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_force_1(&self, force: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_torque(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_torque_1(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddRelativeTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_torque(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddRelativeTorque_Injected(Vector3&,ForceMode)")]
    pub fn add_relative_torque_1(&self, torque: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddForceAtPosition_Injected(Vector3&,Vector3&,ForceMode)")]
    pub fn add_force_at_position(&self, force: &mut Vector3, position: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::AddForceAtPosition_Injected(Vector3&,Vector3&,ForceMode)")]
    pub fn add_force_at_position_1(&self, force: &mut Vector3, position: &mut Vector3, mode: ForceMode) {}

    #[unity_icall("UnityEngine.ArticulationBody::ResetCenterOfMass")]
    pub fn reset_center_of_mass(&self) {}

    #[unity_icall("UnityEngine.ArticulationBody::ResetInertiaTensor")]
    pub fn reset_inertia_tensor(&self) {}

    #[unity_icall("UnityEngine.ArticulationBody::Sleep")]
    pub fn sleep(&self) {}

    #[unity_icall("UnityEngine.ArticulationBody::IsSleeping")]
    pub fn is_sleeping(&self) -> bool {}

    #[unity_icall("UnityEngine.ArticulationBody::WakeUp")]
    pub fn wake_up(&self) {}

    #[unity_icall("UnityEngine.ArticulationBody::TeleportRoot_Injected(Vector3&,Quaternion&)")]
    pub fn teleport_root(&self, position: &mut Vector3, rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetDenseJacobian(ArticulationJacobian&)")]
    pub fn get_dense_jacobian(&self, jacobian: &mut ArticulationJacobian) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::GetJointPositions(List<System.Single>)")]
    pub fn get_joint_positions(&self, positions: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetJointPositions(List<System.Single>)")]
    pub fn set_joint_positions(&self, positions: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetJointVelocities(List<System.Single>)")]
    pub fn get_joint_velocities(&self, velocities: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetJointVelocities(List<System.Single>)")]
    pub fn set_joint_velocities(&self, velocities: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetJointAccelerations(List<System.Single>)")]
    pub fn get_joint_accelerations(&self, accelerations: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetJointAccelerations(List<System.Single>)")]
    pub fn set_joint_accelerations(&self, accelerations: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetJointForces(List<System.Single>)")]
    pub fn get_joint_forces(&self, forces: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetJointForces(List<System.Single>)")]
    pub fn set_joint_forces(&self, forces: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetDriveTargets(List<System.Single>)")]
    pub fn get_drive_targets(&self, targets: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetDriveTargets(List<System.Single>)")]
    pub fn set_drive_targets(&self, targets: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetDriveTargetVelocities(List<System.Single>)")]
    pub fn get_drive_target_velocities(&self, target_velocities: List<f32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::SetDriveTargetVelocities(List<System.Single>)")]
    pub fn set_drive_target_velocities(&self, target_velocities: List<f32>) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetDofStartIndices(List<System.Int32>)")]
    pub fn get_dof_start_indices(&self, dof_start_indices: List<i32>) -> i32 {}

    #[unity_icall("UnityEngine.ArticulationBody::GetClosestPoint_Injected(Vector3&,Vector3&)")]
    pub fn get_closest_point(&self, point: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetRelativePointVelocity_Injected(Vector3&,Vector3&)")]
    pub fn get_relative_point_velocity(&self, relative_point: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.ArticulationBody::GetPointVelocity_Injected(Vector3&,Vector3&)")]
    pub fn get_point_velocity(&self, world_point: &mut Vector3, ret: &mut Vector3) {}

}
