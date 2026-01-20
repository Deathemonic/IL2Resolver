#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemType};
use super::playable_output_handle::PlayableOutputHandle;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "PlayableOutput", namespace = "UnityEngine.Playables", value_type)]
pub struct PlayableOutput {
    pub m_handle: PlayableOutputHandle,
}

#[unity_impl]
impl PlayableOutput {
    #[unity_method(name = "get_Null", static)]
    pub fn get_null() -> PlayableOutput {}

    #[unity_method(name = "GetHandle")]
    pub fn get_handle(&self) -> PlayableOutputHandle {}

    #[unity_method(name = "GetPlayableOutputType")]
    pub fn get_playable_output_type(&self) -> Option<SystemType> {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, other: PlayableOutput) -> bool {}

}
