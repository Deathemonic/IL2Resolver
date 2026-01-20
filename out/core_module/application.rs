#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_void;
use unity_derive::*;
use crate::mscorlib::{Action, SystemString};
use crate::mscorlib::collections::{Array};
use super::advertising_identifier_callback::AdvertisingIdentifierCallback;
use super::application_install_mode::ApplicationInstallMode;
use super::application_sandbox_type::ApplicationSandboxType;
use super::async_operation::AsyncOperation;
use super::log_callback::LogCallback;
use super::low_memory_callback::LowMemoryCallback;
use super::network_reachability::NetworkReachability;
use super::object::Object;
use super::runtime_platform::RuntimePlatform;
use super::stack_trace_log_type::StackTraceLogType;
use super::system_language::SystemLanguage;
use super::thread_priority::ThreadPriority;
use super::unity_action::UnityAction;
use super::user_authorization::UserAuthorization;

#[repr(transparent)]
#[derive(UnityClass)]
#[unity(assembly = "UnityEngine.CoreModule", class = "Application", namespace = "UnityEngine")]
pub struct Application(pub *mut c_void);

#[unity_impl]
impl Application {
    #[unity_ctor]
    pub fn new() -> Option<Self> {}

    #[unity_icall("UnityEngine.Application::get_isLoadingLevel")]
    pub fn get_is_loading_level() -> bool {}

    #[unity_method(name = "get_streamedBytes", static)]
    pub fn get_streamed_bytes() -> i32 {}

    #[unity_method(name = "get_webSecurityEnabled", static)]
    pub fn get_web_security_enabled() -> bool {}

    #[unity_icall("UnityEngine.Application::get_isPlaying")]
    pub fn get_is_playing() -> bool {}

    #[unity_icall("UnityEngine.Application::get_isFocused")]
    pub fn get_is_focused() -> bool {}

    #[unity_icall("UnityEngine.Application::get_buildGUID")]
    pub fn get_build_guid() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_runInBackground")]
    pub fn get_run_in_background() -> bool {}

    #[unity_icall("UnityEngine.Application::set_runInBackground(System.Boolean)")]
    pub fn set_run_in_background(value: bool) {}

    #[unity_icall("UnityEngine.Application::get_isBatchMode")]
    pub fn get_is_batch_mode() -> bool {}

    #[unity_icall("UnityEngine.Application::get_dataPath")]
    pub fn get_data_path() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_streamingAssetsPath")]
    pub fn get_streaming_assets_path() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_persistentDataPath")]
    pub fn get_persistent_data_path() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_temporaryCachePath")]
    pub fn get_temporary_cache_path() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_absoluteURL")]
    pub fn get_absolute_url() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_unityVersion")]
    pub fn get_unity_version() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_version")]
    pub fn get_version() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_installerName")]
    pub fn get_installer_name() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_identifier")]
    pub fn get_identifier() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_installMode")]
    pub fn get_install_mode() -> ApplicationInstallMode {}

    #[unity_icall("UnityEngine.Application::get_sandboxType")]
    pub fn get_sandbox_type() -> ApplicationSandboxType {}

    #[unity_icall("UnityEngine.Application::get_productName")]
    pub fn get_product_name() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_companyName")]
    pub fn get_company_name() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_cloudProjectId")]
    pub fn get_cloud_project_id() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_targetFrameRate")]
    pub fn get_target_frame_rate() -> i32 {}

    #[unity_icall("UnityEngine.Application::set_targetFrameRate(System.Int32)")]
    pub fn set_target_frame_rate(value: i32) {}

    #[unity_icall("UnityEngine.Application::get_stackTraceLogType")]
    pub fn get_stack_trace_log_type() -> StackTraceLogType {}

    #[unity_icall("UnityEngine.Application::set_stackTraceLogType(StackTraceLogType)")]
    pub fn set_stack_trace_log_type(value: StackTraceLogType) {}

    #[unity_icall("UnityEngine.Application::get_consoleLogPath")]
    pub fn get_console_log_path() -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::get_backgroundLoadingPriority")]
    pub fn get_background_loading_priority() -> ThreadPriority {}

    #[unity_icall("UnityEngine.Application::set_backgroundLoadingPriority(ThreadPriority)")]
    pub fn set_background_loading_priority(value: ThreadPriority) {}

