#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemArray, SystemString};
use super::compute_buffer_mode::ComputeBufferMode;
use super::compute_buffer_type::ComputeBufferType;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ComputeBuffer", namespace = "UnityEngine")]
pub struct ComputeBuffer(pub *mut c_void);

#[unity_impl]
impl ComputeBuffer {
    #[unity_ctor]
    pub fn new(count: i32, stride: i32) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_1(count: i32, stride: i32, type_ref: ComputeBufferType) -> Option<Self> {}

    #[unity_ctor]
    pub fn new_2(count: i32, stride: i32, type_ref: ComputeBufferType, usage: ComputeBufferMode) -> Option<Self> {}

    #[unity_icall("UnityEngine.ComputeBuffer::get_count")]
    pub fn get_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.ComputeBuffer::get_stride")]
    pub fn get_stride(&self) -> i32 {}

    #[unity_method(name = "set_name")]
    pub fn set_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.ComputeBuffer::DestroyBuffer(ComputeBuffer)")]
    pub fn dispose(buf: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.ComputeBuffer::InitBuffer(System.Int32,System.Int32,ComputeBufferType,ComputeBufferMode)")]
    pub fn init_buffer(count: i32, stride: i32, type_ref: ComputeBufferType, usage: ComputeBufferMode) -> isize {}

    #[unity_icall("UnityEngine.ComputeBuffer::DestroyBuffer(ComputeBuffer)")]
    pub fn release(buf: Option<ComputeBuffer>) {}

    #[unity_icall("UnityEngine.ComputeBuffer::IsValidBuffer(ComputeBuffer)")]
    pub fn is_valid_buffer(buf: Option<ComputeBuffer>) -> bool {}

    #[unity_icall("UnityEngine.ComputeBuffer::InternalSetNativeData(System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_data(&self, data: isize, native_buffer_start_index: i32, compute_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::InternalSetData(System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_set_data(&self, data: Option<SystemArray>, managed_buffer_start_index: i32, compute_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::InternalGetData(System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_get_data(&self, data: Option<SystemArray>, managed_buffer_start_index: i32, compute_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::BeginBufferWrite(System.Int32,System.Int32)")]
    pub fn begin_buffer_write(&self, offset: i32, size: i32) -> *mut () {}

    #[unity_icall("UnityEngine.ComputeBuffer::EndBufferWrite(System.Int32)")]
    pub fn end_buffer_write(&self, bytes_written: i32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::SetCounterValue(System.UInt32)")]
    pub fn set_counter_value(&self, counter_value: u32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::CopyCount(ComputeBuffer,ComputeBuffer,System.Int32)")]
    pub fn copy_count(src: Option<ComputeBuffer>, dst: Option<ComputeBuffer>, dst_offset_bytes: i32) {}

    #[unity_icall("UnityEngine.ComputeBuffer::GetNativeBufferPtr")]
    pub fn get_native_buffer_ptr(&self) -> isize {}

}
