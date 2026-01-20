#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;
use crate::mscorlib::{SystemArray, SystemObject, SystemString};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Hash128", namespace = "UnityEngine", value_type)]
pub struct Hash128 {
    pub u64_0: u64,
    pub u64_1: u64,
}

#[unity_impl]
impl Hash128 {
    #[unity_method(name = "get_isValid")]
    pub fn get_is_valid(&self) -> bool {}

    #[unity_method(name = "CompareTo")]
    pub fn compare_to(&self, rhs: Hash128) -> i32 {}

    #[unity_icall("UnityEngine.Hash128::Parse(System.String)")]
    pub fn parse(hash_string: &str) -> Hash128 {}

    #[unity_icall("UnityEngine.Hash128::Hash128ToStringImpl(Hash128)")]
    pub fn hash128to_string_impl(hash: Hash128) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Hash128::ComputeFromArray(System.Array,System.Int32,System.Int32,System.Int32,Hash128&)")]
    pub fn compute_from_array(data: Option<SystemArray>, start: i32, count: i32, elem_size: i32, hash: &mut Hash128) {}

    #[unity_method(name = "Compute", static)]
    pub fn compute(val: i32) -> Hash128 {}

    #[unity_method(name = "Compute", static)]
    pub fn compute_1(val: f32) -> Hash128 {}

    #[unity_icall("UnityEngine.Hash128::ComputeFromString(System.String,Hash128&)")]
    pub fn append(data: &str, hash: &mut Hash128) {}

    #[unity_icall("UnityEngine.Hash128::ComputeFromPtr(System.IntPtr,System.Int32,System.Int32,System.Int32,Hash128&)")]
    pub fn append_1(data: isize, start: i32, count: i32, elem_size: i32, hash: &mut Hash128) {}

    #[unity_method(name = "Append")]
    pub fn append_2(&self, val: i32) {}

    #[unity_method(name = "Append")]
    pub fn append_3(&self, val: f32) {}

    #[unity_method(name = "Equals")]
    pub fn equals(&self, obj: Option<SystemObject>) -> bool {}

    #[unity_method(name = "Equals")]
    pub fn equals_1(&self, obj: Hash128) -> bool {}

    #[unity_method(name = "GetHashCode")]
    pub fn get_hash_code(&self) -> i32 {}

    #[unity_method(name = "CompareTo")]
    pub fn compare_to_1(&self, obj: Option<SystemObject>) -> i32 {}

    #[unity_icall("UnityEngine.Hash128::Parse_Injected(System.String,Hash128&)")]
    pub fn parse_1(hash_string: &str, ret: &mut Hash128) {}

    #[unity_icall("UnityEngine.Hash128::Hash128ToStringImpl_Injected(Hash128&)")]
    pub fn hash128to_string_impl_1(hash: &mut Hash128) -> Option<SystemString> {}

}
