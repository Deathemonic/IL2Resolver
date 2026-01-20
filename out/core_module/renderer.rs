#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::math::{Matrix4x4, Vector4};
use crate::mscorlib::{SystemObject, SystemString};
use crate::mscorlib::collections::{Array};
use super::bounds::Bounds;
use super::game_object::GameObject;
use super::light_probe_usage::LightProbeUsage;
use super::material::Material;
use super::material_property_block::MaterialPropertyBlock;
use super::motion_vector_generation_mode::MotionVectorGenerationMode;
use super::ray_tracing_mode::RayTracingMode;
use super::reflection_probe_usage::ReflectionProbeUsage;
use super::shadow_casting_mode::ShadowCastingMode;
use super::transform::Transform;
use crate::core_module::{Component, Object};

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Renderer", namespace = "UnityEngine", inherit = "Component,Object")]
pub struct Renderer(pub *mut c_void);

#[unity_impl]
impl Renderer {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Renderer::get_bounds")]
    pub fn get_bounds(&self) -> Bounds {}

    #[unity_icall("UnityEngine.Renderer::set_bounds_Injected(Bounds&)")]
    pub fn set_bounds(&self, value: &mut Bounds) {}

    #[unity_icall("UnityEngine.Renderer::get_localBounds")]
    pub fn get_local_bounds(&self) -> Bounds {}

    #[unity_icall("UnityEngine.Renderer::set_localBounds_Injected(Bounds&)")]
    pub fn set_local_bounds(&self, value: &mut Bounds) {}

