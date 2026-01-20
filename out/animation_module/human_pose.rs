#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::math::{Quaternion, Vector3};
use crate::mscorlib::collections::{Array};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "HumanPose", namespace = "UnityEngine", value_type)]
pub struct HumanPose {
    pub body_position: Vector3,
    pub body_rotation: Quaternion,
    pub muscles: Array<f32>,
}