    #[unity_icall("UnityEngine.Application::get_genuine")]
    pub fn get_genuine() -> bool {}

    #[unity_icall("UnityEngine.Application::get_genuineCheckAvailable")]
    pub fn get_genuine_check_available() -> bool {}

    #[unity_method(name = "get_isShowingSplashScreen", static)]
    pub fn get_is_showing_splash_screen() -> bool {}

    #[unity_icall("UnityEngine.Application::get_platform")]
    pub fn get_platform() -> RuntimePlatform {}

    #[unity_method(name = "get_isMobilePlatform", static)]
    pub fn get_is_mobile_platform() -> bool {}

    #[unity_method(name = "get_isConsolePlatform", static)]
    pub fn get_is_console_platform() -> bool {}

    #[unity_icall("UnityEngine.Application::get_systemLanguage")]
    pub fn get_system_language() -> SystemLanguage {}

    #[unity_icall("UnityEngine.Application::get_internetReachability")]
    pub fn get_internet_reachability() -> NetworkReachability {}

    #[unity_method(name = "get_isPlayer", static)]
    pub fn get_is_player() -> bool {}

    #[unity_method(name = "get_levelCount", static)]
    pub fn get_level_count() -> i32 {}

    #[unity_method(name = "get_loadedLevel", static)]
    pub fn get_loaded_level() -> i32 {}

    #[unity_method(name = "get_loadedLevelName", static)]
    pub fn get_loaded_level_name() -> Option<SystemString> {}

    #[unity_method(name = "get_isEditor", static)]
    pub fn get_is_editor() -> bool {}

    #[unity_icall("UnityEngine.Application::Quit(System.Int32)")]
    pub fn quit(exit_code: i32) {}

    #[unity_icall("UnityEngine.Application::CancelQuit")]
    pub fn cancel_quit() {}

    #[unity_icall("UnityEngine.Application::Unload")]
    pub fn unload() {}

    #[unity_method(name = "GetStreamProgressForLevel", static)]
    pub fn get_stream_progress_for_level(level_index: i32) -> f32 {}

    #[unity_method(name = "GetStreamProgressForLevel", static)]
    pub fn get_stream_progress_for_level_1(level_name: &str) -> f32 {}

    #[unity_method(name = "CanStreamedLevelBeLoaded", static)]
    pub fn can_streamed_level_be_loaded(level_index: i32) -> bool {}

    #[unity_icall("UnityEngine.Application::CanStreamedLevelBeLoaded(System.String)")]
    pub fn can_streamed_level_be_loaded_1(level_name: &str) -> bool {}

    #[unity_icall("UnityEngine.Application::IsPlaying(Object)")]
    pub fn is_playing(obj: Option<Object>) -> bool {}

    #[unity_icall("UnityEngine.Application::GetBuildTags")]
    pub fn get_build_tags() -> Array<SystemString> {}

    #[unity_icall("UnityEngine.Application::SetBuildTags(System.String[])")]
    pub fn set_build_tags(build_tags: Array<SystemString>) {}

    #[unity_icall("UnityEngine.Application::HasProLicense")]
    pub fn has_pro_license() -> bool {}

    #[unity_icall("UnityEngine.Application::HasARGV(System.String)")]
    pub fn has_argv(name: &str) -> bool {}

    #[unity_icall("UnityEngine.Application::GetValueForARGV(System.String)")]
    pub fn get_value_for_argv(name: &str) -> Option<SystemString> {}

    #[unity_icall("UnityEngine.Application::RequestAdvertisingIdentifierAsync(Application.AdvertisingIdentifierCallback)")]
    pub fn request_advertising_identifier_async(delegate_method: Option<AdvertisingIdentifierCallback>) -> bool {}

    #[unity_icall("UnityEngine.Application::OpenURL(System.String)")]
    pub fn open_url(url: &str) {}

    #[unity_method(name = "ForceCrash", static)]
    pub fn force_crash(mode: i32) {}

    #[unity_icall("UnityEngine.Application::RequestUserAuthorization(UserAuthorization)")]
    pub fn request_user_authorization(mode: UserAuthorization) -> Option<AsyncOperation> {}

