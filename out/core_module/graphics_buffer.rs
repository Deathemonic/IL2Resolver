#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemArray, SystemString};
use super::compute_buffer::ComputeBuffer;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "GraphicsBuffer", namespace = "UnityEngine")]
pub struct GraphicsBuffer(pub *mut c_void);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Target {
    #[default]
    Vertex = 1,
    Index = 2,
    CopySource = 4,
    CopyDestination = 8,
    Structured = 16,
    Raw = 32,
    Append = 64,
    Counter = 128,
    IndirectArguments = 256,
    Constant = 512,
}

#[unity_impl]
impl GraphicsBuffer {
    #[unity_ctor]
    pub fn new(target: Target, count: i32, stride: i32) -> Option<Self> {}

    #[unity_icall("UnityEngine.GraphicsBuffer::get_count")]
    pub fn get_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.GraphicsBuffer::get_stride")]
    pub fn get_stride(&self) -> i32 {}

    #[unity_icall("UnityEngine.GraphicsBuffer::get_target")]
    pub fn get_target(&self) -> Target {}

    #[unity_method(name = "set_name")]
    pub fn set_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::DestroyBuffer(GraphicsBuffer)")]
    pub fn dispose(buf: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::InitBuffer(GraphicsBuffer.Target,System.Int32,System.Int32)")]
    pub fn init_buffer(target: Target, count: i32, stride: i32) -> isize {}

    #[unity_icall("UnityEngine.GraphicsBuffer::DestroyBuffer(GraphicsBuffer)")]
    pub fn release(buf: Option<GraphicsBuffer>) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::IsValidBuffer(GraphicsBuffer)")]
    pub fn is_valid_buffer(buf: Option<GraphicsBuffer>) -> bool {}

    #[unity_icall("UnityEngine.GraphicsBuffer::InternalSetNativeData(System.IntPtr,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn set_data(&self, data: isize, native_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::InternalSetData(System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_set_data(&self, data: Option<SystemArray>, managed_buffer_start_index: i32, graphics_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::InternalGetData(System.Array,System.Int32,System.Int32,System.Int32,System.Int32)")]
    pub fn internal_get_data(&self, data: Option<SystemArray>, managed_buffer_start_index: i32, compute_buffer_start_index: i32, count: i32, elem_size: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::GetNativeBufferPtr")]
    pub fn get_native_buffer_ptr(&self) -> isize {}

    #[unity_icall("UnityEngine.GraphicsBuffer::SetCounterValue(System.UInt32)")]
    pub fn set_counter_value(&self, counter_value: u32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::CopyCountCC(ComputeBuffer,ComputeBuffer,System.Int32)")]
    pub fn copy_count(src: Option<ComputeBuffer>, dst: Option<ComputeBuffer>, dst_offset_bytes: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::CopyCountGC(GraphicsBuffer,ComputeBuffer,System.Int32)")]
    pub fn copy_count_1(src: Option<GraphicsBuffer>, dst: Option<ComputeBuffer>, dst_offset_bytes: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::CopyCountCG(ComputeBuffer,GraphicsBuffer,System.Int32)")]
    pub fn copy_count_2(src: Option<ComputeBuffer>, dst: Option<GraphicsBuffer>, dst_offset_bytes: i32) {}

    #[unity_icall("UnityEngine.GraphicsBuffer::CopyCountGG(GraphicsBuffer,GraphicsBuffer,System.Int32)")]
    pub fn copy_count_3(src: Option<GraphicsBuffer>, dst: Option<GraphicsBuffer>, dst_offset_bytes: i32) {}

}
