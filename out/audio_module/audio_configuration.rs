#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use super::audio_speaker_mode::AudioSpeakerMode;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioConfiguration", namespace = "UnityEngine", value_type)]
pub struct AudioConfiguration {
    pub speaker_mode: AudioSpeakerMode,
    pub dsp_buffer_size: i32,
    pub sample_rate: i32,
    pub num_real_voices: i32,
    pub num_virtual_voices: i32,
}