    #[unity_icall("UnityEngine.Renderer::get_enabled")]
    pub fn get_enabled(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::set_enabled(System.Boolean)")]
    pub fn set_enabled(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::get_isVisible")]
    pub fn get_is_visible(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::get_shadowCastingMode")]
    pub fn get_shadow_casting_mode(&self) -> ShadowCastingMode {}

    #[unity_icall("UnityEngine.Renderer::set_shadowCastingMode(ShadowCastingMode)")]
    pub fn set_shadow_casting_mode(&self, value: ShadowCastingMode) {}

    #[unity_icall("UnityEngine.Renderer::get_receiveShadows")]
    pub fn get_receive_shadows(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::set_receiveShadows(System.Boolean)")]
    pub fn set_receive_shadows(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::get_forceRenderingOff")]
    pub fn get_force_rendering_off(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::set_forceRenderingOff(System.Boolean)")]
    pub fn set_force_rendering_off(&self, value: bool) {}

    #[unity_method(name = "get_staticShadowCaster")]
    pub fn get_static_shadow_caster(&self) -> bool {}

    #[unity_method(name = "set_staticShadowCaster")]
    pub fn set_static_shadow_caster(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::get_motionVectorGenerationMode")]
    pub fn get_motion_vector_generation_mode(&self) -> MotionVectorGenerationMode {}

    #[unity_icall("UnityEngine.Renderer::set_motionVectorGenerationMode(MotionVectorGenerationMode)")]
    pub fn set_motion_vector_generation_mode(&self, value: MotionVectorGenerationMode) {}

    #[unity_icall("UnityEngine.Renderer::get_lightProbeUsage")]
    pub fn get_light_probe_usage(&self) -> LightProbeUsage {}

    #[unity_icall("UnityEngine.Renderer::set_lightProbeUsage(LightProbeUsage)")]
    pub fn set_light_probe_usage(&self, value: LightProbeUsage) {}

    #[unity_icall("UnityEngine.Renderer::get_reflectionProbeUsage")]
    pub fn get_reflection_probe_usage(&self) -> ReflectionProbeUsage {}

    #[unity_icall("UnityEngine.Renderer::set_reflectionProbeUsage(ReflectionProbeUsage)")]
    pub fn set_reflection_probe_usage(&self, value: ReflectionProbeUsage) {}

    #[unity_icall("UnityEngine.Renderer::get_renderingLayerMask")]
    pub fn get_rendering_layer_mask(&self) -> u32 {}

    #[unity_icall("UnityEngine.Renderer::set_renderingLayerMask(System.UInt32)")]
    pub fn set_rendering_layer_mask(&self, value: u32) {}

    #[unity_icall("UnityEngine.Renderer::get_rendererPriority")]
    pub fn get_renderer_priority(&self) -> i32 {}

    #[unity_icall("UnityEngine.Renderer::set_rendererPriority(System.Int32)")]
    pub fn set_renderer_priority(&self, value: i32) {}

    #[unity_icall("UnityEngine.Renderer::get_rayTracingMode")]
    pub fn get_ray_tracing_mode(&self) -> RayTracingMode {}

    #[unity_icall("UnityEngine.Renderer::set_rayTracingMode(RayTracingMode)")]
    pub fn set_ray_tracing_mode(&self, value: RayTracingMode) {}

    #[unity_icall("UnityEngine.Renderer::get_sortingLayerName")]
    pub fn get_sorting_layer_name(&self) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Renderer::set_sortingLayerName(System.String)")]
    pub fn set_sorting_layer_name(&self, value: &str) {}

    #[unity_icall("UnityEngine.Renderer::get_sortingLayerID")]
    pub fn get_sorting_layer_id(&self) -> i32 {}

    #[unity_icall("UnityEngine.Renderer::set_sortingLayerID(System.Int32)")]
    pub fn set_sorting_layer_id(&self, value: i32) {}

    #[unity_icall("UnityEngine.Renderer::get_sortingOrder")]
    pub fn get_sorting_order(&self) -> i32 {}

    #[unity_icall("UnityEngine.Renderer::set_sortingOrder(System.Int32)")]
    pub fn set_sorting_order(&self, value: i32) {}

    #[unity_icall("UnityEngine.Renderer::get_allowOcclusionWhenDynamic")]
    pub fn get_allow_occlusion_when_dynamic(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::set_allowOcclusionWhenDynamic(System.Boolean)")]
    pub fn set_allow_occlusion_when_dynamic(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::get_isPartOfStaticBatch")]
    pub fn get_is_part_of_static_batch(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::get_worldToLocalMatrix_Injected(Matrix4x4&)")]
    pub fn get_world_to_local_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Renderer::get_localToWorldMatrix_Injected(Matrix4x4&)")]
    pub fn get_local_to_world_matrix(&self, ret: &mut Matrix4x4) {}

    #[unity_icall("UnityEngine.Renderer::get_lightProbeProxyVolumeOverride")]
    pub fn get_light_probe_proxy_volume_override(&self) -> Option<GameObject> {}

    #[unity_icall("UnityEngine.Renderer::set_lightProbeProxyVolumeOverride(GameObject)")]
    pub fn set_light_probe_proxy_volume_override(&self, value: Option<GameObject>) {}

    #[unity_icall("UnityEngine.Renderer::get_probeAnchor")]
    pub fn get_probe_anchor(&self) -> Option<Transform> {}

    #[unity_icall("UnityEngine.Renderer::set_probeAnchor(Transform)")]
    pub fn set_probe_anchor(&self, value: Option<Transform>) {}

    #[unity_method(name = "get_lightmapIndex")]
    pub fn get_lightmap_index(&self) -> i32 {}

    #[unity_method(name = "set_lightmapIndex")]
    pub fn set_lightmap_index(&self, value: i32) {}

    #[unity_method(name = "get_realtimeLightmapIndex")]
    pub fn get_realtime_lightmap_index(&self) -> i32 {}

    #[unity_method(name = "set_realtimeLightmapIndex")]
    pub fn set_realtime_lightmap_index(&self, value: i32) {}

    #[unity_method(name = "get_lightmapScaleOffset")]
    pub fn get_lightmap_scale_offset(&self) -> Vector4 {}

    #[unity_method(name = "set_lightmapScaleOffset")]
    pub fn set_lightmap_scale_offset(&self, value: Vector4) {}

    #[unity_method(name = "get_realtimeLightmapScaleOffset")]
    pub fn get_realtime_lightmap_scale_offset(&self) -> Vector4 {}

    #[unity_method(name = "set_realtimeLightmapScaleOffset")]
    pub fn set_realtime_lightmap_scale_offset(&self, value: Vector4) {}

    #[unity_method(name = "get_materials")]
    pub fn get_materials(&self) -> Array<Material> {}

    #[unity_method(name = "set_materials")]
    pub fn set_materials(&self, value: Array<Material>) {}

    #[unity_method(name = "get_material")]
    pub fn get_material(&self) -> Option<Material> {}

    #[unity_method(name = "set_material")]
    pub fn set_material(&self, value: Option<Material>) {}

    #[unity_method(name = "get_sharedMaterial")]
    pub fn get_shared_material(&self) -> Option<Material> {}

    #[unity_method(name = "set_sharedMaterial")]
    pub fn set_shared_material(&self, value: Option<Material>) {}

    #[unity_method(name = "get_sharedMaterials")]
    pub fn get_shared_materials(&self) -> Array<Material> {}

    #[unity_method(name = "set_sharedMaterials")]
    pub fn set_shared_materials(&self, value: Array<Material>) {}

    #[unity_method(name = "get_castShadows")]
    pub fn get_cast_shadows(&self) -> bool {}

    #[unity_method(name = "set_castShadows")]
    pub fn set_cast_shadows(&self, value: bool) {}

    #[unity_method(name = "get_motionVectors")]
    pub fn get_motion_vectors(&self) -> bool {}

    #[unity_method(name = "set_motionVectors")]
    pub fn set_motion_vectors(&self, value: bool) {}

    #[unity_method(name = "get_useLightProbes")]
    pub fn get_use_light_probes(&self) -> bool {}

    #[unity_method(name = "set_useLightProbes")]
    pub fn set_use_light_probes(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::ResetBounds")]
    pub fn reset_bounds(&self) {}

    #[unity_icall("UnityEngine.Renderer::ResetLocalBounds")]
    pub fn reset_local_bounds(&self) {}

    #[unity_icall("UnityEngine.Renderer::SetStaticLightmapST(Vector4)")]
    pub fn set_static_lightmap_st(&self, st: Vector4) {}

    #[unity_icall("UnityEngine.Renderer::GetMaterialArray")]
    pub fn get_material_array(&self) -> Array<Material> {}

    #[unity_icall("UnityEngine.Renderer::CopyMaterialArray(Material[])")]
    pub fn copy_material_array(&self, m: &mut Array<Material>) {}

    #[unity_icall("UnityEngine.Renderer::CopySharedMaterialArray(Material[])")]
    pub fn copy_shared_material_array(&self, m: &mut Array<Material>) {}

    #[unity_icall("UnityEngine.Renderer::SetMaterialArray(Material[])")]
    pub fn set_material_array(&self, m: Array<Material>) {}

    #[unity_icall("UnityEngine.Renderer::HasPropertyBlock")]
    pub fn has_property_block(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::Internal_SetPropertyBlock(MaterialPropertyBlock)")]
    pub fn set_property_block(&self, properties: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Renderer::Internal_SetPropertyBlockMaterialIndex(MaterialPropertyBlock,System.Int32)")]
    pub fn set_property_block_1(&self, properties: Option<MaterialPropertyBlock>, material_index: i32) {}

    #[unity_icall("UnityEngine.Renderer::Internal_GetPropertyBlock(MaterialPropertyBlock)")]
    pub fn get_property_block(&self, dest: Option<MaterialPropertyBlock>) {}

    #[unity_icall("UnityEngine.Renderer::Internal_GetPropertyBlockMaterialIndex(MaterialPropertyBlock,System.Int32)")]
    pub fn get_property_block_1(&self, dest: Option<MaterialPropertyBlock>, material_index: i32) {}

    #[unity_icall("UnityEngine.Renderer::GetIsStaticShadowCaster")]
    pub fn get_is_static_shadow_caster(&self) -> bool {}

    #[unity_icall("UnityEngine.Renderer::SetIsStaticShadowCaster(System.Boolean)")]
    pub fn set_is_static_shadow_caster(&self, value: bool) {}

    #[unity_icall("UnityEngine.Renderer::SetStaticBatchInfo(System.Int32,System.Int32)")]
    pub fn set_static_batch_info(&self, first_sub_mesh: i32, sub_mesh_count: i32) {}

    #[unity_icall("UnityEngine.Renderer::GetMaterialCount")]
    pub fn get_material_count(&self) -> i32 {}

    #[unity_icall("UnityEngine.Renderer::GetSharedMaterialArray")]
    pub fn get_shared_material_array(&self) -> Array<Material> {}

    #[unity_icall("UnityEngine.Renderer::GetClosestReflectionProbesInternal(System.Object)")]
    pub fn get_closest_reflection_probes(&self, result: Option<SystemObject>) {}

    #[unity_icall("UnityEngine.Renderer::get_bounds_Injected(Bounds&)")]
    pub fn get_bounds_1(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Renderer::get_localBounds_Injected(Bounds&)")]
    pub fn get_local_bounds_1(&self, ret: &mut Bounds) {}

    #[unity_icall("UnityEngine.Renderer::SetStaticLightmapST_Injected(Vector4&)")]
    pub fn set_static_lightmap_st_1(&self, st: &mut Vector4) {}

    #[unity_icall("UnityEngine.Renderer::GetLightmapST_Injected(LightmapType,Vector4&)")]
    pub fn get_lightmap_st(&self, lt: *mut c_void, ret: &mut Vector4) {}

    #[unity_icall("UnityEngine.Renderer::SetLightmapST_Injected(Vector4&,LightmapType)")]
    pub fn set_lightmap_st(&self, st: &mut Vector4, lt: *mut c_void) {}

}