    #[unity_icall("UnityEngine.Application::HasUserAuthorization(UserAuthorization)")]
    pub fn has_user_authorization(mode: UserAuthorization) -> bool {}

    #[unity_method(name = "add_lowMemory", static)]
    pub fn add_low_memory(value: Option<LowMemoryCallback>) {}

    #[unity_method(name = "remove_lowMemory", static)]
    pub fn remove_low_memory(value: Option<LowMemoryCallback>) {}

    #[unity_method(name = "remove_logMessageReceived", static)]
    pub fn remove_log_message_received(value: Option<LogCallback>) {}

    #[unity_method(name = "remove_logMessageReceivedThreaded", static)]
    pub fn remove_log_message_received_threaded(value: Option<LogCallback>) {}

    #[unity_icall("UnityEngine.Application::Internal_ExternalCall(System.String)")]
    pub fn external_call(script: &str) {}

    #[unity_method(name = "DontDestroyOnLoad", static)]
    pub fn dont_destroy_on_load(o: Option<Object>) {}

    #[unity_method(name = "CaptureScreenshot", static)]
    pub fn capture_screenshot(filename: &str, super_size: i32) {}

    #[unity_method(name = "CaptureScreenshot", static)]
    pub fn capture_screenshot_1(filename: &str) {}

    #[unity_method(name = "add_onBeforeRender", static)]
    pub fn add_on_before_render(value: Option<UnityAction>) {}

    #[unity_method(name = "remove_onBeforeRender", static)]
    pub fn remove_on_before_render(value: Option<UnityAction>) {}

    #[unity_method(name = "add_focusChanged", static)]
    pub fn add_focus_changed(value: *mut c_void) {}

    #[unity_method(name = "remove_focusChanged", static)]
    pub fn remove_focus_changed(value: *mut c_void) {}

    #[unity_method(name = "add_deepLinkActivated", static)]
    pub fn add_deep_link_activated(value: *mut c_void) {}

    #[unity_method(name = "remove_deepLinkActivated", static)]
    pub fn remove_deep_link_activated(value: *mut c_void) {}

    #[unity_method(name = "add_wantsToQuit", static)]
    pub fn add_wants_to_quit(value: *mut c_void) {}

    #[unity_method(name = "remove_wantsToQuit", static)]
    pub fn remove_wants_to_quit(value: *mut c_void) {}

    #[unity_method(name = "add_quitting", static)]
    pub fn add_quitting(value: Option<Action>) {}

    #[unity_method(name = "remove_quitting", static)]
    pub fn remove_quitting(value: Option<Action>) {}

    #[unity_method(name = "add_unloading", static)]
    pub fn add_unloading(value: Option<Action>) {}

    #[unity_method(name = "remove_unloading", static)]
    pub fn remove_unloading(value: Option<Action>) {}

    #[unity_icall("UnityEngine.Application::SetLogCallbackDefined(System.Boolean)")]
    pub fn register_log_callback(defined: bool) {}

    #[unity_icall("UnityEngine.Application::SetLogCallbackDefined(System.Boolean)")]
    pub fn register_log_callback_threaded(defined: bool) {}

    #[unity_method(name = "LoadLevel", static)]
    pub fn load_level(index: i32) {}

    #[unity_method(name = "LoadLevel", static)]
    pub fn load_level_1(name: &str) {}

    #[unity_method(name = "LoadLevelAdditive", static)]
    pub fn load_level_additive(index: i32) {}

    #[unity_method(name = "LoadLevelAdditive", static)]
    pub fn load_level_additive_1(name: &str) {}

    #[unity_method(name = "LoadLevelAsync", static)]
    pub fn load_level_async(index: i32) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadLevelAsync", static)]
    pub fn load_level_async_1(level_name: &str) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadLevelAdditiveAsync", static)]
    pub fn load_level_additive_async(index: i32) -> Option<AsyncOperation> {}

    #[unity_method(name = "LoadLevelAdditiveAsync", static)]
    pub fn load_level_additive_async_1(level_name: &str) -> Option<AsyncOperation> {}

    #[unity_method(name = "UnloadLevel", static)]
    pub fn unload_level(index: i32) -> bool {}

    #[unity_method(name = "UnloadLevel", static)]
    pub fn unload_level_1(scene_path: &str) -> bool {}

}
