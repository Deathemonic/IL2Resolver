#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemString};
use crate::mscorlib::collections::{Array};
use super::human_bone::HumanBone;
use super::skeleton_bone::SkeletonBone;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanDescription", namespace = "UnityEngine", value_type)]
pub struct HumanDescription {
    pub human: Array<HumanBone>,
    pub skeleton: Array<SkeletonBone>,
    pub m_arm_twist: f32,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_global_scale: f32,
    pub m_root_motion_bone_name: Option<SystemString>,
    pub m_has_translation_do_f: bool,
    pub m_has_extra_root: bool,
    pub m_skeleton_has_parents: bool,
}

#[unity_impl]
impl HumanDescription {
    #[unity_method(name = "get_upperArmTwist")]
    pub fn get_upper_arm_twist(&self) -> f32 {}

    #[unity_method(name = "set_upperArmTwist")]
    pub fn set_upper_arm_twist(&self, value: f32) {}

    #[unity_method(name = "get_lowerArmTwist")]
    pub fn get_lower_arm_twist(&self) -> f32 {}

    #[unity_method(name = "set_lowerArmTwist")]
    pub fn set_lower_arm_twist(&self, value: f32) {}

    #[unity_method(name = "get_upperLegTwist")]
    pub fn get_upper_leg_twist(&self) -> f32 {}

    #[unity_method(name = "set_upperLegTwist")]
    pub fn set_upper_leg_twist(&self, value: f32) {}

    #[unity_method(name = "get_lowerLegTwist")]
    pub fn get_lower_leg_twist(&self) -> f32 {}

    #[unity_method(name = "set_lowerLegTwist")]
    pub fn set_lower_leg_twist(&self, value: f32) {}

    #[unity_method(name = "get_armStretch")]
    pub fn get_arm_stretch(&self) -> f32 {}

    #[unity_method(name = "set_armStretch")]
    pub fn set_arm_stretch(&self, value: f32) {}

    #[unity_method(name = "get_legStretch")]
    pub fn get_leg_stretch(&self) -> f32 {}

    #[unity_method(name = "set_legStretch")]
    pub fn set_leg_stretch(&self, value: f32) {}

    #[unity_method(name = "get_feetSpacing")]
    pub fn get_feet_spacing(&self) -> f32 {}

    #[unity_method(name = "set_feetSpacing")]
    pub fn set_feet_spacing(&self, value: f32) {}

    #[unity_method(name = "get_hasTranslationDoF")]
    pub fn get_has_translation_do_f(&self) -> bool {}

    #[unity_method(name = "set_hasTranslationDoF")]
    pub fn set_has_translation_do_f(&self, value: bool) {}

}
