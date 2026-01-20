#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use super::animation_clip::AnimationClip;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AnimationModule", class = "AnimationClipPair", namespace = "UnityEngine")]
pub struct AnimationClipPair(pub *mut c_void);
