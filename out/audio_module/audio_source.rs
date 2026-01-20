#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::collections::{Array};
use super::audio_clip::AudioClip;
use super::audio_mixer_group::AudioMixerGroup;
use super::audio_rolloff_mode::AudioRolloffMode;
use super::audio_source_curve_type::AudioSourceCurveType;
use super::audio_velocity_update_mode::AudioVelocityUpdateMode;
use super::fft_window::FFTWindow;
use crate::core_module::AnimationCurve;
use crate::audio_module::AudioBehaviour;
use crate::core_module::{Behaviour, Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.AudioModule", class = "AudioSource", namespace = "UnityEngine", inherit = "AudioBehaviour,Behaviour,Component,Object")]
pub struct AudioSource(pub *mut c_void);

#[unity_impl]
impl AudioSource {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.AudioSource::get_volume")]
    pub fn get_volume(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_volume(System.Single)")]
    pub fn set_volume(&self, value: f32) {}

    #[unity_method(name = "get_pitch")]
    pub fn get_pitch(&self) -> f32 {}

    #[unity_method(name = "set_pitch")]
    pub fn set_pitch(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_time")]
    pub fn get_time(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_time(System.Single)")]
    pub fn set_time(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_timeSamples")]
    pub fn get_time_samples(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioSource::set_timeSamples(System.Int32)")]
    pub fn set_time_samples(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioSource::get_clip")]
    pub fn get_clip(&self) -> Option<AudioClip> {}

    #[unity_icall("UnityEngine.AudioSource::set_clip(AudioClip)")]
    pub fn set_clip(&self, value: Option<AudioClip>) {}

    #[unity_icall("UnityEngine.AudioSource::get_outputAudioMixerGroup")]
    pub fn get_output_audio_mixer_group(&self) -> Option<AudioMixerGroup> {}

    #[unity_icall("UnityEngine.AudioSource::set_outputAudioMixerGroup(AudioMixerGroup)")]
    pub fn set_output_audio_mixer_group(&self, value: Option<AudioMixerGroup>) {}

    #[unity_icall("UnityEngine.AudioSource::get_isPlaying")]
    pub fn get_is_playing(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::get_isVirtual")]
    pub fn get_is_virtual(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::get_loop")]
    pub fn get_loop(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_loop(System.Boolean)")]
    pub fn set_loop(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_ignoreListenerVolume")]
    pub fn get_ignore_listener_volume(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_ignoreListenerVolume(System.Boolean)")]
    pub fn set_ignore_listener_volume(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_playOnAwake")]
    pub fn get_play_on_awake(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_playOnAwake(System.Boolean)")]
    pub fn set_play_on_awake(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_ignoreListenerPause")]
    pub fn get_ignore_listener_pause(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_ignoreListenerPause(System.Boolean)")]
    pub fn set_ignore_listener_pause(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_velocityUpdateMode")]
    pub fn get_velocity_update_mode(&self) -> AudioVelocityUpdateMode {}

    #[unity_icall("UnityEngine.AudioSource::set_velocityUpdateMode(AudioVelocityUpdateMode)")]
    pub fn set_velocity_update_mode(&self, value: AudioVelocityUpdateMode) {}

    #[unity_icall("UnityEngine.AudioSource::get_panStereo")]
    pub fn get_pan_stereo(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_panStereo(System.Single)")]
    pub fn set_pan_stereo(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_spatialBlend")]
    pub fn get_spatial_blend(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_spatialBlend(System.Single)")]
    pub fn set_spatial_blend(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_spatialize")]
    pub fn get_spatialize(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_spatialize(System.Boolean)")]
    pub fn set_spatialize(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_spatializePostEffects")]
    pub fn get_spatialize_post_effects(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_spatializePostEffects(System.Boolean)")]
    pub fn set_spatialize_post_effects(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_reverbZoneMix")]
    pub fn get_reverb_zone_mix(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_reverbZoneMix(System.Single)")]
    pub fn set_reverb_zone_mix(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_bypassEffects")]
    pub fn get_bypass_effects(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_bypassEffects(System.Boolean)")]
    pub fn set_bypass_effects(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_bypassListenerEffects")]
    pub fn get_bypass_listener_effects(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_bypassListenerEffects(System.Boolean)")]
    pub fn set_bypass_listener_effects(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_bypassReverbZones")]
    pub fn get_bypass_reverb_zones(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_bypassReverbZones(System.Boolean)")]
    pub fn set_bypass_reverb_zones(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_dopplerLevel")]
    pub fn get_doppler_level(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_dopplerLevel(System.Single)")]
    pub fn set_doppler_level(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_spread")]
    pub fn get_spread(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_spread(System.Single)")]
    pub fn set_spread(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_priority")]
    pub fn get_priority(&self) -> i32 {}

    #[unity_icall("UnityEngine.AudioSource::set_priority(System.Int32)")]
    pub fn set_priority(&self, value: i32) {}

    #[unity_icall("UnityEngine.AudioSource::get_mute")]
    pub fn get_mute(&self) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::set_mute(System.Boolean)")]
    pub fn set_mute(&self, value: bool) {}

    #[unity_icall("UnityEngine.AudioSource::get_minDistance")]
    pub fn get_min_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_minDistance(System.Single)")]
    pub fn set_min_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_maxDistance")]
    pub fn get_max_distance(&self) -> f32 {}

    #[unity_icall("UnityEngine.AudioSource::set_maxDistance(System.Single)")]
    pub fn set_max_distance(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::get_rolloffMode")]
    pub fn get_rolloff_mode(&self) -> AudioRolloffMode {}

    #[unity_icall("UnityEngine.AudioSource::set_rolloffMode(AudioRolloffMode)")]
    pub fn set_rolloff_mode(&self, value: AudioRolloffMode) {}

    #[unity_method(name = "get_minVolume")]
    pub fn get_min_volume(&self) -> f32 {}

    #[unity_method(name = "set_minVolume")]
    pub fn set_min_volume(&self, value: f32) {}

    #[unity_method(name = "get_maxVolume")]
    pub fn get_max_volume(&self) -> f32 {}

    #[unity_method(name = "set_maxVolume")]
    pub fn set_max_volume(&self, value: f32) {}

    #[unity_method(name = "get_rolloffFactor")]
    pub fn get_rolloff_factor(&self) -> f32 {}

    #[unity_method(name = "set_rolloffFactor")]
    pub fn set_rolloff_factor(&self, value: f32) {}

    #[unity_icall("UnityEngine.AudioSource::Play(System.Double)")]
    pub fn play(&self, delay: f64) {}

    #[unity_icall("UnityEngine.AudioSource::GetCustomCurveHelper(AudioSource,AudioSourceCurveType)")]
    pub fn get_custom_curve_helper(source: Option<AudioSource>, type_ref: AudioSourceCurveType) -> Option<AnimationCurve> {}

    #[unity_icall("UnityEngine.AudioSource::PlayHelper(AudioSource,System.UInt64)")]
    pub fn play_1(source: Option<AudioSource>, delay: u64) {}

    #[unity_icall("UnityEngine.AudioSource::PlayHelper(AudioSource,System.UInt64)")]
    pub fn play_2(source: Option<AudioSource>, delay: u64) {}

    #[unity_icall("UnityEngine.AudioSource::PlayOneShotHelper(AudioSource,AudioClip,System.Single)")]
    pub fn play_one_shot(source: Option<AudioSource>, clip: Option<AudioClip>, volume_scale: f32) {}

    #[unity_icall("UnityEngine.AudioSource::SetScheduledStartTime(System.Double)")]
    pub fn set_scheduled_start_time(&self, time: f64) {}

    #[unity_icall("UnityEngine.AudioSource::SetScheduledEndTime(System.Double)")]
    pub fn set_scheduled_end_time(&self, time: f64) {}

    #[unity_icall("UnityEngine.AudioSource::Stop(System.Boolean)")]
    pub fn stop(&self, stop_one_shots: bool) {}

    #[unity_icall("UnityEngine.AudioSource::Pause")]
    pub fn pause(&self) {}

    #[unity_icall("UnityEngine.AudioSource::UnPause")]
    pub fn un_pause(&self) {}

    #[unity_icall("UnityEngine.AudioSource::set_clip(AudioClip)")]
    pub fn play_clip_at_point(&self, value: Option<AudioClip>) {}

    #[unity_icall("UnityEngine.AudioSource::SetCustomCurveHelper(AudioSource,AudioSourceCurveType,AnimationCurve)")]
    pub fn set_custom_curve(source: Option<AudioSource>, type_ref: AudioSourceCurveType, curve: Option<AnimationCurve>) {}

    #[unity_icall("UnityEngine.AudioSource::GetOutputDataHelper(AudioSource,System.Single[],System.Int32)")]
    pub fn get_output_data(source: Option<AudioSource>, samples: &mut Array<f32>, channel: i32) {}

    #[unity_icall("UnityEngine.AudioSource::GetSpectrumDataHelper(AudioSource,System.Single[],System.Int32,FFTWindow)")]
    pub fn get_spectrum_data(source: Option<AudioSource>, samples: &mut Array<f32>, channel: i32, window: FFTWindow) {}

    #[unity_icall("UnityEngine.AudioSource::SetSpatializerFloat(System.Int32,System.Single)")]
    pub fn set_spatializer_float(&self, index: i32, value: f32) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::GetSpatializerFloat(System.Int32,System.Single&)")]
    pub fn get_spatializer_float(&self, index: i32, value: &mut f32) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::GetAmbisonicDecoderFloat(System.Int32,System.Single&)")]
    pub fn get_ambisonic_decoder_float(&self, index: i32, value: &mut f32) -> bool {}

    #[unity_icall("UnityEngine.AudioSource::SetAmbisonicDecoderFloat(System.Int32,System.Single)")]
    pub fn set_ambisonic_decoder_float(&self, index: i32, value: f32) -> bool {}

}
