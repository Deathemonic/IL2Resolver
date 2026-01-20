#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemString};
use super::audio_configuration::AudioConfiguration;
use super::audio_configuration_change_handler::AudioConfigurationChangeHandler;
use super::audio_speaker_mode::AudioSpeakerMode;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioSettings", namespace = "UnityEngine")]
pub struct AudioSettings(pub *mut c_void);

#[unity_impl]
impl AudioSettings {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioSettings::get_driverCapabilities")]
    pub fn get_driver_capabilities() -> AudioSpeakerMode {}

    #[unity_method(name = "get_speakerMode", static)]
    pub fn get_speaker_mode() -> AudioSpeakerMode {}

    #[unity_method(name = "set_speakerMode", static)]
    pub fn set_speaker_mode(value: AudioSpeakerMode) {}

    #[unity_icall("UnityEngine.AudioSettings::get_dspTime")]
    pub fn get_dsp_time() -> f64 {}

    #[unity_method(name = "get_outputSampleRate", static)]
    pub fn get_output_sample_rate() -> i32 {}

    #[unity_method(name = "set_outputSampleRate", static)]
    pub fn set_output_sample_rate(value: i32) {}

    #[unity_icall("UnityEngine.AudioSettings::SetConfiguration(AudioConfiguration)")]
    pub fn set_configuration(config: AudioConfiguration) -> bool {}

    #[unity_icall("UnityEngine.AudioSettings::GetSampleRate")]
    pub fn get_sample_rate() -> i32 {}

    #[unity_icall("UnityEngine.AudioSettings::GetDSPBufferSize(System.Int32&,System.Int32&)")]
    pub fn get_dsp_buffer_size(buffer_length: &mut i32, num_buffers: &mut i32) {}

    #[unity_icall("UnityEngine.AudioSettings::GetSpatializerPluginName")]
    pub fn get_spatializer_plugin_name() -> Option<SystemString> {}

    #[unity_method(name = "add_OnAudioConfigurationChanged", static)]
    pub fn add_on_audio_configuration_changed(value: Option<AudioConfigurationChangeHandler>) {}

    #[unity_method(name = "remove_OnAudioConfigurationChanged", static)]
    pub fn remove_on_audio_configuration_changed(value: Option<AudioConfigurationChangeHandler>) {}

    #[unity_icall("UnityEngine.AudioSettings::GetAmbisonicDecoderPluginName")]
    pub fn get_ambisonic_decoder_plugin_name() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.AudioSettings::SetConfiguration_Injected(AudioConfiguration&)")]
    pub fn set_configuration_1(config: &mut AudioConfiguration) -> bool {}

    #[unity_icall("UnityEngine.AudioSettings::GetConfiguration_Injected(AudioConfiguration&)")]
    pub fn get_configuration(ret: &mut AudioConfiguration) {}

}
