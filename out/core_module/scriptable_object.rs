#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{SystemType};
use crate::core_module::Object;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "ScriptableObject", namespace = "UnityEngine", inherit = "Object")]
pub struct ScriptableObject(pub *mut c_void);

#[unity_impl]
impl ScriptableObject {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.ScriptableObject::SetDirty")]
    pub fn set_dirty(&self) {}

    #[unity_icall("UnityEngine.ScriptableObject::CreateScriptableObject(ScriptableObject)")]
    pub fn create_scriptable_object(this: Option<ScriptableObject>) {}

    #[unity_icall("UnityEngine.ScriptableObject::CreateScriptableObjectInstanceFromName(System.String)")]
    pub fn create_scriptable_object_instance_from_name(class_name: &str) -> Option<ScriptableObject> {}

    #[unity_icall("UnityEngine.ScriptableObject::CreateScriptableObjectInstanceFromType(System.Type,System.Boolean)")]
    pub fn create_scriptable_object_instance_from_type(type_ref: Option<SystemType>, apply_defaults_and_reset: bool) -> Option<ScriptableObject> {}

    #[unity_icall("UnityEngine.ScriptableObject::ResetAndApplyDefaultInstances(Object)")]
    pub fn reset_and_apply_default_instances(obj: Option<Object>) {}

}
