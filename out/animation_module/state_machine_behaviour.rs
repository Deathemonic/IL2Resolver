#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::animator::Animator;
use super::animator_controller_playable::AnimatorControllerPlayable;
use super::animator_state_info::AnimatorStateInfo;
use crate::core_module::{Object, ScriptableObject};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "StateMachineBehaviour", namespace = "UnityEngine", inherit = "ScriptableObject,Object")]
pub struct StateMachineBehaviour(pub *mut c_void);

#[unity_impl]
impl StateMachineBehaviour {
    #[unity_method(name = "OnStateEnter")]
    pub fn on_state_enter(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32) {}

    #[unity_method(name = "OnStateUpdate")]
    pub fn on_state_update(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32) {}

    #[unity_method(name = "OnStateExit")]
    pub fn on_state_exit(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32) {}

    #[unity_method(name = "OnStateMove")]
    pub fn on_state_move(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32) {}

    #[unity_method(name = "OnStateIK")]
    pub fn on_state_ik(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32) {}

    #[unity_method(name = "OnStateMachineEnter")]
    pub fn on_state_machine_enter(&self, animator: Option<Animator>, state_machine_path_hash: i32) {}

    #[unity_method(name = "OnStateMachineExit")]
    pub fn on_state_machine_exit(&self, animator: Option<Animator>, state_machine_path_hash: i32) {}

    #[unity_method(name = "OnStateEnter")]
    pub fn on_state_enter_1(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateUpdate")]
    pub fn on_state_update_1(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateExit")]
    pub fn on_state_exit_1(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateMove")]
    pub fn on_state_move_1(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateIK")]
    pub fn on_state_ik_1(&self, animator: Option<Animator>, state_info: AnimatorStateInfo, layer_index: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateMachineEnter")]
    pub fn on_state_machine_enter_1(&self, animator: Option<Animator>, state_machine_path_hash: i32, controller: AnimatorControllerPlayable) {}

    #[unity_method(name = "OnStateMachineExit")]
    pub fn on_state_machine_exit_1(&self, animator: Option<Animator>, state_machine_path_hash: i32, controller: AnimatorControllerPlayable) {}

}
