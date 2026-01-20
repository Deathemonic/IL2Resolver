#![allow(non_camel_case_types)]
#![allow(dead_code)]

use unity_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "JobHandle", namespace = "Unity.Jobs", value_type)]
pub struct JobHandle {
    pub job_group: isize,
    pub version: i32,
}

#[unity_impl]
impl JobHandle {
    #[unity_method(name = "get_IsCompleted")]
    pub fn get_is_completed(&self) -> bool {}

    #[unity_icall("Unity.Jobs.JobHandle::ScheduleBatchedJobsAndCompleteAll(System.Void*,System.Int32)")]
    pub fn complete_all(jobs: *mut (), count: i32) {}

    #[unity_icall("Unity.Jobs.JobHandle::ScheduleBatchedJobs")]
    pub fn schedule_batched_jobs() {}

    #[unity_icall("Unity.Jobs.JobHandle::ScheduleBatchedJobsAndComplete(JobHandle&)")]
    pub fn schedule_batched_jobs_and_complete(job: &mut JobHandle) {}

    #[unity_icall("Unity.Jobs.JobHandle::ScheduleBatchedJobsAndIsCompleted(JobHandle&)")]
    pub fn schedule_batched_jobs_and_is_completed(job: &mut JobHandle) -> bool {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternal2(JobHandle&,JobHandle&)")]
    pub fn combine_dependencies_internal2(job0: &mut JobHandle, job1: &mut JobHandle) -> JobHandle {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternal3(JobHandle&,JobHandle&,JobHandle&)")]
    pub fn combine_dependencies_internal3(job0: &mut JobHandle, job1: &mut JobHandle, job2: &mut JobHandle) -> JobHandle {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternalPtr(System.Void*,System.Int32)")]
    pub fn combine_dependencies_internal_ptr(jobs: *mut (), count: i32) -> JobHandle {}

    #[unity_icall("Unity.Jobs.JobHandle::CheckFenceIsDependencyOrDidSyncFence(JobHandle,JobHandle)")]
    pub fn check_fence_is_dependency_or_did_sync_fence(job_handle: JobHandle, depends_on: JobHandle) -> bool {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternal2_Injected(JobHandle&,JobHandle&,JobHandle&)")]
    pub fn combine_dependencies_internal2_1(job0: &mut JobHandle, job1: &mut JobHandle, ret: &mut JobHandle) {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternal3_Injected(JobHandle&,JobHandle&,JobHandle&,JobHandle&)")]
    pub fn combine_dependencies_internal3_1(job0: &mut JobHandle, job1: &mut JobHandle, job2: &mut JobHandle, ret: &mut JobHandle) {}

    #[unity_icall("Unity.Jobs.JobHandle::CombineDependenciesInternalPtr_Injected(System.Void*,System.Int32,JobHandle&)")]
    pub fn combine_dependencies_internal_ptr_1(jobs: *mut (), count: i32, ret: &mut JobHandle) {}

    #[unity_icall("Unity.Jobs.JobHandle::CheckFenceIsDependencyOrDidSyncFence_Injected(JobHandle&,JobHandle&)")]
    pub fn check_fence_is_dependency_or_did_sync_fence_1(job_handle: &mut JobHandle, depends_on: &mut JobHandle) -> bool {}

}
