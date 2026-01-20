#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemType};
use super::playable_graph::PlayableGraph;
use super::playable_handle::PlayableHandle;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Playable", namespace = "UnityEngine.Playables", value_type)]
pub struct Playable {
    pub m_handle: PlayableHandle,
}

#[unity_impl]
impl Playable {
    #[unity_method(name = "get_Null", static)]
    pub fn get_null() -> Playable {}

    #[unity_method(name = "Create", static)]
    pub fn create(graph: PlayableGraph, input_count: i32) -> Playable {}

    #[unity_method(name = "GetHandle")]
    pub fn get_handle(&self) -> PlayableHandle {}

    #[unity_method(name = "GetPlayableType")]
    pub fn get_playable_type(&self) -> Option<SystemType> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: Playable) -> bool {}

}
