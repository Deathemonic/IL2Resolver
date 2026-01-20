#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Recorder", namespace = "UnityEngine.Profiling")]
pub struct Recorder(pub *mut c_void);

#[unity_impl]
impl Recorder {
    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_method(name = "set_enabled")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_method(name = "get_elapsedNanoseconds")]
    pub fn get_elapsed_nanoseconds(&self) -> i64 {}

    #[unity_method(name = "get_gpuElapsedNanoseconds")]
    pub fn get_gpu_elapsed_nanoseconds(&self) -> i64 {}

    #[unity_method(name = "get_sampleBlockCount")]
    pub fn get_sample_block_count(&self) -> i32 {}

    #[unity_method(name = "get_gpuSampleBlockCount")]
    pub fn get_gpu_sample_block_count(&self) -> i32 {}

    #[unity_method(name = "Get", static)]
    pub fn get(sampler_name: &str) -> Option<Recorder> {}

    #[unity_method(name = "FilterToCurrentThread")]
    pub fn filter_to_current_thread(&self) {}

    #[unity_method(name = "CollectFromAllThreads")]
    pub fn collect_from_all_threads(&self) {}

}
