#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Quaternion, Vector3};
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Transform", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct Transform(pub *mut c_void);

#[unity_impl]
impl Transform {
    #[unity_icall("UnityEngine.Transform::get_position_Injected(Vector3&)")]
    pub fn get_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::set_position_Injected(Vector3&)")]
    pub fn set_position(&self, value: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::get_localPosition_Injected(Vector3&)")]
    pub fn get_local_position(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::set_localPosition_Injected(Vector3&)")]
    pub fn set_local_position(&self, value: &mut Vector3) {}

    #[unity_method(name = "get_eulerAngles")]
    pub fn get_euler_angles(&self) -> Vector3 {}

    #[unity_method(name = "set_eulerAngles")]
    pub fn set_euler_angles(&self, value: Vector3) {}

    #[unity_method(name = "get_localEulerAngles")]
    pub fn get_local_euler_angles(&self) -> Vector3 {}

    #[unity_method(name = "set_localEulerAngles")]
    pub fn set_local_euler_angles(&self, value: Vector3) {}

    #[unity_method(name = "get_right")]
    pub fn get_right(&self) -> Vector3 {}

    #[unity_method(name = "set_right")]
    pub fn set_right(&self, value: Vector3) {}

    #[unity_method(name = "get_up")]
    pub fn get_up(&self) -> Vector3 {}

    #[unity_method(name = "set_up")]
    pub fn set_up(&self, value: Vector3) {}

    #[unity_method(name = "get_forward")]
    pub fn get_forward(&self) -> Vector3 {}

    #[unity_method(name = "set_forward")]
    pub fn set_forward(&self, value: Vector3) {}

    #[unity_icall("UnityEngine.Transform::get_rotation_Injected(Quaternion&)")]
    pub fn get_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::set_rotation_Injected(Quaternion&)")]
    pub fn set_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::get_localRotation_Injected(Quaternion&)")]
    pub fn get_local_rotation(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::set_localRotation_Injected(Quaternion&)")]
    pub fn set_local_rotation(&self, value: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::get_localScale_Injected(Vector3&)")]
    pub fn get_local_scale(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::set_localScale_Injected(Vector3&)")]
    pub fn set_local_scale(&self, value: &mut Vector3) {}

    #[unity_method(name = "get_parent")]
    pub fn get_parent(&self) -> Option<Transform> {}

    #[unity_method(name = "set_parent")]
    pub fn set_parent(&self, value: Option<Transform>) {}

    #[unity_icall("UnityEngine.Transform::get_worldToLocalMatrix_Injected(Matrix4x4&)")]
    pub fn get_world_to_local_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Transform::get_localToWorldMatrix_Injected(Matrix4x4&)")]
    pub fn get_local_to_world_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_method(name = "get_root")]
    pub fn get_root(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Transform::get_childCount")]
    pub fn get_child_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::get_lossyScale")]
    pub fn get_lossy_scale(&self) -> Vector3 {}

    #[unity_icall("UnityEngine.Transform::get_hasChanged")]
    pub fn get_has_changed(&self) -> bool {}

    #[unity_icall("UnityEngine.Transform::set_hasChanged(System.Boolean)")]
    pub fn set_has_changed(&self, value: bool) {}

    #[unity_method(name = "get_hierarchyCapacity")]
    pub fn get_hierarchy_capacity(&self) -> i32 {}

    #[unity_method(name = "set_hierarchyCapacity")]
    pub fn set_hierarchy_capacity(&self, value: i32) {}

    #[unity_method(name = "get_hierarchyCount")]
    pub fn get_hierarchy_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::GetRotationOrderInternal")]
    pub fn get_rotation_order_internal(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::SetRotationOrderInternal(RotationOrder)")]
    pub fn set_rotation_order_internal(&self, rotation_order: *mut c_void) {}

    #[unity_icall("UnityEngine.Transform::SetPositionAndRotation_Injected(Vector3&,Quaternion&)")]
    pub fn set_position_and_rotation(&self, position: &mut Vector3, rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::SetLocalPositionAndRotation_Injected(Vector3&,Quaternion&)")]
    pub fn set_local_position_and_rotation(&self, local_position: &mut Vector3, local_rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::GetPositionAndRotation(Vector3&,Quaternion&)")]
    pub fn get_position_and_rotation(&self, position: &mut Vector3, rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::GetLocalPositionAndRotation(Vector3&,Quaternion&)")]
    pub fn get_local_position_and_rotation(&self, local_position: &mut Vector3, local_rotation: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::get_position_Injected(Vector3&)")]
    pub fn translate(&self, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::get_localRotation_Injected(Quaternion&)")]
    pub fn rotate(&self, ret: &mut Quaternion) {}

    #[unity_icall("UnityEngine.Transform::RotateAroundInternal(Vector3,System.Single)")]
    pub fn rotate_around_internal(&self, axis: Vector3, angle: f32) {}

    #[unity_icall("UnityEngine.Transform::TransformDirection_Injected(Vector3&,Vector3&)")]
    pub fn rotate_1(&self, direction: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::Internal_LookAt(Vector3,Vector3)")]
    pub fn look_at(&self, world_position: Vector3, world_up: Vector3) {}

    #[unity_icall("UnityEngine.Transform::Internal_LookAt(Vector3,Vector3)")]
    pub fn look_at_1(&self, world_position: Vector3, world_up: Vector3) {}

    #[unity_icall("UnityEngine.Transform::DetachChildren")]
    pub fn detach_children(&self) {}

    #[unity_icall("UnityEngine.Transform::SetAsFirstSibling")]
    pub fn set_as_first_sibling(&self) {}

    #[unity_icall("UnityEngine.Transform::SetAsLastSibling")]
    pub fn set_as_last_sibling(&self) {}

    #[unity_icall("UnityEngine.Transform::SetSiblingIndex(System.Int32)")]
    pub fn set_sibling_index(&self, index: i32) {}

    #[unity_icall("UnityEngine.Transform::MoveAfterSibling(Transform,System.Boolean)")]
    pub fn move_after_sibling(&self, transform: Option<Transform>, notify_editor_and_mark_dirty: bool) {}

    #[unity_icall("UnityEngine.Transform::GetSiblingIndex")]
    pub fn get_sibling_index(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::FindRelativeTransformWithPath(Transform,System.String,System.Boolean)")]
    pub fn find_relative_transform_with_path(transform: Option<Transform>, path: &str, is_active_only: bool) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Transform::SendTransformChangedScale")]
    pub fn send_transform_changed_scale(&self) {}

    #[unity_icall("UnityEngine.Transform::IsChildOf(Transform)")]
    pub fn is_child_of(&self, parent: Option<Transform>) -> bool {}

    #[unity_method(name = "GetEnumerator")]
    pub fn get_enumerator(&self) -> *mut c_void {}

    #[unity_icall("UnityEngine.Transform::RotateAround_Injected(Vector3&,System.Single)")]
    pub fn rotate_around(&self, axis: &mut Vector3, angle: f32) {}

    #[unity_icall("UnityEngine.Transform::RotateAroundLocal_Injected(Vector3&,System.Single)")]
    pub fn rotate_around_local(&self, axis: &mut Vector3, angle: f32) {}

    #[unity_icall("UnityEngine.Transform::GetChild(System.Int32)")]
    pub fn get_child(&self, index: i32) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Transform::internal_getHierarchyCapacity")]
    pub fn internal_get_hierarchy_capacity(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::internal_setHierarchyCapacity(System.Int32)")]
    pub fn internal_set_hierarchy_capacity(&self, value: i32) {}

    #[unity_icall("UnityEngine.Transform::internal_getHierarchyCount")]
    pub fn internal_get_hierarchy_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Transform::IsNonUniformScaleTransform")]
    pub fn is_non_uniform_scale_transform(&self) -> bool {}

    #[unity_icall("UnityEngine.Transform::SetConstrainProportionsScale(System.Boolean)")]
    pub fn set_constrain_proportions_scale(&self, is_linked: bool) {}

    #[unity_icall("UnityEngine.Transform::IsConstrainProportionsScale")]
    pub fn is_constrain_proportions_scale(&self) -> bool {}

    #[unity_icall("UnityEngine.Transform::GetLocalEulerAngles_Injected(RotationOrder,Vector3&)")]
    pub fn get_local_euler_angles_1(&self, order: *mut c_void, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::SetLocalEulerAngles_Injected(Vector3&,RotationOrder)")]
    pub fn set_local_euler_angles_1(&self, euler: &mut Vector3, order: *mut c_void) {}

    #[unity_icall("UnityEngine.Transform::SetLocalEulerHint_Injected(Vector3&)")]
    pub fn set_local_euler_hint(&self, euler: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::RotateAroundInternal_Injected(Vector3&,System.Single)")]
    pub fn rotate_around_internal_1(&self, axis: &mut Vector3, angle: f32) {}

    #[unity_icall("UnityEngine.Transform::Internal_LookAt_Injected(Vector3&,Vector3&)")]
    pub fn internal_look_at(&self, world_position: &mut Vector3, world_up: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::InverseTransformDirection_Injected(Vector3&,Vector3&)")]
    pub fn inverse_transform_direction(&self, direction: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::TransformVector_Injected(Vector3&,Vector3&)")]
    pub fn transform_vector(&self, vector: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::InverseTransformVector_Injected(Vector3&,Vector3&)")]
    pub fn inverse_transform_vector(&self, vector: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::TransformPoint_Injected(Vector3&,Vector3&)")]
    pub fn transform_point(&self, position: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::InverseTransformPoint_Injected(Vector3&,Vector3&)")]
    pub fn inverse_transform_point(&self, position: &mut Vector3, ret: &mut Vector3) {}

    #[unity_icall("UnityEngine.Transform::get_lossyScale_Injected(Vector3&)")]
    pub fn get_lossy_scale_1(&self, ret: &mut Vector3) {}

}
